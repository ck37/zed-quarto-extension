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

Syntax highlighting powered by [`tree-sitter-quarto`](https://github.com/ck37/tree-sitter-quarto):

- **Quarto-specific**: executable code cells, chunk options (`#| key: value`), inline code cells, cross-references
- **Pandoc extensions**: fenced divs, attribute lists, citations, shortcodes, strikethrough (`~~text~~`), subscript (`H~2~O`), superscript (`x^2^`), highlight (`==text==`), underline (`[text]{.underline}`)
- **Core Markdown**: headings, bold (`**text**`), italic (`*text*`), links (`[text](url)`), inline code (`` `code` ``), code blocks, lists, YAML front matter
- **Math support**: inline (`$...$`) and display (`$$...$$`) with LaTeX syntax
- **Tables**: pipe tables with alignment markers
- **Embedded language injections**: Python, R, Julia, SQL, JavaScript, TypeScript, Bash code chunks

> **Note**: This extension provides syntax highlighting only. For language server features (completions, hover, diagnostics), see [`docs/lsp-status.md`](docs/lsp-status.md) for the current state and options.

## Known Limitations

### Highlighting Limitations

Some Pandoc extensions parse correctly but don't highlight in Zed due to theme limitations: strikethrough (`@text.strike`), highlight/mark (`@text.highlight`), subscript (`@text.subscript`), and superscript (`@text.super`). These will highlight if/when Zed adds theme support. See [`docs/pandoc-extensions-scope-issue.md`](docs/pandoc-extensions-scope-issue.md) for details.

### Other Limitations

- **Preview/render workflows**: Out of scope for this extension—use the Quarto CLI or VSCode extension for visual editing and preview.
- **Grammar completeness**: tree-sitter-quarto is in alpha (58/58 tests passing). Some edge cases in Quarto/Pandoc syntax may not be fully supported yet. See the [grammar repository](https://github.com/ck37/tree-sitter-quarto) for current status.

## Contributing

Contributions are welcome! See [`CONTRIBUTING.md`](CONTRIBUTING.md) for development setup, testing, and architecture details.
