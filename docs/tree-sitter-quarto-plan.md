# tree-sitter-quarto Implementation Plan

This document provides a detailed implementation guide for creating a dedicated `tree-sitter-quarto` grammar.

## Overview

With the Pandoc grammar enriched (Phase 1 complete), we can proceed with a dedicated `tree-sitter-quarto` that:

- Provides first-class support for all remaining Quarto syntax
- Enables proper highlighting of Quarto-only features (chunk options, execution directives, layout rules)
- Allows the grammar to be adopted by other editors (Neovim, Helix, etc.)
- Potentially lives in the official `tree-sitter-grammars` organization
- Is maintained in collaboration with the Quarto project

## Implementation Steps

### 1. Initialize grammar repository

```bash
npm install -g tree-sitter-cli
tree-sitter init quarto
```

### 2. Inherit from tree-sitter-markdown

Add as git submodule and extend:

```javascript
// In grammar.js
const markdown = require('tree-sitter-markdown/common/common.js');

module.exports = grammar(markdown, {
  name: 'quarto',

  rules: {
    // Extend markdown with Quarto features
    document: $ => repeat(choice(
      $.yaml_frontmatter,
      $.callout,
      $.div_block,
      $.shortcode,
      ...markdown.rules.document  // Inherit base markdown
    )),

    yaml_frontmatter: $ => seq(
      '---',
      $._newline,
      $.yaml_content,
      '---'
    ),

    callout: $ => seq(/* ... */),
    div_block: $ => seq(/* ... */),
    shortcode: $ => seq(/* ... */)
  }
});
```

### 3. Define all Quarto syntax

- YAML frontmatter with Quarto-specific keys
- Div blocks with attributes `:::{.class #id}`
- Callouts `:::{.callout-note}`, `:::{.callout-warning}`
- Shortcodes `{{< include file.qmd >}}`
- Code chunks with Quarto execution options
- Cross-references `@fig-plot`, `@tbl-data`
- Citations `@smith2024`

### 4. Create comprehensive test suite

Cover all Quarto syntax constructs.

### 5. Publish to npm

Propose to Quarto team for official adoption.

### 6. Update this extension

Switch from `tree-sitter-pandoc-markdown` to `tree-sitter-quarto`.

## Repository Structure

```
tree-sitter-quarto/
├── grammar.js              # Grammar definition
├── src/
│   ├── parser.c           # Generated C parser
│   └── scanner.c          # Custom scanner for complex rules
├── queries/
│   ├── highlights.scm     # Syntax highlighting
│   ├── injections.scm     # Language injection (R, Python in chunks)
│   ├── indents.scm        # Indentation rules
│   └── outline.scm        # Document outline
├── test/corpus/           # Test cases
└── package.json
```

## Quarto-Specific Syntax to Implement

The dedicated grammar focuses on Quarto-only constructs that remain after Phase 1:

1. **Executable chunk option lines** – `#| echo: false`, `#| warning: false`, multi-line option blocks.
2. **Cell attribute blocks & layout directives** – column layout helpers, margins, and other Quarto-specific attribute cascades.
3. **Extended shortcodes & publishing directives** – e.g., `{{< layout >}}`, conditional rendering helpers not part of upstream Pandoc.
4. **Execution metadata plumbing** – links between YAML front matter defaults and chunk-level overrides.
5. **Rich embedded language hooks** – Quarto's fenced cells that carry execution semantics beyond standard fenced blocks.

## Key Considerations

- Must maintain compatibility with base markdown syntax
- Should handle mixed content (markdown + code in multiple languages)
- Needs injection queries for embedded languages (R, Python, Julia, etc.)
- Must support incremental parsing for large documents

## Next Steps

1. **Create repository** under tree-sitter-grammars or as independent project
2. **Develop grammar** using tree-sitter-markdown as foundation
3. **Write comprehensive tests** covering all Quarto syntax
4. **Submit to tree-sitter-grammars** for official adoption
5. **Coordinate with Quarto team** for official support
6. **Update this extension** to use tree-sitter-quarto once ready

## Contributing

If you're interested in helping create `tree-sitter-quarto`, that would be a valuable contribution to the entire Quarto ecosystem across all editors. See the [Grammar Roadmap](grammar-roadmap.md) for context on how this fits into the overall development plan.
