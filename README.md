# Quarto extension for Zed

Quarto brings literate programming to Zed with first-class syntax highlighting and language server support for `.qmd` files. This extension uses `tree-sitter-pandoc-markdown` for syntax highlighting and launches the official `quarto language-server` for IDE features.

## Installation

1. Install the [Quarto CLI](https://quarto.org/docs/get-started/) so that `quarto` is available on your `PATH`.
2. Clone this repository locally.
3. In Zed, open the command palette and run `zed: install dev extension`, then select this repository directory.

Zed will automatically compile the extension and its grammars.

## Configuration

Add optional settings to `settings.json` when a custom CLI path or flags are required:

```json
"quarto": {
  "server_path": "/custom/path/to/quarto",
  "additional_args": ["--log-level", "info"]
}
```

## Testing

Run the automated checks with:

```bash
cargo test --workspace --all-features
```

`tests/highlights.rs` validates highlighting coverage, while `tests/lsp_smoke.rs` ensures the Quarto CLI can be discovered on the current system.

## Known Limitations

- **Preview/render workflows** are out of scope for this extension—use the Quarto CLI or VSCode extension for visual editing and preview.
- **Grammar completeness**: `tree-sitter-pandoc-markdown` is a community project that extends `tree-sitter-markdown`. Some edge cases in Pandoc syntax may not be fully supported yet.
- **No official tree-sitter-quarto**: Until an official Quarto grammar exists, we rely on Pandoc markdown as the closest approximation.
- **Limited Quarto-specific syntax highlighting**: The extension provides standard Pandoc markdown highlighting (headings, links, code blocks, lists, citations, emphasis), YAML frontmatter, and language injections for Python, R, Julia, and SQL code chunks. The following Quarto-specific features are **not** currently highlighted distinctly:
  - **Callouts**: `:::{.callout-note}`, `:::{.callout-warning}`, etc. (displayed as plain text)
  - **Shortcodes**: `{{< include file.qmd >}}`, `{{< video url >}}`, etc. (not highlighted as special syntax)
  - **Div blocks with attributes**: `:::{.column-margin}`, `:::{#fig-plot}` (not distinguished from regular text)
  - **Cross-references**: `@fig-plot`, `@tbl-data`, `@eq-equation` (not highlighted as references)
  - **Code chunk execution options**: `#| echo: false`, `#| warning: false` (treated as comments)

## Design Notes

### Why tree-sitter-pandoc-markdown?

Quarto documents (`.qmd`) are **not** plain Markdown—they're based on [Pandoc's Markdown](https://pandoc.org/MANUAL.html#pandocs-markdown), which includes additional syntax:

- **Div blocks**: `:::` with attributes like `{.class #id}`
- **Citations**: `@citation` references
- **Shortcodes**: `{{< shortcode >}}` syntax
- **Fenced divs**: Extended YAML frontmatter
- **Code chunks**: Special attributes beyond standard fenced code blocks

### TextMate vs Tree-sitter

The [official Quarto VSCode extension](https://github.com/quarto-dev/quarto/tree/main/apps/vscode) uses **TextMate grammars** (`.tmLanguage` files), which are regex-based pattern matching systems. However, **Zed only supports tree-sitter grammars**, which are proper parsers.

Since there is no official `tree-sitter-quarto` grammar yet, we use [`tree-sitter-pandoc-markdown`](https://github.com/jmbuhr/tree-sitter-pandoc-markdown), which extends `tree-sitter-markdown` with Pandoc-specific features that Quarto is built upon.

This approach provides:
- Proper parsing of Quarto/Pandoc syntax constructs
- Accurate syntax highlighting for citations, divs, shortcodes
- Better error recovery than regex-based approaches
- Foundation for future code navigation features

## Syntax Highlighting Technical Details

### Why Different Editors Have Different Approaches

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

### Current State

Our extension currently provides **basic Pandoc markdown highlighting** through `tree-sitter-pandoc-markdown` as a temporary solution. This covers core markdown syntax but lacks Quarto-specific features because the underlying grammar doesn't define them as separate node types.

**This is temporary.** The proper solution is to create a dedicated `tree-sitter-quarto` grammar.

### Planned: tree-sitter-quarto Grammar

We plan to create an official `tree-sitter-quarto` grammar rather than extending `tree-sitter-pandoc-markdown`. This will:

- Provide first-class support for all Quarto syntax
- Enable proper highlighting of Quarto-specific features
- Allow the grammar to be adopted by other editors (Neovim, Helix, etc.)
- Potentially be hosted in the official `tree-sitter-grammars` organization
- Be maintained as an official Quarto project

**Why not extend tree-sitter-pandoc-markdown?** Quarto has enough unique syntax (callouts, shortcodes, execution options, cross-references) that a dedicated grammar is warranted. This also allows for cleaner separation and official support from the Quarto team.

#### Implementation Plan

1. **Initialize grammar repository**:
   ```bash
   npm install -g tree-sitter-cli
   tree-sitter init quarto
   ```

2. **Inherit from tree-sitter-markdown**: Add as git submodule and extend
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

3. **Define all Quarto syntax**:
   - YAML frontmatter with Quarto-specific keys
   - Div blocks with attributes `:::{.class #id}`
   - Callouts `:::{.callout-note}`, `:::{.callout-warning}`
   - Shortcodes `{{< include file.qmd >}}`
   - Code chunks with Quarto execution options
   - Cross-references `@fig-plot`, `@tbl-data`
   - Citations `@smith2024`

4. **Create comprehensive test suite** covering all Quarto syntax

5. **Publish to npm** and propose to Quarto team for official adoption

6. **Update this extension** to use `tree-sitter-quarto` instead of `tree-sitter-pandoc-markdown`

**Key considerations:**
- Must maintain compatibility with base markdown syntax
- Should handle mixed content (markdown + code in multiple languages)
- Needs injection queries for embedded languages (R, Python, Julia, etc.)
- Must support incremental parsing for large documents

**Repository structure:**
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

#### Quarto-Specific Syntax to Implement

The grammar needs to handle these Quarto-specific features:

1. **Callout syntax** - `:::{.callout-note}`, `:::{.callout-warning}`, etc.
2. **Shortcodes** - `{{< include file.qmd >}}`, `{{< video url >}}`
3. **Div blocks with attributes** - `:::{.column-margin}`, `:::{#fig-plot}`
4. **Cross-references** - `@fig-plot`, `@tbl-data`, `@eq-equation`
5. **Code chunk execution options** - `#| echo: false`, `#| warning: false`
6. **YAML frontmatter** - Already supported via injection, but could be native
7. **Embedded code execution** - R, Python, Julia chunks with special Quarto attributes

#### Next Steps

1. **Create repository** under tree-sitter-grammars or as independent project
2. **Develop grammar** using tree-sitter-markdown as foundation
3. **Write comprehensive tests** covering all Quarto syntax
4. **Submit to tree-sitter-grammars** for official adoption
5. **Coordinate with Quarto team** for official support
6. **Update this extension** to use tree-sitter-quarto once ready

## Contributing

This extension currently uses `tree-sitter-pandoc-markdown` as a temporary solution. If you're interested in helping create `tree-sitter-quarto`, that would be a valuable contribution to the entire Quarto ecosystem across all editors.
