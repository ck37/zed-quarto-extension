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

Our extension now ships with the upstream [`tree-sitter-pandoc-markdown` feat/phase-1-pandoc-grammar branch](https://github.com/ck37/tree-sitter-pandoc-markdown/tree/feat/phase-1-pandoc-grammar), enabling **Pandoc-aware highlighting** that understands Quarto-relevant constructs such as fenced divs, citations, shortcodes, chunk options, YAML front matter, math, pipe tables, and footnotes. Phase 1 is now complete! Remaining features are tracked in [`grammar-feature-needs.md`](grammar-feature-needs.md).

## Why tree-sitter-pandoc-markdown?

Quarto documents (`.qmd`) are **not** plain Markdown—they're based on [Pandoc's Markdown](https://pandoc.org/MANUAL.html#pandocs-markdown), which includes additional syntax:

- **Div blocks**: `:::` with attributes like `{.class #id}`
- **Citations**: `@citation` references
- **Shortcodes**: `{{< shortcode >}}` syntax
- **Fenced divs**: Extended YAML frontmatter
- **Code chunks**: Special attributes beyond standard fenced code blocks

## TextMate vs Tree-sitter

The [official Quarto VSCode extension](https://github.com/quarto-dev/quarto/tree/main/apps/vscode) uses **TextMate grammars** (`.tmLanguage` files), which are regex-based pattern matching systems. However, **Zed only supports tree-sitter grammars**, which are proper parsers.

Since there is no official `tree-sitter-quarto` grammar yet, we use [`tree-sitter-pandoc-markdown`](https://github.com/ck37/tree-sitter-pandoc-markdown), which extends `tree-sitter-markdown` with Pandoc-specific features that Quarto is built upon.

This approach provides:
- Proper parsing of Quarto/Pandoc syntax constructs
- Accurate syntax highlighting for citations, divs, shortcodes
- Better error recovery than regex-based approaches
- Foundation for future code navigation features
