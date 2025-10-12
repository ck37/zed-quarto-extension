# Verification Findings: Extension-to-Extension Grammar Injection

**Date**: 2025-10-12
**Investigation**: Phase 1 (Code Analysis) of [verification-plan.md](./verification-plan.md)

## Executive Summary

**Root cause confirmed**: Extension-to-extension grammar injection fails due to the combination of:
1. Asynchronous WASM grammar loading for extension grammars
2. Synchronous injection resolution using `.now_or_never()`

The hypothesis from [zed-modification-analysis.md](./zed-modification-analysis.md) is **CONFIRMED**.

## Detailed Findings

### 1. Injection Resolution Code (`crates/language/src/syntax_map.rs`)

**Function**: `get_injections()` at line 1315

**Key code** (lines 1393-1396):
```rust
let language = language_registry
    .language_for_name_or_extension(&language_name)
    .now_or_never()  // <-- THE PROBLEM
    .and_then(|language| language.ok());
```

**Behavior:**
- Calls `language_for_name_or_extension()` which returns a `Future`
- Uses `.now_or_never()` to attempt immediate resolution
- `.now_or_never()` returns:
  - `Some(result)` if the Future completes immediately
  - `None` if the Future is still pending (requires async await)

**Result:**
- If `language` is `Some`: Creates `ParseStepLanguage::Loaded` (line 1408)
- If `language` is `None`: Creates `ParseStepLanguage::Pending` (lines 1415-1423)

### 2. Language Lookup (`crates/language/src/language_registry.rs`)

**Function**: `language_for_name_or_extension()` at line 685

**Key code** (lines 685-710):
```rust
pub fn language_for_name_or_extension(
    self: &Arc<Self>,
    string: &str,
) -> impl Future<Output = Result<Arc<Language>>> {
    let string = UniCase::new(string);
    let rx = self.get_or_load_language(|name, config, current_best_match| {
        // ... matching logic ...
    });
    async move { rx.await? }
}
```

**Behavior:**
- Returns a `Future` that must be awaited
- Calls `get_or_load_language()` which returns a `oneshot::Receiver`
- The future resolves when language is loaded

### 3. Language Loading (`crates/language/src/language_registry.rs`)

**Function**: `load_language()` at line 922

**Key code** (lines 930-936):
```rust
// If the language is already loaded, resolve with it immediately.
for loaded_language in state.languages.iter() {
    if loaded_language.id == language.id {
        tx.send(Ok(loaded_language.clone())).unwrap();
        return rx;
    }
}
```

**Behavior:**
- Checks if language is already loaded in memory
- If yes: Returns immediately (resolves Future synchronously)
- If no: Spawns async task to load language (lines 944-999)

**Critical**: Line 956 shows grammar loading happens during language loading:
```rust
let grammar = Some(this.get_or_load_grammar(grammar).await?);
```

### 4. Grammar Loading (`crates/language/src/language_registry.rs`)

**Function**: `get_or_load_grammar()` at line 1026

**Key code** (lines 1033-1080):
```rust
if let Some(grammar) = state.grammars.get_mut(name.as_ref()) {
    match grammar {
        AvailableGrammar::Native(grammar) | AvailableGrammar::Loaded(_, grammar) => {
            tx.send(Ok(grammar.clone())).ok();  // IMMEDIATE
        }
        AvailableGrammar::Unloaded(wasm_path) => {
            // ... spawn async task to load WASM ...  // ASYNC
            self.executor
                .spawn(async move {
                    let wasm_bytes = std::fs::read(&wasm_path)?;
                    // ... load grammar from WASM bytes ...
                })
                .detach();
        }
    }
}
```

**Grammar states:**
- `AvailableGrammar::Native` - Built-in grammars (compiled into Zed binary)
  - **Available immediately** ✅
- `AvailableGrammar::Loaded` - Already loaded into memory
  - **Available immediately** ✅
- `AvailableGrammar::Unloaded` - WASM file path, not yet loaded
  - **Requires async loading** ❌ → `.now_or_never()` returns `None`
- `AvailableGrammar::Loading` - Currently being loaded
  - **Still async** ❌ → `.now_or_never()` returns `None`

### 5. Grammar Registration

**Built-in grammars** (lines 540-548):
```rust
pub fn register_native_grammars(
    &self,
    grammars: impl IntoIterator<Item = (impl Into<Arc<str>>, impl Into<tree_sitter::Language>)>,
) {
    self.state.write().grammars.extend(
        grammars
            .into_iter()
            .map(|(name, grammar)| (name.into(), AvailableGrammar::Native(grammar.into()))),
    );
}
```

**Extension grammars** (lines 552-561):
```rust
pub fn register_wasm_grammars(
    &self,
    grammars: impl IntoIterator<Item = (impl Into<Arc<str>>, PathBuf)>,
) {
    let mut state = self.state.write();
    state.grammars.extend(
        grammars
            .into_iter()
            .map(|(name, path)| (name.into(), AvailableGrammar::Unloaded(path))),
    );
}
```

**Key difference:**
- Built-in: Registered as `Native` → immediately available
- Extension: Registered as `Unloaded` → requires async loading

## Complete Flow Diagram

### Built-in Grammar Injection (Works ✅)

```
1. User opens .qmd file
2. Zed parses with pandoc_markdown grammar
3. Encounters (inline) node, triggers injection
4. syntax_map.rs::get_injections() called
5. Looks up "markdown-inline"
   ↓
6. language_registry.language_for_name_or_extension("markdown-inline")
   ↓
7. load_language() checks if loaded
   → Already loaded OR grammar is Native
   ↓
8. Returns immediately (Future resolves synchronously)
   ↓
9. .now_or_never() returns Some(language) ✅
   ↓
10. Creates ParseStepLanguage::Loaded
11. Injection succeeds ✅
```

### Extension Grammar Injection (Fails ❌)

```
1. User opens .qmd file
2. Zed parses with pandoc_markdown grammar
3. Encounters (inline) node, triggers injection
4. syntax_map.rs::get_injections() called
5. Looks up "pandoc_markdown_inline"
   ↓
6. language_registry.language_for_name_or_extension("pandoc_markdown_inline")
   ↓
7. load_language() checks if loaded
   → NOT loaded yet
   ↓
8. Spawns async task to load language
   ↓
9. Language loading calls get_or_load_grammar()
   ↓
10. Grammar is AvailableGrammar::Unloaded (WASM file)
    ↓
11. Spawns async task to load WASM
    ↓
12. Returns Future (still pending, requires await)
    ↓
13. .now_or_never() returns None ❌
    ↓
14. Creates ParseStepLanguage::Pending with just name
15. Injection fails ❌
```

## Why Built-in Works but Extension Doesn't

| Aspect | Built-in Grammar | Extension Grammar |
|--------|------------------|-------------------|
| Registration | `AvailableGrammar::Native` | `AvailableGrammar::Unloaded` |
| Storage | Compiled into Zed binary | WASM file on disk |
| Availability | Immediately available in memory | Requires file read + WASM loading |
| Loading | Synchronous (already loaded) | Asynchronous (needs async/await) |
| `.now_or_never()` | Returns `Some(language)` ✅ | Returns `None` ❌ |
| Injection result | Works ✅ | Fails ❌ |

## Answer to Key Questions

From [verification-plan.md](./verification-plan.md):

1. **Does `language_for_name_or_extension()` call `get_or_load_grammar()`?**
   - Not directly. It calls `load_language()`, which then calls `get_or_load_grammar()`.

2. **Does `get_injections()` trigger any grammar loading?**
   - Yes, but only asynchronously. The `.now_or_never()` call doesn't wait for completion.

3. **Is there a separate code path for built-in vs extension grammars?**
   - No separate code path, but different `AvailableGrammar` enum variants behave differently:
     - `Native` → immediate resolution
     - `Unloaded` → async resolution

4. **Are extension grammars loaded eagerly or lazily?**
   - **Lazily**. Grammars are registered as `Unloaded` and only loaded when first requested.

5. **Is there a check for "grammar loaded in memory" before injection?**
   - Yes, in `load_language()` lines 930-936. But if not loaded, it spawns async task.

6. **How do built-in languages differ in their registration/availability?**
   - Built-ins are registered as `AvailableGrammar::Native` (already in memory)
   - Extensions are registered as `AvailableGrammar::Unloaded` (WASM file path)

## The Root Cause

**The problem is architectural:**

The injection resolution code in `syntax_map.rs` uses synchronous resolution (`.now_or_never()`) on an asynchronous loading system (`Future`). This works for built-in grammars (immediately available) but fails for extension grammars (require async WASM loading).

**Why `.now_or_never()`?**

Injection resolution likely needs to be synchronous because it happens during parsing, which is performance-critical and may not be able to pause for async operations. Using `.now_or_never()` is a way to say "give me the result if it's ready, otherwise skip it."

**Why extension grammars aren't pre-loaded:**

Extension grammars are registered as `Unloaded` to avoid loading all WASM modules upfront (memory/performance optimization). They're loaded on-demand when a file of that type is opened.

**The mismatch:**

- Injection resolution expects immediate availability
- Extension grammars provide lazy loading
- No mechanism exists to pre-load "injectable" grammars

## Hypothesis Status

**CONFIRMED ✅**

Our hypothesis from [zed-modification-analysis.md](./zed-modification-analysis.md) was correct:

> **Theory 1: WASM Grammar Loading Boundary** ⭐ (Most Likely)
>
> Built-in grammars are compiled directly into Zed binary (native code)
> Extension grammars are loaded as WASM modules at runtime
> When injection resolution happens, it might only look for grammars in the **already-loaded** set

This is exactly what the code shows.

## Proposed Solutions

Based on the verified root cause, here are the approaches to fix it:

### Solution 1: Make Injection Resolution Async

**Modify**: `crates/language/src/syntax_map.rs`

**Change**: Remove `.now_or_never()` and make injection resolution async-aware

**Pros**: Clean solution, works for all cases
**Cons**: May require significant refactoring of parsing system

### Solution 2: Pre-load Injectable Grammars

**Modify**: `crates/language/src/language_registry.rs`

**Change**: Add mechanism to eagerly load grammars marked as injectable

**Example**:
```rust
// When registering extension grammar with injectable flag
pub fn register_wasm_grammars_with_options(
    &self,
    grammars: impl IntoIterator<Item = (impl Into<Arc<str>>, PathBuf, GrammarOptions)>,
) {
    for (name, path, options) in grammars {
        let grammar = AvailableGrammar::Unloaded(path.clone());
        self.state.write().grammars.insert(name.into(), grammar);

        // If marked as injectable, pre-load immediately
        if options.injectable {
            self.executor.spawn(self.clone().get_or_load_grammar(name.into())).detach();
        }
    }
}
```

**Pros**: Minimal changes to injection resolution
**Cons**: Requires extension manifest changes, uses more memory

### Solution 3: Check and Trigger Loading in Injection Resolution

**Modify**: `crates/language/src/syntax_map.rs`

**Change**: When language lookup returns None, check if grammar exists and trigger loading for next parse

**Pros**: Graceful degradation, works eventually
**Cons**: Injection won't work on first parse, only after reload

## Recommendation

**Solution 2** (Pre-load injectable grammars) is the most practical:

1. **Least invasive**: Doesn't require async refactoring of parsing system
2. **Opt-in**: Extensions explicitly mark grammars as injectable
3. **Predictable**: Grammars are loaded at extension load time
4. **Memory overhead is minimal**: Most injection targets are small inline grammars

Implementation would involve:
1. Add `injectable` field to grammar declarations in `extension.toml`
2. Modify `register_wasm_grammars()` to accept options
3. Pre-load grammars marked as injectable during registration

## Next Steps

1. Update [zed-modification-analysis.md](./zed-modification-analysis.md) to reflect confirmed findings
2. File Zed issue with:
   - Confirmed root cause (not hypothesis)
   - Code references (line numbers from this document)
   - Proposed solution with implementation details
3. Optionally: Prepare PR implementing Solution 2

## Files Referenced

All code references from:
- **Zed commit**: main branch as of 2025-10-12
- **Repository**: https://github.com/zed-industries/zed

### Key File Locations

- `crates/language/src/syntax_map.rs`
  - Line 1315: `get_injections()` function
  - Line 1393-1396: `.now_or_never()` call (root cause)

- `crates/language/src/language_registry.rs`
  - Line 685: `language_for_name_or_extension()`
  - Line 922: `load_language()`
  - Line 1026: `get_or_load_grammar()`
  - Line 540: `register_native_grammars()`
  - Line 552: `register_wasm_grammars()`

## Related Documentation

- [verification-plan.md](./verification-plan.md) - Investigation plan that led to these findings
- [zed-modification-analysis.md](./zed-modification-analysis.md) - Original hypothesis (now confirmed)
- [extension-research.md](./extension-research.md) - Testing that confirmed limitation exists
- [alternative-approaches.md](./alternative-approaches.md) - Workarounds considered
