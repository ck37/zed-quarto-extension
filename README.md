# Quarto extension for Zed

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

## Known Limitations

- Bold/italic highlighting partially working: The pandoc-markdown grammar uses a dual-grammar architecture (separate block and inline grammars), but extension-to-extension grammar injection didn't work in Zed.
  - Current workaround: Injecting Zed's built-in `markdown-inline` grammar provides ~70% coverage:
    - ✅ Works: Bold (`**`/`__`), italic (`*`/`_`), inline code
    - ❌ Doesn't work: Links, mixed content (partially), Pandoc extensions (strikethrough, subscript, superscript)
  - Root cause (confirmed): When extension grammars load asynchronously, the LanguageRegistry version wasn't incremented, so pending injections were never resolved. Built-in grammars worked because they're immediately available.
  - Fix implemented: One-line change to increment registry version when languages load, enabling pending injections to be resolved. See [zed-fix-implemented.md](docs/bold-highlighting-investigation/zed-fix-implemented.md)
  - Investigation: Complete technical analysis and verification in [`docs/bold-highlighting-investigation/`](docs/bold-highlighting-investigation/)
  - Status: Fix pending testing and PR to Zed. Once merged, this extension will switch to full `pandoc_markdown_inline` grammar for 100% coverage.
  - Timeline: Workaround active now (70% coverage); full fix expected within weeks pending Zed PR review
- Preview/render workflows are out of scope for this extension—use the Quarto CLI or VSCode extension for visual editing and preview.
- Grammar completeness: `tree-sitter-pandoc-markdown` is a community project that extends `tree-sitter-markdown`. Some edge cases in Pandoc syntax may not be fully supported yet.
- No official tree-sitter-quarto: Until an official Quarto grammar exists, we rely on Pandoc markdown as the closest approximation.

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
