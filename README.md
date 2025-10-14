# Quarto extension for Zed

[![CI](https://github.com/ck37/zed-quarto-extension/actions/workflows/ci.yml/badge.svg)](https://github.com/ck37/zed-quarto-extension/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-available-blue)](docs/)

Quarto brings literate programming to Zed with first-class syntax highlighting for `.qmd` files. This extension uses `tree-sitter-quarto` for comprehensive Quarto-aware syntax highlighting.

> **Note**: This extension addresses [zed-industries/zed#12406](https://github.com/zed-industries/zed/issues/12406).

## Installation

1. Clone this repository locally.
2. In Zed, open the command palette and run `zed: install dev extension`, then select this repository directory.

Zed will automatically compile the extension and its grammars.

## Features

- Quarto-aware syntax highlighting powered by [`tree-sitter-quarto`](https://github.com/ck37/tree-sitter-quarto)
  - Quarto-specific features: executable code cells, chunk options (`#| key: value`), inline code cells, cross-references
  - Pandoc extensions: fenced divs, attribute lists, citations, shortcodes
  - Core Markdown: headings, links, code blocks, lists, emphasis, YAML front matter
  - Math support: inline (`$...$`) and display (`$$...$$`) with LaTeX syntax
  - Pipe tables with alignment markers
  - Footnotes and cross-references
- Embedded language injections for Quarto code chunks (Python, R, Julia, SQL, JavaScript, TypeScript, Bash)

> **Note**: This extension provides syntax highlighting only. For language server features (completions, hover, diagnostics), see [`docs/LSP_STATUS.md`](docs/LSP_STATUS.md) for the current state and options.

## Highlighting Features

tree-sitter-quarto provides unified grammar handling for both block and inline content:

- Bold (`**text**`, `__text__`)
- Italic (`*text*`, `_text_`)
- Bold+italic (`***text***`)
- Inline code (`` `code` ``)
- Links (`[text](url)`)
- Pandoc extensions: strikethrough (`~~text~~`), subscript (`H~2~O`), superscript (`x^2^`), highlight (`==text==`), underline (`[text]{.underline}`)

## Known Limitations

- Preview/render workflows: Out of scope for this extension—use the Quarto CLI or VSCode extension for visual editing and preview.
- Grammar completeness: tree-sitter-quarto is in alpha (58/58 tests passing). Some edge cases in Quarto/Pandoc syntax may not be fully supported yet. See the [grammar repository](https://github.com/ck37/tree-sitter-quarto) for current status.

## Architecture

Quarto documents (`.qmd`) are based on [Pandoc's Markdown](https://pandoc.org/MANUAL.html#pandocs-markdown), not standard Markdown. This extension uses [`tree-sitter-quarto`](https://github.com/ck37/tree-sitter-quarto), a unified grammar specifically designed for Quarto files, since Zed requires tree-sitter grammars (not TextMate/regex-based grammars like VSCode uses).

Tree-sitter provides proper parsing with better error recovery and forms the foundation for advanced editor features (code navigation, folding, refactoring).

For technical details on the grammar and architecture, see:
- [`docs/syntax-highlighting-architecture.md`](docs/syntax-highlighting-architecture.md) - Technical comparison of grammar systems
- [`docs/scope-naming-decision.md`](docs/scope-naming-decision.md) - Why this extension uses Zed-compatible scope names

## Contributing

Contributions are welcome! See [`CONTRIBUTING.md`](CONTRIBUTING.md) for development setup, testing, and architecture details.
