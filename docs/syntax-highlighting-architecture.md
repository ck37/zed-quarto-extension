# Syntax Highlighting Technical Details

## Why Different Editors Have Different Approaches

| Editor | Grammar System | Approach |
|--------|---------------|----------|
| **VSCode** | TextMate (regex) | Pattern matching with `.tmLanguage` files |
| **RStudio** | Ace Editor (regex) | Custom JavaScript modes with regex rules |
| **Zed** | Tree-sitter (AST) | Proper parser generating abstract syntax trees |

**RStudio and VSCode** use **regex-based pattern matching** (TextMate grammars), which is simpler but less powerful. They can quickly add Quarto-specific patterns without building a full parser.

**Zed requires tree-sitter grammars**, which are proper parsers that understand the syntax structure. This provides:
- More accurate parsing and better error recovery
- Foundation for advanced features (folding, navigation, refactoring)
- Better performance for large files
- More effort to implement Quarto-specific syntax
- Limited to what the grammar defines

## Current State

Our extension uses [`tree-sitter-quarto`](https://github.com/ck37/tree-sitter-quarto), enabling comprehensive syntax highlighting for Quarto documents including **Quarto-specific features** (executable code cells, chunk options, inline code cells, cross-references), **Pandoc extensions** (fenced divs, citations, shortcodes, attribute lists), and **core Markdown** (headings, bold, italic, links, code blocks, lists). Remaining features are tracked in [`grammar-feature-needs.md`](grammar-feature-needs.md).

## Why tree-sitter-quarto?

Quarto documents (`.qmd`) are **not** plain Markdown—they're based on [Pandoc's Markdown](https://pandoc.org/MANUAL.html#pandocs-markdown) with Quarto-specific extensions:

- **Quarto code cells**: Executable code blocks with `#| option: value` syntax
- **Div blocks**: `:::` with attributes like `{.class #id}`
- **Citations**: `@citation` references
- **Shortcodes**: `{{< shortcode >}}` syntax
- **Cross-references**: `@fig-plot`, `@tbl-data`
- **Extended YAML frontmatter**: Quarto-specific metadata

## TextMate vs Tree-sitter

The [official Quarto VSCode extension](https://github.com/quarto-dev/quarto/tree/main/apps/vscode) uses **TextMate grammars** (`.tmLanguage` files), which are regex-based pattern matching systems. However, **Zed only supports tree-sitter grammars**, which are proper parsers.

We use [`tree-sitter-quarto`](https://github.com/ck37/tree-sitter-quarto), which extends `tree-sitter-markdown` with both Pandoc-specific features and Quarto-only syntax.

This approach provides:
- Proper parsing of Quarto/Pandoc syntax constructs
- Accurate syntax highlighting for citations, divs, shortcodes
- Better error recovery than regex-based approaches
- Foundation for future code navigation features
