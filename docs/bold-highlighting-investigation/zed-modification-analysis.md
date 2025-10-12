# Zed Custom Grammar Injection Analysis

Analysis of why extension-to-extension grammar injection doesn't work in Zed, and proposal for contributing a fix.

## The Problem

**What works**: Extensions can inject built-in languages
- Example: `vue` → `javascript`, `css`, `html`

**What doesn't work**: Extensions cannot inject custom grammars
- Example: `pandoc_markdown` → `pandoc_markdown_inline`

When Zed processes:
```scheme
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))
```

The injection fails because `"pandoc_markdown_inline"` is extension-defined, not built-in.

## Research Findings (2025-10-12)

### What We Discovered

**1. Built-in markdown-inline exists**
- Located at `crates/languages/src/markdown-inline/`
- Has `hidden = true` in config.toml
- Explains why `markdown-inline` injection works

**2. Extension injection infrastructure exists**
- Extensions CAN define custom `injections.scm` files
- Issue #4612 confirmed: Custom injections supported
- BUT: Only works for injecting built-in languages

**3. Extension languages ARE in the registry**
- Extension languages added to same `available_languages` list as built-ins
- Via `register_language()` method in `crates/language/src/language_registry.rs`
- No separate registry for extensions

**4. The lookup code EXISTS**
```rust
// From language_registry.rs - language_for_name_or_extension()
state
    .available_languages
    .iter()
    .rev()
    .fold(None, |best_language_match, language| { ... })
```
- Iterates through ALL available languages (built-in + extensions)
- Uses name and extension matching
- SHOULD find extension-loaded languages

## Root Cause Analysis

**Key finding**: Extension-to-extension grammar injection is **not supported** in Zed. This is confirmed through extensive testing, but the underlying technical reason is still a hypothesis requiring investigation.

### Evidence from Extension Research

**Extensive testing already performed** (see [extension-research.md](./extension-research.md)):

1. **With** `languages/pandoc_markdown_inline/` + `highlights.scm` → ❌ FAILED
2. **Without** `languages/pandoc_markdown_inline/` → ❌ FAILED
3. **With** `languages/pandoc_markdown_inline/config.toml` + `hidden = true` → ❌ FAILED

**No successful examples found** in any Zed extension:
- Vue/Svelte → inject JavaScript/CSS (built-in languages only) ✅
- Nix → inject bash/python (built-in languages only) ✅
- PHP → inject to phpdoc (possibly special-cased, unclear) ⚠️
- **No extension found** injecting custom grammar → custom grammar ❌

### The Injection Resolution Code

**Location**: `crates/language/src/syntax_map.rs`
- Method: `get_injections()` calls `language_registry.language_for_name_or_extension()`
- **Does** search through all available languages (built-in + extensions)
- **Does** iterate through extension-loaded languages
- **Should theoretically work** based on code structure

### Why It Doesn't Work: Hypotheses

The infrastructure exists, but there's a limitation preventing extension-to-extension injection. Based on code analysis and observed behavior, here are the most likely explanations (requiring investigation):

#### Theory 1: WASM Grammar Loading Boundary ⭐ (Most Likely)

**The issue:**
- Built-in grammars are compiled directly into Zed binary (native code)
- Extension grammars are loaded as WASM modules at runtime
- When injection resolution happens, it might only look for grammars in the **already-loaded** set
- Extension grammars might be registered in the language list but not yet loaded into memory as WASM modules

**Why this explains the behavior:**
```
Built-in grammar:  registered → immediately available in memory → injection works ✅
Extension grammar: registered → WASM file on disk → needs loading → injection fails ❌
```

**Evidence:**
- Extension grammars ARE in the registry (confirmed via code inspection)
- Lookup code DOES iterate through them (confirmed in language_registry.rs)
- But they still don't work as injection targets
- The distinguishing factor is native code (built-in) vs WASM (extension)

#### Theory 2: Lazy Loading / On-Demand Grammar Loading

**The issue:**
- Grammars might only be loaded when explicitly requested (e.g., when opening a file of that type)
- Injection resolution might only check **already-loaded** grammars, not triggering loads
- `pandoc_markdown` is loaded (file is open), but `pandoc_markdown_inline` never gets loaded

**The sequence:**
```
1. User opens .qmd file
2. Zed loads `pandoc_markdown` grammar (WASM)
3. Starts parsing, encounters (inline) nodes
4. Injection query says: inject "pandoc_markdown_inline"
5. Checks loaded grammars: NOT FOUND
6. Doesn't trigger loading of pandoc_markdown_inline WASM
7. Injection fails
```

#### Theory 3: Extension Sandbox / Grammar Access Restrictions

**The issue:**
- Extensions might have restricted access to other extension grammars (even within same extension)
- Built-in grammars are in a "global" namespace accessible to all
- Extension grammars are in a restricted namespace

Less likely because both grammars are defined in the same extension.

#### About Issue #484

Extensions Issue #484 reports "injections don't work on nodes with children", but this does NOT fully explain what we observe:

- ✅ `pandoc_markdown` → `markdown-inline` (built-in) works on `(inline)` nodes
- ❌ `pandoc_markdown` → `pandoc_markdown_inline` (extension) fails on same `(inline)` nodes

If Issue #484 was the root cause, BOTH should fail because they target the same node structure. The distinguishing factor is **built-in vs extension-defined**, not node structure. Issue #484 may be related or contribute to limitations, but the primary issue is that extension-to-extension grammar injection is not supported.

## Relevant Zed Issues

- **Issue #4612** - Custom Treesitter language injection queries (resolved - works for built-in languages only)
- **Discussion #14953** - Writing custom injection for specific language (examples only use built-in target languages)
- **Issue #9656** / **PR #22268** - Added standard injection.language support with backwards compatibility
- **Extensions Issue #484** - Reports injection limitations with nodes that have children (still open)
- **PR #20527** - Added HTML injections for markdown (proves built-in injections work)

## Path Forward

### Current Workaround (Implemented)
- ✅ Using built-in `markdown-inline` injection
- ✅ Provides ~70% coverage for bold/italic
- ✅ Solves primary user complaint

### Proposed Fix to Zed

**Recommended approach for Zed issue**:

1. **Present findings**:
   - Extension grammars ARE in the same registry as built-ins
   - Lookup code EXISTS and iterates through all languages
   - Infrastructure appears present, but extension-to-extension injection doesn't work
   - Extensive testing performed (with/without language configs)

2. **Document the limitation**:
   - All successful extension injections target built-in languages only
   - No examples found of extension-to-extension grammar injection
   - Built-in `markdown-inline` works, but extension-defined grammars don't on same node structure
   - Issue #484 may be related but doesn't fully explain the limitation (built-in works, extension fails on same nodes)

3. **Present technical hypothesis**:
   - Most likely cause: WASM grammar loading boundary (needs confirmation)
   - Built-in grammars: compiled into binary (immediately available)
   - Extension grammars: loaded as WASM at runtime (may not be loaded when injection resolution occurs)
   - Hypothesis: Injection resolution only checks already-loaded grammars
   - See detailed analysis in "Why It Doesn't Work: Hypotheses" section

4. **Present use case**:
   - Dual-grammar architectures are standard for markdown (CommonMark spec)
   - Pandoc markdown naturally splits into block/inline grammars
   - This pattern aligns with tree-sitter best practices
   - Current workaround provides only 70% coverage

5. **Request investigation and guidance**:
   - Can the Zed team confirm whether extension grammars are loaded when injection resolution occurs?
   - See "Proposed Implementation" section for potential solutions based on hypothesis
   - Would Zed team accept a PR to enable extension-to-extension injection?
   - Request guidance on preferred implementation approach

### Proposed Implementation

**Note:** The approaches below are based on our hypothesis that extension grammars aren't loaded when injection resolution occurs. This needs investigation/confirmation.

#### Approach 1: Trigger Grammar Loading in Injection Resolution

**Files to modify:**

**1. `crates/language/src/syntax_map.rs`** (Primary target)

Currently likely does:
```rust
fn get_injections(...) {
    let language_name = "pandoc_markdown_inline";

    // Probably only checks already-loaded grammars
    if let Some(language) = self.language_registry.language_for_name(language_name) {
        // Use grammar
    }
    // Fails silently if not found
}
```

Proposed fix:
```rust
fn get_injections(...) {
    let language_name = "pandoc_markdown_inline";

    // Check if language exists in registry (even if not loaded)
    if let Some(language_metadata) = self.language_registry.find_language_metadata(language_name) {
        // Trigger loading if it's an extension grammar (loads WASM)
        let language = self.language_registry.load_grammar_if_needed(language_metadata).await;
        // Use grammar
    }
}
```

**2. `crates/language/src/language_registry.rs`** (Secondary target)

Add new method to handle injection-specific grammar loading:
```rust
pub fn load_grammar_for_injection(&self, language_name: &str) -> Result<Arc<Grammar>> {
    // Find language in available_languages
    if let Some(language) = self.find_language(language_name) {
        // Check if grammar is already loaded
        if let Some(loaded) = self.get_loaded_grammar(language) {
            return Ok(loaded);
        }

        // Load WASM grammar if extension grammar
        if language.is_extension_grammar() {
            return self.load_extension_grammar_sync_or_async(language);
        }
    }
    Err(anyhow!("Language not found: {}", language_name))
}
```

Update existing lookup method:
```rust
pub fn language_for_name_or_extension(&self, name: &str) -> Option<Arc<Language>> {
    if let Some(language) = self.find_language(name) {
        // NEW: Ensure grammar is loaded before returning
        if language.is_extension_grammar() && !language.is_loaded() {
            let _ = self.load_grammar_for_injection(&language.name);
        }
        Some(language)
    } else {
        None
    }
}
```

#### Approach 2: Eager Loading with Injectable Flag

Alternative approach: Pre-load extension grammars marked as injectable:

```toml
# extension.toml
[grammars.pandoc_markdown_inline]
repository = "https://github.com/ck37/tree-sitter-pandoc-markdown"
commit = "..."
path = "inline"
injectable = true  # Pre-load for injection use
```

This would load the grammar during registration, making it available for injection without on-demand loading.

**Which approach to use would depend on confirming the root cause and Zed team's architecture preferences.**

## Benefits of Contributing Fix

- Enables dual-grammar architectures in extensions (like Pandoc markdown)
- Benefits all Zed extension developers
- Makes Zed extensions more powerful
- Aligns with tree-sitter best practices

## Timeline

- **Research**: ✅ Complete (extensive testing performed)
- **File Zed issue**: Ready when desired
- **Await Zed team response**: Clarify if limitation is intentional
- **Implementation**: 1-3 weeks (if Zed team confirms fixable and provides guidance)
- **PR review**: 1-2 weeks
- **Switch to full grammar**: After Zed fix released and adopted

Note: Timeline assumes Zed team confirms this is fixable and not an intentional limitation.

## For Reference: File Locations and Key Functions

### Primary Files to Modify

**`crates/language/src/syntax_map.rs`**
- Function: `get_injections()` - Processes injection queries and resolves language names
- Calls: `language_registry.language_for_name_or_extension()`
- **Hypothesis**: May only check already-loaded grammars
- **Potential fix**: Trigger grammar loading if language exists but not loaded

**`crates/language/src/language_registry.rs`**
- Function: `language_for_name_or_extension()` - Looks up languages by name
- Function: `register_language()` - Adds languages to available_languages list
- Function: `get_or_load_grammar()` - Loads grammar (including WASM for extensions)
- **Hypothesis**: Lookup finds extension languages but may not ensure grammar is loaded
- **Potential fix**: Add grammar loading check in lookup path, or new `load_grammar_for_injection()` method

### Related Files (Optional Enhancement)

If Zed team prefers explicit injectable flag:

**`crates/extension/src/extension_manifest.rs`**
- Parses `extension.toml` grammar declarations
- Could add support for `injectable = true` flag

**`crates/language/src/language.rs`**
- Language structure and configuration
- Could add injectable field to language config

### Reference Files (Examples)

```
zed-industries/zed/
├── crates/language/src/
│   ├── syntax_map.rs               - PRIMARY (injection resolution)
│   ├── language_registry.rs        - PRIMARY (grammar loading)
│   ├── language.rs                 - Structure definition
│   └── markdown-inline/            - Example built-in hidden language
│       ├── config.toml             - Has `hidden = true`
│       └── highlights.scm
├── crates/extension/src/
│   └── extension_manifest.rs       - (optional: injectable flag)
└── extensions/                      - Example extensions
    ├── vue/languages/vue/injections.scm      - Injects built-in JS/CSS
    └── svelte/languages/svelte/injections.scm - Injects built-in JS/CSS
```

### Key Code Paths

**Injection Resolution Flow:**
```
1. syntax_map.rs::get_injections()
   ↓
2. language_registry.rs::language_for_name_or_extension()
   ↓
3. Iterates through available_languages
   ↓
4. Returns Language if found
   ↓
5. ❌ FAILS if grammar not loaded into memory (extension WASM)
   ✅ WORKS if grammar is built-in (already in binary)
```

**Grammar Loading Flow:**
```
1. language_registry.rs::get_or_load_grammar()
   ↓
2. Checks if grammar already loaded
   ↓
3. If extension grammar, loads WASM module
   ↓
4. Caches loaded grammar
```

**Proposed Fix Flow:**
```
1. syntax_map.rs::get_injections()
   ↓
2. language_registry.rs::load_grammar_for_injection() (NEW)
   ↓
3. Check if language exists in registry
   ↓
4. If not loaded, call get_or_load_grammar()
   ↓
5. ✅ Extension WASM now loaded and available for injection
```

## Related Documentation

- [extension-research.md](./extension-research.md) - Comprehensive research of all Zed extensions and testing performed
- [alternative-approaches.md](./alternative-approaches.md) - Other approaches considered
- [Zed Extension Languages](https://zed.dev/docs/extensions/languages#code-injections) - Official documentation
- [tree-sitter-pandoc-markdown](https://github.com/ck37/tree-sitter-pandoc-markdown) - Grammar repository
