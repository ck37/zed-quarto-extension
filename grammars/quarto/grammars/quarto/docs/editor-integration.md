# Editor Integration Guide

**Audience:** Editor extension developers integrating tree-sitter-quarto

This document provides guidance for integrating tree-sitter-quarto into editor plugins and extensions.

## Scope Naming Philosophy

tree-sitter-quarto uses **standard tree-sitter scope conventions** to remain editor-agnostic:

```scheme
(atx_heading) @markup.heading          # Standard across editors
(emphasis) @markup.italic              # Not Zed-specific @text.emphasis
(code_span) @markup.raw.inline         # Works in Neovim, Helix, VSCode
(shortcode_name) @function             # Semantic, not presentation
(chunk_option_key) @property           # Universal scope
```

### Why Standard Scopes?

- ✅ **Editor agnostic**: Same grammar works everywhere
- ✅ **Single source of truth**: One `queries/highlights.scm` to maintain
- ✅ **Separation of concerns**: Grammar = parsing, extension = presentation
- ✅ **Standard practice**: Follows tree-sitter ecosystem conventions

### Architecture: Grammar vs Extension

```
┌─────────────────────────────────────────────────┐
│ tree-sitter-quarto                              │
│ - Parses .qmd files                             │
│ - Provides semantic AST                         │
│ - Uses standard scopes (@markup.*, @function)   │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ Editor Extension (your code)                    │
│ - Loads grammar WASM                            │
│ - Remaps scopes if needed                       │
│ - Applies theme colors                          │
│ - Adds editor-specific features                 │
└─────────────────────────────────────────────────┘
```

## For Editor Extension Developers

### Recommended Approach: Scope Remapping

If your editor requires different scope names (e.g., Zed uses `@text.*` instead of `@markup.*`):

**✅ Recommended**: Handle scope remapping in your editor extension
- Load our `queries/highlights.scm`
- Remap scopes to your editor's conventions
- Example: `@markup.heading` → `@text.title` (Zed)

**❌ Not recommended**: Maintaining editor-specific query files in this repo
- Creates maintenance burden (N files for N editors)
- Violates separation of concerns
- Most editors can handle scope remapping

### Common Scope Remappings

Example mappings for editors that use non-standard scopes:

#### Zed Editor

```
@markup.heading     → @text.title
@markup.italic      → @text.emphasis
@markup.bold        → @text.strong
@markup.raw.inline  → @text.literal
@markup.link.text   → @text.link
```

#### Custom Editor

Consult your editor's theme/scope documentation and create similar mappings.

### Integration Steps

1. **Add grammar dependency**
   ```toml
   # For Rust-based editors
   [dependencies]
   tree-sitter-quarto = { git = "https://github.com/ck37/tree-sitter-quarto" }
   ```

2. **Load highlight queries**
   ```rust
   let highlights = include_str!("../queries/highlights.scm");
   ```

3. **Optional: Remap scopes**
   ```rust
   // Example pseudocode
   fn remap_scope(scope: &str) -> &str {
       match scope {
           "@markup.heading" => "@text.title",
           "@markup.italic" => "@text.emphasis",
           // ... other mappings
           _ => scope
       }
   }
   ```

4. **Load injection queries**
   ```rust
   let injections = include_str!("../queries/injections.scm");
   ```

5. **Configure language injection**
   - Ensure Python, R, Julia parsers are available
   - Enable multi-language highlighting
   - Test with sample `.qmd` files

### Testing Your Integration

**Test files provided:**
- `examples/sample.qmd` - Comprehensive feature showcase
- `examples/test-callout.qmd` - Callout blocks
- `examples/test-tabset.qmd` - Tabsets
- `examples/test-conditional.qmd` - Conditional content

**Validation checklist:**
- [ ] Syntax highlighting works for all Quarto features
- [ ] Language injection works (Python/R/Julia code highlighted)
- [ ] Code folding works on cells and divs
- [ ] No parse errors on valid `.qmd` files
- [ ] Performance acceptable on large documents

## Language Injection

tree-sitter-quarto supports injection of 15+ languages:

**Core languages:**
- Python
- R
- Julia
- SQL
- Bash/Shell

**Web languages:**
- JavaScript
- TypeScript

**Markup/diagram languages:**
- Mermaid
- Dot (Graphviz)
- HTML

**Queries location:** `queries/injections.scm`

Your editor extension needs to ensure these language parsers are available for injection to work.

## Query Files

tree-sitter-quarto provides comprehensive query files:

- **`queries/highlights.scm`** - Syntax highlighting (required)
- **`queries/injections.scm`** - Language injection (recommended)
- **`queries/folds.scm`** - Code folding (optional)
- **`queries/indents.scm`** - Indentation (optional)
- **`queries/locals.scm`** - Local scopes (optional)

### Minimal Integration

At minimum, load `highlights.scm` and `injections.scm` for basic functionality.

### Full Integration

Load all query files for complete editor experience including folding and smart indentation.

## Performance Considerations

tree-sitter-quarto is designed for editor performance:

- Incremental parsing (only re-parses changed sections)
- LR(1) parsing (linear time complexity)
- Target: <100ms for typical documents (not yet benchmarked)

**If you encounter performance issues:**
1. Ensure you're using incremental parsing
2. Profile with large `.qmd` files (1000+ lines)
3. Report performance issues with reproducible examples

## Editor-Specific Features

### Code Folding

Fold regions defined in `queries/folds.scm`:
- Executable code cells
- Fenced divs (callouts, tabsets, etc.)
- Lists
- YAML front matter

### Indentation

Smart indentation rules in `queries/indents.scm`:
- Cell content indentation
- Nested div indentation
- List item continuation

### Outline/Symbols

Use AST to build document outline:
- Headings (atx_heading, setext_heading)
- Executable cells (executable_code_cell with label)
- Callout blocks
- Tabsets

## Examples

### Neovim (nvim-treesitter)

```lua
-- Not yet published, example configuration:
require('nvim-treesitter.configs').setup {
  ensure_installed = { "quarto", "python", "r", "julia" },
  highlight = { enable = true },
  indent = { enable = true },
  fold = { enable = true },
}
```

### Zed Editor

See [zed-quarto-extension](https://github.com/ck37/zed-quarto-extension) for reference implementation with scope remapping.

### Helix

```toml
# Not yet available
```

## Troubleshooting

### Parse Errors

If you encounter parse errors on valid `.qmd` files:

1. **Verify grammar version**: Ensure you're using the latest release
2. **Check input encoding**: tree-sitter expects UTF-8
3. **Test with sample files**: Use `examples/sample.qmd` to isolate issues
4. **Report issues**: Include minimal reproducible example

### Highlighting Issues

If syntax highlighting doesn't work:

1. **Check query loading**: Ensure `highlights.scm` is loaded correctly
2. **Verify scope mapping**: If using custom scopes, check your remapping
3. **Test with simple file**: Start with basic example and add complexity
4. **Check theme support**: Ensure your theme defines colors for used scopes

### Injection Not Working

If language injection doesn't work:

1. **Verify language parsers**: Ensure Python/R/Julia parsers are installed
2. **Check injection queries**: Load `injections.scm`
3. **Test parser independently**: Verify `tree-sitter-python` etc. work
4. **Check parser priority**: Ensure Quarto parser has higher priority

## Resources

- **Grammar source**: https://github.com/ck37/tree-sitter-quarto
- **Query reference**: https://tree-sitter.github.io/tree-sitter/syntax-highlighting
- **Scope conventions**: Standard tree-sitter scopes (see queries/highlights.scm)
- **Reference implementation**: [zed-quarto-extension#4](https://github.com/ck37/zed-quarto-extension/issues/4)

## Contributing

If you build an editor integration:

1. **Share your extension**: Add it to our README
2. **Report issues**: Help us improve the grammar
3. **Share learnings**: Document editor-specific challenges
4. **Contribute query improvements**: If you find highlighting gaps

---

**Questions?** Open an issue at https://github.com/ck37/tree-sitter-quarto/issues
