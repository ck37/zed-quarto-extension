# Quarto extension for Zed

[![CI](https://github.com/ck37/zed-quarto-extension/actions/workflows/ci.yml/badge.svg)](https://github.com/ck37/zed-quarto-extension/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-available-blue)](docs/)

Quarto brings literate programming to Zed with first-class syntax highlighting for `.qmd` files. This extension uses `tree-sitter-quarto` for Quarto-native syntax highlighting.

> **Note**: This extension addresses [zed-industries/zed#12406](https://github.com/zed-industries/zed/issues/12406).

## Installation

1. Clone this repository locally.
2. In Zed, open the command palette and run `zed: install dev extension`, then select this repository directory.

Zed will automatically compile the extension and its grammars.

## Features

- Quarto-native syntax highlighting powered by [`tree-sitter-quarto`](https://github.com/ck37/tree-sitter-quarto)
  - Unified grammar architecture (single grammar vs dual grammar approach)
  - Core Markdown structures: headings, code blocks, lists, emphasis, YAML front matter
  - Quarto-specific features: executable code cells (`{python}`), inline code cells, chunk options (`#|`)
  - Pandoc extensions: fenced divs, attribute lists, citations, cross-references, footnotes
  - Math support: inline (`$...$`) and display (`$$...$$`) with LaTeX syntax
  - Pipe tables with alignment markers
- Embedded language injections for 15+ languages in code chunks (Python, R, Julia, SQL, JavaScript, TypeScript, and more)

> **Note**: This extension provides syntax highlighting only. For language server features (completions, hover, diagnostics), see [`docs/LSP_STATUS.md`](docs/LSP_STATUS.md) for the current state and options.

## Highlighting Features

Inline formatting support:
- Bold (`**text**`)
- Italic (`*text*`)
- Inline code (`` `code` ``)
- Citations (`@reference`, `[@reference]`)
- Links (`[text](url)`) - Known grammar parsing issue ([tree-sitter-quarto#2](https://github.com/ck37/tree-sitter-quarto/issues/2))

Block-level elements:
- Headings (all levels, including hyphenated headings)
- Code blocks with syntax highlighting
- Executable code cells (`{python}`, `{r}`, etc.)
- YAML front matter
- Lists (ordered and unordered)
- Block quotes
- Tables

## Known Limitations

- Link parsing: Markdown links are currently not parsing correctly and show as ERROR nodes in the parse tree ([tree-sitter-quarto#2](https://github.com/ck37/tree-sitter-quarto/issues/2)). Highlight queries are in place and ready to work once the grammar is fixed.
- Preview/render workflows: Out of scope for this extension—use the Quarto CLI or VSCode extension for visual editing and preview.
- Grammar maturity: `tree-sitter-quarto` is a new grammar under active development. Some edge cases in Quarto/Pandoc syntax may not be fully supported yet. Please report issues at [tree-sitter-quarto](https://github.com/ck37/tree-sitter-quarto).

## Contributing

Contributions are welcome! See [`CONTRIBUTING.md`](CONTRIBUTING.md) for development setup, testing, and architecture details.

If you're interested in helping improve `tree-sitter-quarto`, please contribute at [github.com/ck37/tree-sitter-quarto](https://github.com/ck37/tree-sitter-quarto)—this would benefit the entire Quarto ecosystem across all editors.

## Testing

The extension includes automated tests to prevent regressions:

```bash
# Run all tests
cargo test --workspace --all-features

# Run highlights validation (prevents query syntax errors)
cargo test --test highlights_validation
```
