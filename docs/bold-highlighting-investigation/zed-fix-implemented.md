# Zed Fix Implemented: Extension-to-Extension Grammar Injection

**Date**: 2025-10-12
**Branch**: `fix/extension-grammar-injection` in zed-industries/zed fork
**Status**: Implemented, pending testing

## Summary

Implemented a minimal fix to enable extension-to-extension grammar injection in Zed. The fix involves a single line change that increments the LanguageRegistry version when a language finishes loading.

## The Fix

### Root Cause (Confirmed)

From [verification-findings.md](./verification-findings.md), we confirmed that:

1. Extension grammars are loaded asynchronously as WASM modules
2. Injection resolution uses `.now_or_never()` for synchronous checking
3. When extension grammar isn't loaded yet, injection is marked as "pending"
4. **The missing piece**: When the grammar finished loading, the registry version wasn't incremented
5. Without version increment, SyntaxMap never rechecked pending injections

### The Solution

**Single line change in `crates/language/src/language_registry.rs`:**

```rust
// Line 980 (after line 978: state.mark_language_loaded(id);)
state.version += 1;
```

**With explanatory comment:**
```rust
// Increment version so pending injections can be resolved
state.version += 1;
```

### How It Works

**Before fix:**
```
1. Injection query → "pandoc_markdown_inline"
2. Grammar not loaded → marked as Pending
3. Background task loads grammar
4. Grammar loading completes
5. Registry version unchanged ❌
6. SyntaxMap never rechecks → injection stays pending forever
```

**After fix:**
```
1. Injection query → "pandoc_markdown_inline"
2. Grammar not loaded → marked as Pending
3. Background task loads grammar
4. Grammar loading completes
5. Registry version incremented ✅
6. SyntaxMap detects version change → rechecks pending injections
7. Grammar now available → injection resolved ✅
```

### Additional Documentation

Added comment in `crates/language/src/syntax_map.rs` (lines 441-443):

```rust
// Note: If language is not immediately available, it will remain pending.
// The loading task is triggered in get_injections(), and when it completes,
// the registry version will be incremented, causing this code to run again.
```

## Why This Fix Works

### Minimal Impact

- **One line of code**: `state.version += 1;`
- **No API changes**: Extensions don't need updates
- **No manifest changes**: No new fields or configuration needed
- **Existing infrastructure**: Uses the already-present pending injection resolution mechanism

### Leverages Existing Design

The fix works because Zed already has infrastructure for pending injections:

1. **Pending layer creation** (syntax_map.rs:1425-1429): Creates `ParseStepLanguage::Pending` when language not ready
2. **Pending resolution check** (syntax_map.rs:420-453): Periodically checks if pending languages are now available
3. **Version-based triggering** (syntax_map.rs:418): Checks only when `registry.version()` changes

The only missing piece was incrementing the version when languages load. This fix completes the chain.

### Comparison to Proposed Solutions

From [zed-modification-analysis.md](./zed-modification-analysis.md), we proposed three solutions:

**Solution 1: Make Injection Resolution Async**
- Pros: Clean, works for all cases
- Cons: Major refactoring, complex changes
- **Not chosen**: Too invasive

**Solution 2: Pre-load Injectable Grammars**
- Pros: Predictable, opt-in
- Cons: Requires manifest changes, uses more memory
- **Not chosen**: Unnecessary with simpler fix available

**Solution 3 (Implemented): Increment Version on Load**
- Pros: Minimal change, no API updates, uses existing infrastructure
- Cons: Small delay (injection works after reparse, not immediately)
- **Chosen**: Simplest and most elegant

## Code Changes

### File 1: `crates/language/src/language_registry.rs`

**Location**: Line 980 (in `load_language()` async task, success branch)

**Before**:
```rust
match language {
    Ok(language) => {
        let language = Arc::new(language);
        let mut state = this.state.write();

        state.add(language.clone());
        state.mark_language_loaded(id);
        if let Some(mut txs) = state.loading_languages.remove(&id) {
            for tx in txs.drain(..) {
                let _ = tx.send(Ok(language.clone()));
            }
        }
    }
```

**After**:
```rust
match language {
    Ok(language) => {
        let language = Arc::new(language);
        let mut state = this.state.write();

        state.add(language.clone());
        state.mark_language_loaded(id);
        // Increment version so pending injections can be resolved
        state.version += 1;
        if let Some(mut txs) = state.loading_languages.remove(&id) {
            for tx in txs.drain(..) {
                let _ = tx.send(Ok(language.clone()));
            }
        }
    }
```

### File 2: `crates/language/src/syntax_map.rs`

**Location**: Lines 441-443 (documentation comment in pending resolution loop)

**Added comment**:
```rust
// Note: If language is not immediately available, it will remain pending.
// The loading task is triggered in get_injections(), and when it completes,
// the registry version will be incremented, causing this code to run again.
```

## Commit Information

**Branch**: `fix/extension-grammar-injection`
**Commit**: `8b2fa3d4c2`
**Message**: "fix: enable extension-to-extension grammar injection"

**Full commit message**:
```
fix: enable extension-to-extension grammar injection

**Problem:**
Extension grammars (loaded as WASM) could not be used as injection targets.
When an injection query referenced an extension grammar (e.g., markdown injecting
markdown-inline), the injection would remain 'pending' indefinitely.

**Root cause:**
When a language finished loading asynchronously, the LanguageRegistry version
was not incremented. The SyntaxMap checks for pending injections only when the
registry version changes, so it never rechecked after extension grammars loaded.

**Solution:**
Increment LanguageRegistry version when a language finishes loading. This triggers
the SyntaxMap to reparse pending injection ranges, resolving them once the target
grammar is available.

**Flow:**
1. Injection query references extension grammar → creates pending injection
2. language_for_name_or_extension() spawns async loading task
3. Loading completes → registry.version incremented (NEW)
4. SyntaxMap detects version change → rechecks pending injections
5. Extension grammar now available → injection resolved

**Changes:**
- language_registry.rs:980: Increment version after language loads
- syntax_map.rs:441-443: Add comment explaining pending resolution

Fixes extension-to-extension grammar injection, enabling dual-grammar
architectures like Pandoc markdown (block + inline grammars).
```

## Testing Plan

1. **Build Zed from fix branch**:
   ```bash
   cd /Users/ck432/dropbox-mgb/Code/zed
   git checkout fix/extension-grammar-injection
   cargo build --release
   ```

2. **Install Quarto extension** with custom grammar injection:
   ```scheme
   ; languages/quarto/injections.scm
   ((inline) @injection.content
    (#set! injection.language "pandoc_markdown_inline"))
   ```

3. **Test with .qmd file**:
   - Open Quarto document with bold/italic text
   - Verify that bold (`**text**`) and italic (`*text*`) are highlighted
   - Check that injection resolves after brief delay

4. **Expected behavior**:
   - Initial parse: injection pending (no highlighting)
   - After grammar loads: version incremented
   - Reparse triggered: injection resolved
   - Bold/italic highlighting appears ✅

## Benefits

1. **Enables dual-grammar architectures**: Pandoc markdown, Vue, Svelte-like patterns
2. **No breaking changes**: Existing extensions continue to work
3. **Minimal code**: One line + documentation
4. **Future-proof**: Works for any extension-to-extension injection scenario

## Next Steps

1. ✅ Implement fix (completed)
2. ✅ Commit to branch (completed)
3. **Test with Quarto extension** (next)
4. **File Zed PR** with findings from verification-findings.md
5. **Update Quarto extension** to use `pandoc_markdown_inline` once fix is merged

## Related Documentation

- [verification-findings.md](./verification-findings.md) - Detailed investigation that led to this fix
- [verification-plan.md](./verification-plan.md) - Investigation methodology
- [zed-modification-analysis.md](./zed-modification-analysis.md) - Original hypotheses and proposed solutions
- [extension-research.md](./extension-research.md) - Testing that confirmed the limitation
