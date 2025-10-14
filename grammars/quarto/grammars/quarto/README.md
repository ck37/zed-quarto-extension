# tree-sitter-quarto

[![CI](https://github.com/ck37/tree-sitter-quarto/workflows/CI/badge.svg)](https://github.com/ck37/tree-sitter-quarto/actions)
[![Tests](https://img.shields.io/badge/tests-58%2F58%20passing-brightgreen)](https://github.com/ck37/tree-sitter-quarto/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![tree-sitter](https://img.shields.io/badge/tree--sitter-0.21+-orange)](https://tree-sitter.github.io/)
[![Node Version](https://img.shields.io/badge/node-%3E%3D16-brightgreen)](https://nodejs.org/)
[![Status](https://img.shields.io/badge/status-alpha-yellow)](./docs/plan.md)
[![Spec Coverage](https://img.shields.io/badge/spec%20coverage-98%25-brightgreen)](./openspec)

Tree-sitter parser for [Quarto Markdown](https://quarto.org/) (`.qmd` files), optimized for editor integration.

Status: Alpha Complete - All core features implemented, ready for editor integration

## What is this?

A [tree-sitter](https://tree-sitter.github.io/) parser that understands Quarto's extended Markdown syntax, enabling rich editor features:

- Semantic syntax highlighting - Distinct colors for chunk options, cross-references, executable cells
- Jump-to-definition - Navigate from `@fig-plot` to figure definition
- Autocomplete - Suggest valid chunk option names and values
- Code folding - Collapse executable cells and divs
- Document outline - Navigate structure including cells and callouts

## Why?

Existing parsers serve different purposes:
- [tree-sitter-pandoc-markdown](https://github.com/ck37/tree-sitter-pandoc-markdown) - Great for Pandoc, but not Quarto-aware
- [Quarto Markdown Parser](https://github.com/quarto-dev/quarto-markdown) - Official tree-sitter grammars exist but not production-ready until early 2026

tree-sitter-quarto fills the gap by providing **production-ready semantic parsing** optimized for editor tooling as you author Quarto documents.

**Editor Support:** This parser enables first-class Quarto support in editors like [Zed](https://github.com/zed-industries/zed/issues/12406), Neovim, Helix, and VSCode.

See [detailed comparison](./docs/comparison.md) for more information.

## Features

Fully implemented:

- Executable code cells - Parse `{python}`, `{r}`, `{julia}` with semantic nodes
- Chunk options - Parse `#| key: value` as structured data
- Cross-references - Distinguish `@fig-plot` from `@smith2020` citations
- Inline code cells - `` `{python} expr` `` with language injection
- Shortcodes - `{{< video url >}}` in block and inline contexts
- Enhanced divs - Callouts, tabsets, conditional content
  - `::: {.callout-note}` - 5 types: note, warning, important, tip, caution
  - `::: {.panel-tabset}` - Tab structure with groups
  - `::: {.content-visible when-format="html"}` - Conditional content
- Language injection - 15+ languages (Python, R, Julia, SQL, Bash, JS, Mermaid, etc.)
- Full Pandoc Markdown - Headings, emphasis, links, images, tables, etc.

Known limitations:

- Generic fenced divs (`::: {.custom-class}`) don't parse - [technical details](./docs/generic-fenced-div-limitation.md)
- Multi-line chunk option values not supported
- See [plan.md](./docs/plan.md) for complete list

## Relationship to Official Quarto Grammars

The [quarto-markdown repository](https://github.com/quarto-dev/quarto-markdown) contains official tree-sitter grammars that are also intended for editor integration (RStudio, Positron, etc.). However, these grammars are **not yet production-ready** and are planned for early 2026.

**tree-sitter-quarto is a bridge solution:**
- ✅ **Production-ready NOW** (2025) - All features implemented and tested
- ✅ **Complete package** - Includes comprehensive query files for syntax highlighting
- ✅ **Ready for editor integration** - Proven in real-world use
- ⏳ **Migration path** - Plan to migrate to official grammars when they reach production status (2026+)

**Why official grammars will be better long-term:**
- Battle-tested in Posit's production editors (RStudio, Positron)
- Official support and long-term maintenance
- The "blessed" standard across Quarto/Posit ecosystem

**Current recommendation:** Use tree-sitter-quarto for editor integration today, plan migration to official grammars in 2026+ when production-ready.

See [detailed comparison](./docs/comparison.md) for architecture differences and migration considerations.

## Quick Example

Input `.qmd` file:

````markdown
---
title: "My Analysis"
format: html
---

## Results

See @fig-plot for details.

```{python}
#| label: fig-plot
#| echo: false
import matplotlib.pyplot as plt
plt.plot([1, 2, 3])
```

::: {.callout-note}
The mean is `{python} mean([1, 2, 3])`.
:::

{{< video https://example.com/demo.mp4 >}}
````

Output AST (simplified):

```
(document
  (yaml_front_matter ...)
  (atx_heading ...)
  (paragraph
    (cross_reference type:"fig" id:"plot"))
  (executable_code_cell
    language: "python"
    (chunk_options
      (chunk_option key:"label" value:"fig-plot")
      (chunk_option key:"echo" value:"false"))
    content: ...)
  (callout_block
    (callout_open)
    (inline_code_cell language:"python" ...))
  (shortcode_block name:"video" ...))
```

## Installation

For editor extension developers:

```toml
# Cargo.toml
[dependencies]
tree-sitter-quarto = { git = "https://github.com/ck37/tree-sitter-quarto" }
```

See [editor integration guide](./docs/editor-integration.md) for detailed instructions.

## Documentation

See the [docs/](./docs/) directory for detailed documentation including implementation plan, parser comparisons, editor integration guide, and technical details.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md) for development workflow.

## License

MIT License - see [LICENSE](./LICENSE) file for details
