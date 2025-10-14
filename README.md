# Quarto extension for Zed

[![CI](https://github.com/ck37/zed-quarto-extension/actions/workflows/ci.yml/badge.svg)](https://github.com/ck37/zed-quarto-extension/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-available-blue)](docs/)

Quarto brings literate programming to Zed with first-class syntax highlighting for `.qmd` files. This extension uses `tree-sitter-pandoc-markdown` for Pandoc-aware syntax highlighting.

> **Note**: This extension addresses [zed-industries/zed#12406](https://github.com/zed-industries/zed/issues/12406).

## Installation

1. Clone this repository locally.
2. In Zed, open the command palette and run `zed: install dev extension`, then select this repository directory.

Zed will automatically compile the extension and its grammars.

## Features

- Pandoc-aware syntax highlighting powered by [`tree-sitter-pandoc-markdown`](https://github.com/ck37/tree-sitter-pandoc-markdown/tree/feat/phase-1-pandoc-grammar)
  - Core Markdown structures: headings, links, code blocks, lists, emphasis, YAML front matter
  - Pandoc extensions heavily used by Quarto: fenced divs, attribute lists, citations, cross-references, shortcodes, chunk options
  - Math support: inline (`$...$`) and display (`$$...$$`) with LaTeX syntax
  - Pipe tables with alignment markers
  - Footnotes and cross-references
- Embedded language injections for common Quarto code chunks (Python, R, Julia, SQL)

> **Note**: This extension provides syntax highlighting only. For language server features (completions, hover, diagnostics), see [`docs/LSP_STATUS.md`](docs/LSP_STATUS.md) for the current state and options.

## Highlighting Features

- **Full inline formatting support**: The pandoc-markdown grammar uses a dual-grammar architecture (separate block and inline grammars) with extension-to-extension grammar injection.
  - ✅ Bold (`**text**`, `__text__`)
  - ✅ Italic (`*text*`, `_text_`)
  - ✅ Bold+italic (`***text***`)
  - ✅ Inline code (`` `code` ``)
  - ✅ Links (`[text](url)`)
  - ✅ Pandoc extensions: strikethrough (`~~text~~`), subscript (`H~2~O`), superscript (`x^2^`), highlight (`==text==`), underline (`[text]{.underline}`)

  Note: Initial investigation suggested Zed might need modifications to support extension-to-extension injection, but testing confirmed the mechanism works correctly. See [`docs/bold-highlighting-investigation/`](docs/bold-highlighting-investigation/) for the complete investigation.

## Known Limitations

- **Hyphenated headings**: Headings containing hyphens (e.g., `## Non-Longitudinal Clustering`) don't receive heading color due to dual-grammar injection limitations. See [`docs/hyphenated-headings-issue.md`](docs/hyphenated-headings-issue.md) for details and the recommended solution (migrate to tree-sitter-quarto).
- **Preview/render workflows**: Out of scope for this extension—use the Quarto CLI or VSCode extension for visual editing and preview.
- **Grammar completeness**: `tree-sitter-pandoc-markdown` is a community project that extends `tree-sitter-markdown`. Some edge cases in Pandoc syntax may not be fully supported yet.

**Migration Path**: [tree-sitter-quarto](https://github.com/ck37/tree-sitter-quarto) is now production-ready (alpha complete, 58/58 tests passing). See [issue #3](https://github.com/ck37/zed-quarto-extension/issues/3) for migration plan. This will fix the hyphenated headings issue and provide better Quarto-specific support.

## Architecture

Quarto documents (`.qmd`) are based on [Pandoc's Markdown](https://pandoc.org/MANUAL.html#pandocs-markdown), not standard Markdown. This extension uses [`tree-sitter-pandoc-markdown`](https://github.com/ck37/tree-sitter-pandoc-markdown) for Pandoc-aware syntax highlighting, since Zed requires tree-sitter grammars (not TextMate/regex-based grammars like VSCode uses).

Tree-sitter provides proper parsing with better error recovery and forms the foundation for advanced editor features (code navigation, folding, refactoring).

For technical details on the grammar architecture and roadmap, see:
- [`docs/syntax-highlighting-architecture.md`](docs/syntax-highlighting-architecture.md) - Technical comparison of grammar systems
- [`docs/grammar-roadmap.md`](docs/grammar-roadmap.md) - Development phases and future plans
- [`docs/tree-sitter-quarto-plan.md`](docs/tree-sitter-quarto-plan.md) - Detailed implementation guide for dedicated Quarto grammar

## Contributing

Contributions are welcome! See [`CONTRIBUTING.md`](CONTRIBUTING.md) for development setup, testing, and architecture details.

If you're interested in helping create a dedicated `tree-sitter-quarto` grammar, see [`docs/tree-sitter-quarto-plan.md`](docs/tree-sitter-quarto-plan.md)—this would benefit the entire Quarto ecosystem across all editors.
