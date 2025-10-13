# Tree-sitter Scope Naming Decision

## TL;DR

This extension uses **Zed's legacy scope names** (`@text.*`, `@emphasis.*`) instead of the newer **nvim-treesitter conventions** (`@markup.*`) because Zed's themes don't yet support the modern scopes.

## Background: Scope Naming Standards

### 1. nvim-treesitter Conventions (Modern Standard)
- **Examples**: `@markup.heading`, `@markup.italic`, `@markup.bold`, `@markup.raw.block`
- **Developed by**: nvim-treesitter project (Neovim's tree-sitter integration)
- **Used by**: Neovim, Helix editor
- **Status**: Emerging as the de facto standard for tree-sitter-based editors
- **Advantages**:
  - Well-documented
  - Semantic and consistently structured
  - Most new tree-sitter grammars use these conventions
  - Multiple editors are adopting them

### 2. Legacy Tree-sitter Scopes (Zed's Current Standard)
- **Examples**: `@text.title`, `@text.emphasis`, `@emphasis.strong`, `@text.literal`
- **Status**: Earlier tree-sitter naming conventions
- **Used by**: Zed (as of October 2025)
- **Characteristics**:
  - Less consistent structure
  - Evolved organically over time
  - Still supported by Zed's built-in themes

### 3. TextMate Scopes (Original Inspiration)
- **Format**: `markup.heading.markdown`, `markup.bold.markdown`
- **Used by**: VSCode, Sublime Text (with regex grammars)
- **Relationship**: Original scope naming that inspired tree-sitter

## Our Decision: Use Zed's Legacy Scopes

### Why We Chose Legacy Scopes

1. **Theme Compatibility**: Zed's built-in themes currently only recognize legacy scope names
   - If we use `@markup.italic`, text won't be highlighted because themes expect `@text.emphasis`
   - Users would see no syntax highlighting

2. **Practical Reality**: Zed hasn't migrated to nvim-treesitter conventions yet
   - No timeline for when this migration will happen
   - We need to work with what Zed supports today

3. **User Experience**: Working syntax highlighting is better than technically correct but invisible highlighting

### Implementation Strategy

We handle the scope mismatch at two levels:

#### 1. Block Grammar (`languages/quarto/highlights.scm`)
- Manually maintained with Zed-compatible scopes
- Directly controls highlighting for block-level elements (headings, code blocks, etc.)

#### 2. Inline Grammar (`languages/pandoc_markdown_inline/highlights.scm`)
- The upstream `tree-sitter-pandoc-markdown-inline` grammar uses modern `@markup.*` scopes
- We provide an **override file** in the extension that uses Zed-compatible scopes
- When Zed loads the inline grammar, it uses our override instead of the upstream version
- Contains mappings like:
  - `(emphasis) @text.emphasis` instead of `(emphasis) @markup.italic`
  - `(strong_emphasis) @emphasis.strong` instead of `(strong_emphasis) @markup.bold`
  - etc.

#### 3. Build-time Patching (for Tests Only)
- Our `build.rs` also patches the inline grammar during native test compilation
- This ensures tests work with Zed-compatible scopes
- Not used by Zed at runtime (Zed uses the override file instead)

## Future Migration Path

### When Zed Adopts nvim-treesitter Scopes

Eventually, Zed will likely migrate to nvim-treesitter scope conventions. When that happens:

1. **Update `languages/quarto/highlights.scm`**:
   - Replace `@text.*` with `@markup.*` scopes
   - Replace `@emphasis.strong` with `@markup.bold`
   - Follow the nvim-treesitter conventions document

2. **Delete `languages/pandoc_markdown_inline/highlights.scm`**:
   - Remove the override file entirely
   - Let Zed use the upstream grammar's native `@markup.*` scopes

3. **Remove patching from `build.rs`** (optional cleanup):
   - Delete the scope replacement code (lines 87-120)
   - Or keep it if you want tests to continue working during the transition

4. **Update tests**:
   - Update scope names in `tests/emphasis_highlighting.rs`
   - Update scope names in `tests/highlights.rs`

4. **Verify**:
   - Test that highlighting still works with updated Zed themes
   - Update this document to reflect the new state

### Signs That Zed Has Migrated

Watch for:
- Zed release notes mentioning "nvim-treesitter scope support" or "modern tree-sitter scopes"
- Built-in Zed themes using `@markup.*` scopes
- Zed documentation referencing nvim-treesitter conventions
- Other Zed extensions switching to modern scopes

### Current Zed Status (as of October 2025)

**Official Documentation**: Zed's [Language Extensions documentation](https://zed.dev/docs/extensions/languages) lists the supported scopes - notably `@text.literal`, `@emphasis.strong`, etc. The `@markup.*` scopes are **not documented or supported**.

**Community Discussion**: There's an active discussion ([#23371](https://github.com/zed-industries/zed/discussions/23371)) started in December 2024 about "Start using and suggesting standardized Tree-sitter highlights captures". The community recognizes that:
- Zed currently uses non-standard captures
- Standardization with Neovim/Helix would be beneficial
- Migration needs to avoid syntax highlighting regressions

**No Official Migration Timeline**: As of now, there's no formal GitHub issue or roadmap item for migrating to nvim-treesitter scope conventions. The discussion is exploratory, and any migration would need careful planning to avoid breaking existing themes and extensions.

## Scope Mapping Reference

For future migration, here's the complete mapping:

| Modern (nvim-treesitter) | Legacy (Zed Current) |
|--------------------------|----------------------|
| `@markup.heading` | `@text.title` |
| `@markup.italic` | `@text.emphasis` |
| `@markup.bold` | `@emphasis.strong` |
| `@markup.strikethrough` | `@text.strike` |
| `@markup.raw.block` | `@text.literal` |
| `@markup.raw.inline` | `@text.literal` |
| `@markup.link.label` | `@text.reference` |
| `@markup.link.url` | `@text.uri` |
| `@markup.list.marker` | `@punctuation.special` |
| `@markup.quote.marker` | `@punctuation.special` |
| `@markup.math.inline` | `@string` |
| `@markup.math.block` | `@string` |
| `@keyword.directive` | `@constant.macro` (shortcodes) |
| `@attribute` | `@property` |

## Related Files

- **Block grammar highlights**: `languages/quarto/highlights.scm`
- **Inline grammar highlights override**: `languages/pandoc_markdown_inline/highlights.scm`
- **Test-time patching**: `build.rs` (lines 87-120)
- **Test scope assertions**: `tests/emphasis_highlighting.rs`, `tests/highlights.rs`
- **Grammar source**: `tree-sitter-pandoc-markdown` repository

## Zed's Currently Supported Scopes

For reference, here's the complete list of scopes documented in Zed (as of October 2025):

**Basic Types:**
- `@attribute`, `@boolean`, `@comment`, `@comment.doc`, `@constant`, `@constructor`, `@embedded`
- `@enum`, `@function`, `@hint`, `@keyword`, `@label`, `@number`, `@operator`
- `@preproc`, `@primary`, `@property`, `@tag`, `@tag.doctype`, `@title`, `@type`
- `@variable`, `@variable.special`, `@variant`

**Emphasis & Formatting:**
- `@emphasis`, `@emphasis.strong`, `@text.literal`

**Links & References:**
- `@link_text`, `@link_uri`

**Punctuation:**
- `@punctuation`, `@punctuation.bracket`, `@punctuation.delimiter`
- `@punctuation.list_marker`, `@punctuation.special`

**Strings:**
- `@string`, `@string.escape`, `@string.regex`, `@string.special`, `@string.special.symbol`

**Prediction:**
- `@predictive`

Source: [Zed Language Extensions Documentation](https://zed.dev/docs/extensions/languages)

## References

- [nvim-treesitter scope conventions](https://github.com/nvim-treesitter/nvim-treesitter/blob/master/CONTRIBUTING.md#parser-configurations)
- [Tree-sitter syntax highlighting](https://tree-sitter.github.io/tree-sitter/syntax-highlighting)
- [Zed extension documentation](https://zed.dev/docs/extensions/languages)
- [Zed discussion on standardized captures](https://github.com/zed-industries/zed/discussions/23371)
- This decision made: October 2025
