# Reference Documentation Summary

This document contains relevant excerpts from tree-sitter, Quarto, and Zed documentation for developing tree-sitter-quarto.

## Tree-sitter Grammar Development

### Project Initialization

Initialize a new tree-sitter parser project:
```sh
tree-sitter init
```

This prompts for necessary information and generates essential project files including `grammar.js`.

### Grammar File Structure

Basic structure of a tree-sitter grammar file:

```js
/**
 * @file Parser description
 * @author Author name
 * @license License
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'parser_name',

  rules: {
    source_file: $ => 'hello'
  }
});
```

### Grammar Development Strategy

**Breadth-first approach** - Start with broad structure, add details later:

```javascript
{
  rules: {
    source_file: $ => repeat($._definition),

    _definition: $ => choice(
      $.function_definition
      // TODO: other kinds of definitions
    ),

    function_definition: $ => seq(
      'func',
      $.identifier,
      $.parameter_list,
      $._type,
      $.block
    ),

    // ... more rules with TODOs for expansion
  }
}
```

### Generating and Testing Parsers

```bash
# Generate parser from grammar
npx tree-sitter generate

# Run tests
npx tree-sitter test

# Parse a file
npx tree-sitter parse examples/sample.qmd

# Debug parse tree
npx tree-sitter parse examples/sample.qmd --debug
```

### Query Files (Syntax Highlighting)

Tree-sitter uses `.scm` files (Scheme-like syntax) for queries:

```scheme
(query (identifier) @function.call)
(function_declaration name: (identifier) @function.name)
```

Query files are used for:
- `highlights.scm` - Syntax highlighting
- `injections.scm` - Language injection
- `locals.scm` - Local variable identification
- `folds.scm` - Code folding

### Language Configuration (tree-sitter.json)

```json
{
  "grammars": [
    {
      "scope": "source.js",
      "path": ".",
      "file-types": ["js", "mjs", "cjs"],
      "first-line-regex": "^#!/usr/bin/env node",
      "content-regex": "(?i)^\\s*(\\/\\*|\\/\\/|#|\\{\\})",
      "injection-regex": "(?s)^\\s*(javascript|js|\\{\\})",
      "highlights": "queries/highlights.scm",
      "locals": "queries/locals.scm",
      "injections": "queries/injections.scm"
    }
  ]
}
```

## Quarto Markdown Features

### Executable Code Cells

Basic syntax for executable code cells:

````markdown
```{python}
#| label: fig-plot
#| echo: false
#| fig-cap: "Sample plot"
import matplotlib.pyplot as plt
plt.plot([1, 2, 3])
```
````

### Chunk Options

Chunk options use `#|` prefix at the start of code cells:

```python
#| label: fig-polar
#| echo: false
#| fig-cap: "A line plot on a polar axis"
#| fig-width: 8
#| fig-height: 6
```

**Multi-line chunk options:**

```python
#| fig-cap: |
#|   This is a multi-line caption
#|   that spans multiple lines
```

### Cell Execution Options

Key execution options:
- `echo` - Include source code in output (true/false/fenced)
- `eval` - Evaluate code cells (true/false)
- `output` - Include results (true/false/asis)
- `warning` - Include warnings (true/false)
- `error` - Include errors (true/false)
- `include` - Catch-all for preventing output
- `cache` - Cache results (true/false/refresh)
- `freeze` - Control re-use of computational output (true/false/auto)

### Cross-References

Cross-references use `@` prefix with type identifier:

```markdown
See @fig-plot for the visualization.
Refer to @tbl-summary for statistics.
As shown in @eq-model, the relationship is linear.
```

**Cross-reference types:**
- `@fig-` - Figures
- `@tbl-` - Tables
- `@eq-` - Equations
- `@sec-` - Sections
- `@lst-` - Code listings

**Citations** (no type prefix):
```markdown
According to @knuth1984, literate programming is important.
```

### Inline Code Cells

Inline executable code:

```markdown
The radius is `{python} radius`
The mean is `{r} mean(c(1,2,3))`
```

### Callouts

Five callout types with standard syntax:

```markdown
::: {.callout-note}
Note that there are five types of callouts.
:::

::: {.callout-tip}
## Tip with Title
This is a callout with a title.
:::

::: {.callout-warning}
Warning content here.
:::

::: {.callout-important}
Important information.
:::

::: {.callout-caution collapse="true"}
## Collapsible Caution
This can be expanded by the user.
:::
```

### Divs for Cross-References

Create cross-referenceable content using divs:

````markdown
::: {#fig-code}
```{r}
library(tidyverse)
starwars |> ggplot(aes(height, mass)) + geom_point()
```
A code cell treated like a figure.
:::
````

### Raw Markdown Output

Generate raw markdown from code (bypassing standard div wrappers):

```python
```{python}
#| echo: false
#| output: asis
print("# Heading 1\n")
```
```

```r
```{r}
#| echo: false
#| output: asis
cat("# Heading 1\n")
```
```

### Tabsets

Panel tabsets for organizing content:

```markdown
::: {.panel-tabset}
## Tab 1
Content 1

## Tab 2
Content 2
:::
```

### Conditional Content

Content visible/hidden based on format:

```markdown
::: {.content-visible when-format="html"}
HTML-only content
:::

::: {.content-hidden when-format="pdf"}
Hidden in PDF output
:::
```

## Zed Editor Integration

### Grammar Registration (extension.toml)

Register a tree-sitter grammar in Zed extension:

```toml
[grammars.language_name]
repository = "https://github.com/org/tree-sitter-lang"
rev = "commit_sha"
```

For local development:
```toml
[grammars.language_name]
repository = "file:///path/to/grammar"
```

### Language Configuration (languages/language_name/config.toml)

Define language metadata:

```toml
name = "My Language"
grammar = "my-language"
path_suffixes = ["myl"]
line_comments = ["# "]
tab_size = 4
hard_tabs = false
# first_line_pattern = "^#!/usr/bin/env mylang"
```

### Syntax Highlighting (queries/highlights.scm)

Define syntax highlighting using tree-sitter queries:

```scheme
(string) @string

(pair
  key: (string) @property.json_key)

(number) @number
```

### Bracket Matching (queries/brackets.scm)

Define bracket pairs for navigation:

```scheme
("[" @open "]" @close)
("{" @open "}" @close)
("\"" @open "\"" @close)
```

### Language Server Configuration

Configure language server in extension.toml:

```toml
[language_servers.my-language-server]
name = "My Language LSP"
languages = ["My Language"]
```

Map multiple languages to one language server:

```toml
[language-servers.my-language-server]
name = "Whatever LSP"
languages = ["JavaScript", "HTML", "CSS"]

[language-servers.my-language-server.language_ids]
"JavaScript" = "javascript"
"HTML" = "html"
"CSS" = "css"
```

### File Type Associations

Configure file type associations in settings.json:

```json
{
  "file_types": {
    "Language Name": [
      "*.ext",
      "**/path/pattern/*.yaml"
    ]
  }
}
```

### Syntax Theme Overrides

Customize syntax highlighting in settings:

```json
{
  "experimental.theme_overrides": {
    "syntax": {
      "comment": {
        "font_style": "italic"
      },
      "string": {
        "color": "#00AA00"
      }
    }
  }
}
```

## Key Insights for tree-sitter-quarto

### Grammar Design

1. **Use external scanner for complex tokens** - Cell boundaries and chunk option detection may need custom scanner logic
2. **Language injection** - Use injection queries to enable syntax highlighting for multiple languages (Python, R, Julia) within one document
3. **Semantic node types** - Create distinct node types for:
   - `executable_code_cell` vs regular code blocks
   - `chunk_option` with separate `key` and `value` fields
   - `cross_reference` vs `citation`
   - Enhanced `callout_block` types

### Query Development

1. **Highlights query priorities:**
   - Chunk option keys and values
   - Language specifiers
   - Cross-reference types vs citation IDs
   - Callout types

2. **Injection query pattern:**
```scheme
((executable_code_cell
  (language_name) @_lang
  (#eq? @_lang "python")
  (cell_content) @injection.content)
 (#set! injection.language "python"))
```

### Editor Integration

1. **Zed extension structure:**
   - `extension.toml` - Manifest with grammar reference
   - `grammars/` - Tree-sitter grammar repository reference
   - `languages/qmd/` - Language configuration
   - `languages/qmd/queries/` - Highlighting, injection, folding queries

2. **Testing in Zed:**
   - Use local grammar via `file://` URL during development
   - Test syntax highlighting with actual .qmd files
   - Verify language injection for embedded code

### Quarto-Specific Parsing Challenges

1. **Chunk options:** Must distinguish `#|` at start of cell from regular comments
2. **Cross-references:** Pattern matching `@type-id` vs `@id` (citation)
3. **Inline code cells:** Parse `` `{lang} expr` `` differently from `` `code` ``
4. **Raw blocks:** Handle `{=format}` syntax
5. **Multi-line values:** Support `|` continuation in chunk options

## References

- Tree-sitter Documentation: https://tree-sitter.github.io/tree-sitter/
- Quarto Documentation: https://quarto.org/docs/
- Zed Extensions Guide: https://zed.dev/docs/extensions
- Tree-sitter Query Syntax: https://tree-sitter.github.io/tree-sitter/syntax-highlighting
