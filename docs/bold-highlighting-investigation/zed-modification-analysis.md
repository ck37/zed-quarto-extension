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

## Root Cause: NOT a Missing Feature

**Key insight**: The infrastructure for extension-to-extension injection already exists. This is a timing/visibility issue, not missing functionality.

### Most Likely Cause: Timing/Loading Order

**Injection resolution location**: `crates/language/src/syntax_map.rs`
- Method: `get_injections()` calls `language_registry.language_for_name_or_extension()`
- Searches through all available languages

**The problem**: Extension grammars likely aren't loaded when injection resolution runs
- Languages registered but not yet available
- Async loading race condition
- `loaded: false` flag preventing access

### Other Possible Causes

1. **Hidden language handling**: Extension languages with `hidden = true` might be treated differently than built-in hidden languages
2. **Name mismatch**: Case sensitivity or formatting differences (e.g., `"pandoc_markdown_inline"` vs `"Pandoc Markdown Inline"`)
3. **Lazy loading**: Grammars need to be triggered to load before injection

## Relevant Zed Issues

- **Issue #4612** - Custom Treesitter language injection queries (resolved - confirmed working for built-ins)
- **Discussion #14953** - Writing custom injection for specific language
- **Issue #9656** / **PR #22268** - Added standard injection.language support with backwards compatibility
- **Extensions Issue #484** - Reports injection limitations (still open)

## Path Forward

### Current Workaround (Implemented)
- ✅ Using built-in `markdown-inline` injection
- ✅ Provides ~70% coverage for bold/italic
- ✅ Solves primary user complaint

### Proposed Fix to Zed

**Recommended approach for Zed issue**:

1. **Present findings**:
   - Extension grammars ARE in the same registry as built-ins
   - Lookup code EXISTS and should work
   - Infrastructure is present, not a missing feature

2. **Propose hypothesis**:
   - Most likely a timing/loading order issue
   - Extension grammars not yet loaded when injection resolution runs
   - Possible hidden language handling difference

3. **Request guidance**:
   - Ask Zed team about extension loading order
   - Clarify when extension grammars become available for injection
   - Understand if there's intentional limitation

4. **Offer to contribute**:
   - Willing to implement fix with team guidance
   - Estimated 1-2 days to identify exact cause
   - 1-2 weeks implementation depending on root cause

### Implementation Approaches (if confirmed fixable)

**Option 1: Fix loading order**
- Ensure extension grammars loaded before injection resolution
- May need to modify extension initialization sequence

**Option 2: Lazy grammar loading**
- Check and load extension grammar on-demand during injection
- Handle async loading appropriately

**Option 3: Hidden language support**
- Allow extensions to declare `hidden = true` languages for injection
- Ensure they're accessible the same way built-in hidden languages are

## Benefits of Contributing Fix

- Enables dual-grammar architectures in extensions (like Pandoc markdown)
- Benefits all Zed extension developers
- Makes Zed extensions more powerful
- Aligns with tree-sitter best practices

## Timeline

- **Research**: ✅ Complete
- **File Zed issue**: Ready when desired
- **Investigation**: 1-2 days (identify exact root cause)
- **Implementation**: 1-2 weeks (depending on cause)
- **PR review**: 1-2 weeks
- **Switch to full grammar**: After Zed fix released

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

- [Zed Extension Languages](https://zed.dev/docs/extensions/languages#code-injections)
- [tree-sitter-pandoc-markdown](https://github.com/ck37/tree-sitter-pandoc-markdown)
- [alternative-approaches.md](./alternative-approaches.md) - Other approaches considered
