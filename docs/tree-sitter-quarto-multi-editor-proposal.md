# Proposal: Multi-Editor Query Support for tree-sitter-quarto

## Summary

Add Zed-compatible query files to tree-sitter-quarto alongside the existing standard queries, following industry best practices from Helix, Neovim, and other editors.

## Background

### Current State
- tree-sitter-quarto has `queries/highlights.scm` using standard `@markup.*` scopes
- Zed themes don't recognize `@markup.*` scopes (only `@text.*`, `@emphasis.*`)
- zed-quarto-extension has to maintain separate query files in `languages/quarto/`
- **Problem:** Zed loads grammar's built-in queries instead of extension's overrides

### Industry Pattern

From tree-sitter community discussions:

1. **Queries are NOT canonical across editors** (tree-sitter/tree-sitter#2381)
   - "Each client owns the queries they use to integrate with each grammar"
   - Different editors support different capture names and predicates
   - Complete query sharing isn't practical

2. **Helix maintains editor-specific queries** (helix-editor/helix#3020)
   - Helix uses custom queries, not grammar's default queries
   - "Parsers can be re-used between editors but it isn't a goal to have all editors use the same queries"
   - Each editor has unique scoping and styling needs

3. **Query precedence differs**
   - Neovim: "last-wins" approach
   - Helix: Reverse precedence order (being standardized)
   - Zed: Follows Neovim/new standard

## Proposed Solution

### Option A: Editor-Specific Query Files (RECOMMENDED)

Add Zed-compatible queries alongside existing queries:

```
queries/
  highlights.scm          # Standard @markup.* scopes (current)
  highlights-zed.scm      # Zed-compatible @text.* scopes (new)
  injections.scm          # Shared (or separate if needed)
  indents.scm            # Shared
  folds.scm              # Shared
  locals.scm             # Shared
```

**Advantages:**
- Both query sets maintained in grammar repo
- Clear separation of editor-specific concerns
- Zed extension can reference `highlights-zed.scm` in `extension.toml`
- No breaking changes for existing users
- Follows Helix/Neovim pattern of editor-specific queries

**Implementation:**
1. Copy `queries/highlights.scm` → `queries/highlights-zed.scm`
2. Convert scopes in `highlights-zed.scm`:
   - `@markup.heading` → `@text.title`
   - `@markup.italic` → `@text.emphasis`
   - `@markup.bold` → `@emphasis.strong`
   - `@markup.raw.*` → `@text.literal`
   - `@markup.link.*` → `@text.reference` / `@text.uri`
   - `@markup.quote` → `@comment`
   - `@markup.math` → `@string`
3. Update Zed extension to use `highlights-zed.scm`

### Option B: Query Directory Structure

Create editor-specific subdirectories:

```
queries/
  standard/
    highlights.scm
    injections.scm
  zed/
    highlights.scm
    injections.scm
  helix/
    highlights.scm
```

**Advantages:**
- Cleaner organization for multiple editors
- Room for editor-specific predicates/queries

**Disadvantages:**
- More complex structure
- May not be standard in tree-sitter ecosystem
- Unclear if Zed supports subdirectories

### Option C: Keep Editor Queries in Extensions (CURRENT)

Each editor extension maintains its own queries:

**Advantages:**
- Grammar repo stays editor-agnostic
- Extensions have full control

**Disadvantages:**
- Query duplication across extensions
- No centralized query maintenance
- **Zed currently loads grammar queries instead of extension queries** (bug)

## Recommendation

**Use Option A** (editor-specific query files in grammar repo):

1. **Short-term:** Add `queries/highlights-zed.scm` to tree-sitter-quarto
2. **Medium-term:** Update zed-quarto-extension to reference it
3. **Long-term:** Consider contributing to Zed to properly support extension query overrides

## Scope Mapping Reference

### From Standard → Zed

| Standard Scope | Zed Scope | Notes |
|---|---|---|
| `@markup.heading` | `@text.title` | Headings |
| `@markup.italic` | `@text.emphasis` | Italic text |
| `@markup.bold` | `@emphasis.strong` | Bold text |
| `@markup.raw.inline` | `@text.literal` | Inline code |
| `@markup.raw.block` | `@text.literal` | Code blocks |
| `@markup.link.label` | `@text.reference` | Link text |
| `@markup.link.url` | `@text.uri` | Link URLs |
| `@markup.quote` | `@comment` | Block quotes |
| `@markup.math` | `@string` | Math expressions |
| `@markup.list` | `@punctuation.special` | List markers |

### Zed-Specific Scopes

Additional scopes used in Zed highlighting:
- `@constant.builtin` - Cross-references, special constants
- `@constant.macro` - Shortcodes
- `@property` - Attributes, keys
- `@variable.parameter` - Reference IDs
- `@function.builtin` - Language names in code cells
- `@punctuation.delimiter` - Delimiters, fences
- `@punctuation.bracket` - Brackets
- `@embedded` - Embedded content

## Testing

The zed-quarto-extension now has automated tests (`tests/heading_highlighting.rs`) that verify:
- Grammar parses Quarto correctly
- Query scopes produce expected highlighting
- Headings with hyphens work (e.g., `## data-driven-analysis`)

## References

- [Tree-sitter canonical queries discussion](https://github.com/tree-sitter/tree-sitter/discussions/2381)
- [Helix custom queries discussion](https://github.com/helix-editor/helix/discussions/3020)
- [Zed scope naming decision](./scope-naming-decision.md)
- [Highlighting failure analysis](./highlighting-failure-analysis.md)

## Next Steps

1. Create PR to tree-sitter-quarto adding `queries/highlights-zed.scm`
2. Update zed-quarto-extension `extension.toml` to reference it (if Zed supports custom query names)
3. If Zed doesn't support custom query names yet, file issue with Zed
4. Document workaround (`fix-zed-queries.sh`) until proper support exists
