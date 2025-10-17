# Tree-sitter Query Files

This directory contains tree-sitter query files for syntax highlighting and other editor features.

## Structure

### Standard Queries (`queries/`)

- **highlights.scm** - Uses modern nvim-treesitter scope conventions (`@markup.*`)
  - Compatible with Neovim, Helix, and other editors following nvim-treesitter standards
  - Future-proof as the ecosystem migrates to unified scope naming

- **injections.scm** - Language injection queries for embedded code blocks
- **folds.scm** - Code folding patterns
- **indents.scm** - Smart indentation rules
- **locals.scm** - Variable scoping for code intelligence

### Zed-Specific Queries (`queries/zed/`)

- **highlights.scm** - Uses legacy Zed scope conventions (`@text.*`, `@emphasis.strong`)
  - Required for compatibility with Zed's current theming system
  - Zed automatically prefers `queries/zed/` over standard queries when available
  - Will be deprecated once Zed adopts nvim-treesitter scope conventions

## Scope Naming Conventions

### Modern (nvim-treesitter)

Used in `queries/highlights.scm`:

| Construct | Modern Scope |
|-----------|-------------|
| Headings | `@markup.heading` |
| Italic | `@markup.italic` |
| Bold | `@markup.bold` |
| Inline code | `@markup.raw.inline` |
| Code blocks | `@markup.raw.block` |
| Link text | `@markup.link.label` |
| Link URL | `@markup.link.url` |
| Block quotes | `@markup.quote` |
| List markers | `@markup.list.marker` |
| Inline math | `@markup.math.inline` |
| Display math | `@markup.math.block` |

### Legacy (Zed)

Used in `queries/zed/highlights.scm`:

| Construct | Legacy Scope |
|-----------|-------------|
| Headings | `@text.title` |
| Italic | `@text.emphasis` |
| Bold | `@emphasis.strong` |
| Code (any) | `@text.literal` |
| Link text | `@text.reference` |
| Link URL | `@text.uri` |
| Block quotes | `@comment` |
| List markers | `@punctuation.special` |
| Math (any) | `@string` |

## References

- [nvim-treesitter scope naming](https://github.com/nvim-treesitter/nvim-treesitter/blob/master/CONTRIBUTING.md#parser-configurations)
- [Zed scope naming decision](https://github.com/ck37/zed-quarto-extension/blob/main/docs/scope-naming-decision.md)
- [tree-sitter query syntax](https://tree-sitter.github.io/tree-sitter/syntax-highlighting#queries)

## Maintenance

When updating syntax highlighting:

1. Update `queries/highlights.scm` with modern `@markup.*` scopes
2. Update `queries/zed/highlights.scm` with corresponding legacy scopes
3. Keep both files structurally identical except for scope names
4. Run `npx tree-sitter test` to verify queries are valid

Once Zed adopts nvim-treesitter conventions, `queries/zed/highlights.scm` can be removed.
