# tree-sitter-quarto Implementation

This document describes the implementation of the dedicated `tree-sitter-quarto` grammar.

## Status: ✅ Complete (October 2025)

The extension now uses [`tree-sitter-quarto`](https://github.com/ck37/tree-sitter-quarto) for comprehensive Quarto syntax highlighting.

## Overview

With the Pandoc grammar enriched (Phase 1 complete), we created a dedicated `tree-sitter-quarto` that:

- ✅ Provides first-class support for Quarto syntax
- ✅ Enables proper highlighting of Quarto-only features (chunk options, execution directives)
- ✅ Can be adopted by other editors (Neovim, Helix, etc.)
- 📋 Future: Potentially lives in the official `tree-sitter-grammars` organization

## Implementation Approach

### 1. Grammar Repository

Created at: https://github.com/ck37/tree-sitter-quarto

### 2. Extends tree-sitter-markdown

The grammar inherits from tree-sitter-markdown and extends it:

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

### 3. Quarto Syntax Support

✅ Implemented:
- YAML frontmatter with Quarto-specific keys
- Div blocks with attributes `:::{.class #id}`
- Callouts `:::{.callout-note}`, `:::{.callout-warning}`
- Shortcodes `{{< include file.qmd >}}`
- Code chunks with Quarto execution options (`#| key: value`)
- Cross-references `@fig-plot`, `@tbl-data`
- Citations `@smith2024`
- Inline code cells
- Bold, italic, and other inline formatting

### 4. Test Suite

✅ Comprehensive test suite covering all Quarto syntax constructs (58/58 tests passing)

### 5. Extension Integration

✅ This extension now uses `tree-sitter-quarto` (migrated in commit 1877b3a)

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

## Implemented Quarto-Specific Syntax

The dedicated grammar supports Quarto-only constructs:

1. ✅ **Executable chunk option lines** – `#| echo: false`, `#| warning: false`, multi-line option blocks
2. ✅ **Quarto shortcodes** – `{{< include file.qmd >}}`, `{{< pagebreak >}}`, etc.
3. ✅ **Cross-references** – `@fig-plot`, `@tbl-data`
4. ✅ **Inline code cells** – `` `{r} code` ``
5. ✅ **Rich embedded language hooks** – Quarto's fenced cells with execution semantics

## Key Features

- ✅ Maintains compatibility with base markdown syntax
- ✅ Handles mixed content (markdown + code in multiple languages)
- ✅ Injection queries for embedded languages (R, Python, Julia, SQL, JavaScript, TypeScript, Bash, etc.)
- ✅ Supports incremental parsing for large documents
- ✅ Uses Zed-compatible scopes for theme support

## Future Work

1. 📋 **Publish to npm** for wider adoption
2. 📋 **Submit to tree-sitter-grammars** for official adoption
3. 📋 **Coordinate with Quarto team** for official support
4. 📋 **Additional Pandoc features** – See [`grammar-feature-needs.md`](grammar-feature-needs.md)

## Contributing

If you're interested in helping improve `tree-sitter-quarto`, that would be a valuable contribution to the entire Quarto ecosystem across all editors. See the [Grammar Roadmap](grammar-roadmap.md) for context.
