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

## Root Cause: Architectural Limitation Confirmed

**Key finding**: Extension-to-extension grammar injection is **not supported** in Zed. This is not a timing issue or missing configuration - it's a fundamental limitation.

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

### Why It Doesn't Work

The infrastructure exists, but there's a limitation preventing extension-to-extension injection:

1. **Language registry access**: Extension grammars ARE in the registry, lookup code EXISTS
2. **Built-in special case**: Built-in hidden languages (like `markdown-inline`) work fine
3. **Extension limitation**: Extension-defined grammars not accessible as injection targets
4. **Tree-sitter constraint**: Issue #484 reports injections don't work on nodes with children

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
   - Built-in `markdown-inline` works, but extension-defined grammars don't
   - Issue #484 suggests tree-sitter limitations with complex injections

3. **Present use case**:
   - Dual-grammar architectures are standard for markdown (CommonMark spec)
   - Pandoc markdown naturally splits into block/inline grammars
   - This pattern aligns with tree-sitter best practices
   - Current workaround provides only 70% coverage

4. **Request clarification**:
   - Is this an intentional limitation?
   - Is there a technical reason extension grammars can't be injection targets?
   - Would Zed team accept a PR to enable this?
   - If yes, where should the fix be implemented?

### Potential Implementation Approaches (if Zed team confirms fixable)

The fix would likely be in one of these areas:

**Option 1: Extension grammar visibility**
- Make extension-loaded grammars visible as injection targets
- Ensure hidden extension languages accessible like built-in hidden languages
- May require changes to injection resolution logic

**Option 2: Lazy grammar loading for injections**
- When injection target not found in built-ins, check extensions
- Load extension grammar on-demand if available
- Handle async loading appropriately

**Option 3: Extension manifest declaration**
- Allow extensions to explicitly declare injectable grammars
- Add `injectable = true` flag to grammar declarations in `extension.toml`
- Opt-in approach for backwards compatibility

**Unknown complexity**: Without Zed team input, unclear which approach is feasible or what constraints exist

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

## For Reference: File Locations

```
zed-industries/zed/
├── crates/language/src/
│   ├── syntax_map.rs          - Injection resolution (get_injections)
│   ├── language_registry.rs   - Language lookup and registration
│   ├── language.rs             - Language structure
│   └── markdown-inline/        - Example of built-in hidden language
│       └── config.toml
├── crates/extension/src/
│   └── extension_manifest.rs  - Extension manifest loading
└── extensions/                 - Example extensions with injections
```

## Related Documentation

- [extension-research.md](./extension-research.md) - Comprehensive research of all Zed extensions and testing performed
- [alternative-approaches.md](./alternative-approaches.md) - Other approaches considered
- [Zed Extension Languages](https://zed.dev/docs/extensions/languages#code-injections) - Official documentation
- [tree-sitter-pandoc-markdown](https://github.com/ck37/tree-sitter-pandoc-markdown) - Grammar repository
