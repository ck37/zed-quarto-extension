# Grammar Feature Needs for Zed Quarto Extension

This document outlines missing features in `tree-sitter-pandoc-markdown` that would significantly improve syntax highlighting and language support for Quarto documents in Zed.

## Status Overview

### ✅ Implemented (Pandoc Markdown)
- Fenced divs with attributes (`:::`)
- Attribute lists (`{.class #id key=value}`)
- Attribute spans (`[text]{.class}`)
- Citations (`@item`, `[@item p. 4]`)
- Cross-references (`@fig-plot`, `@tbl-data`)
- YAML front matter (`---` metadata blocks with YAML payload)
- Basic markdown (headings, lists, links, code blocks)
- **Inline formatting** (emphasis, strong, strikethrough, highlight, subscript, superscript, underline)
  - ⚠️ Note: Bold/italic highlighting partially working (~70% coverage) in Zed using built-in markdown-inline injection workaround (see `docs/bold-highlighting-investigation/` and README)
- **Math** - Inline (`$...$`) and display (`$$...$$`) with LaTeX content
- **Pipe tables** - Headers, cells, alignment markers
- **Footnotes** - References (`[^1]`) and inline footnotes (`^[text]`)
- **Raw blocks and inline** - Format-specific output (`{=html}`, `{=latex}`)
- **Shortcodes** (`{{< include file.qmd >}}`)
- **Chunk options** (`#| echo: false`)
- **Percent metadata blocks** - Alternative to YAML frontmatter (`% Title`, `% Author`, `% Date`)

### 🔴 Missing Features

The following features are part of Pandoc Markdown but not yet implemented in the grammar.

---

## High Priority

### 1. Definition Lists

**Syntax:**
```markdown
Term 1
:   Definition 1

Term 2
:   Definition 2a with multiple paragraphs

    Second paragraph of definition 2a
:   Definition 2b (alternative definition)
```

**Current State:** Parsed as paragraphs with colons.

**Impact:**
- Useful for glossaries, API documentation, terminology
- Common in technical documentation
- Pandoc-specific feature

**Required Node Types:**
```
definition_list
definition_term
definition_description
```

**Highlighting Needs:**
```scheme
(definition_term) @markup.heading
(definition_description) @markup.list
```

---

## Medium Priority

### 2. Line Blocks

**Syntax:**
```markdown
| The limerick packs laughs anatomical
| Into space that is quite economical.
|    But the good ones I've seen
|    So seldom are clean
| And the clean ones so seldom are comical
```

**Current State:** Pipes and text parsed as regular paragraph content.

**Impact:**
- Preserves line breaks for poetry, addresses, verses
- Different from code blocks (rendered with normal formatting)
- Important for creative/literary content

**Required Node Types:**
```
line_block
line_block_line
```

**Highlighting Needs:**
```scheme
(line_block) @markup.quote
(line_block_line) @none
"|" @punctuation.special
```

---

## Lower Priority

### 3. Grid Tables

**Syntax:**
```markdown
+---------------+---------------+--------------------+
| Header 1      | Header 2      | Header 3           |
+===============+===============+====================+
| Row 1 Cell 1  | Row 1 Cell 2  | Row 1 Cell 3       |
+---------------+---------------+--------------------+
| Row 2 Cell 1  | Row 2 Cell 2  | Row 2 Cell 3       |
+---------------+---------------+--------------------+
```

**Impact:** More complex than pipe tables but supports multi-line cells.

---

### 4. Simple Tables

**Syntax:**
```markdown
  Right     Left     Center     Default
-------     ------ ----------   -------
     12     12        12            12
    123     123       123          123
      1     1          1             1

Table: Demonstration of simple table syntax.
```

**Impact:** Simpler syntax than pipe tables for basic tables.


## Implementation Recommendations

### Phase 2 (Next Steps)

Most Pandoc features critical for Quarto are now implemented. Remaining features:

1. **Definition Lists** - Useful for glossaries and documentation
2. **Line Blocks** - For poetry and addresses (preserves line breaks)
3. **Grid/Simple Tables** - Alternative table formats (pipe tables already supported)

## Quarto-Specific Features

### Currently Supported
- Cross-references (`@fig-plot`, `@tbl-data`) via Quarto crossref support
- Shortcodes (`{{< include file.qmd >}}`)
- Chunk options (`#| echo: false`)

### Future Enhancements
- Enhanced code chunk attributes (beyond `#|` options)
- Quarto-specific shortcodes (beyond generic `{{< >}}`)
- Cross-format conditional content
- Diagram blocks (mermaid, graphviz)

---

## Testing Strategy

For each new feature:

1. **Add corpus tests** showing valid syntax variations
2. **Create highlight queries** for the new node types
3. **Update injection queries** if the feature contains other languages (YAML, LaTeX, HTML)
4. **Add test fixtures** in `tests/fixtures/` demonstrating real-world usage
5. **Update extension README** to document newly supported features

---

## References

- [Pandoc Manual - Pandoc's Markdown](https://pandoc.org/MANUAL.html#pandocs-markdown)
- [Quarto Documentation](https://quarto.org/docs/guide/)
- [Tree-sitter Grammar Development](https://tree-sitter.github.io/tree-sitter/creating-parsers)
- [tree-sitter-pandoc-markdown Repository](https://github.com/ck37/tree-sitter-pandoc-markdown)

---

## Notes

- Features are ordered by **impact on Quarto authoring experience** rather than parsing complexity
- Most critical Pandoc Markdown features are now implemented (Phase 1 complete)
- Remaining features are lower priority alternatives to already-supported syntax
- Quarto-specific features may ultimately be implemented in a future `tree-sitter-quarto` grammar or via downstream tooling
- Current grammar version: `ck37/tree-sitter-pandoc-markdown@581a8279` (Phase 1 completion + pipe tables)
- **Known limitation**: Bold/italic highlighting partially working (~70% coverage) using built-in markdown-inline injection workaround; custom-to-custom grammar injection not yet supported in Zed extensions (see `docs/bold-highlighting-investigation/` and README)
