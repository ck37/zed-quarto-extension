# Grammar Feature Needs for Zed Quarto Extension

This document outlines missing features in `tree-sitter-pandoc-markdown` that would significantly improve syntax highlighting and language support for Quarto documents in Zed.

## Status Overview

### ✅ Implemented (Pandoc Markdown)
- Fenced divs with attributes (`:::`)
- Attribute lists (`{.class #id key=value}`)
- Citations (`@item`, `[@item p. 4]`)
- YAML front matter (`---` metadata blocks with YAML payload)
- Basic markdown (headings, lists, emphasis, links, code blocks)

### 🔴 Missing Features

The following features are part of Pandoc Markdown and essential for Quarto authoring but not yet implemented in the grammar.

---

## Critical Priority

### 1. Inline Math

**Syntax:**
```markdown
Inline math: $E = mc^2$

Display math:
$$
\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$
```

**Current State:** Dollar signs and LaTeX are treated as regular text characters.

**Impact:**
- Essential for scientific, technical, and academic documents
- Quarto renders math using MathJax/KaTeX
- Poor highlighting makes math hard to read and edit

**Required Node Types:**
```
inline_math     # $...$
display_math    # $$...$$
math_content    # The LaTeX content
```

**Highlighting Needs:**
- Math content should be highlighted with LaTeX syntax or marked distinctly
- Delimiters ($, $$) should be punctuation

**Example Usage:**
```scheme
; In highlights.scm
[(inline_math) (display_math)] @string.special
(math_content) @text.math

; Optionally inject LaTeX
((math_content) @injection.content
 (#set! injection.language "latex"))
```

---

### 2. Tables (Pipe Tables)

**Syntax:**
```markdown
| Header 1    | Header 2    | Header 3    |
|-------------|-------------|-------------|
| Row 1 Col 1 | Row 1 Col 2 | Row 1 Col 3 |
| Row 2 Col 1 | Row 2 Col 2 | Row 2 Col 3 |

With alignment:
| Left | Center | Right |
|:-----|:------:|------:|
| A    | B      | C     |
```

**Current State:** Parsed as paragraph text with pipes.

**Impact:**
- Tables are fundamental for data presentation
- Quarto supports multiple table formats (pipe, grid, simple)
- Pipe tables are most common in markdown

**Required Node Types:**
```
pipe_table           # The entire table
pipe_table_header    # First row
pipe_table_delimiter # The |---|---| row
pipe_table_row       # Data rows
pipe_table_cell      # Individual cells
pipe_table_alignment # :--- or ---: or :---:
```

**Highlighting Needs:**
- Headers should be distinct (bold or different color)
- Pipes should be punctuation
- Alignment markers should be visible

**Example Usage:**
```scheme
; In highlights.scm
(pipe_table_header) @markup.heading
(pipe_table_cell) @none
"|" @punctuation.delimiter
(pipe_table_alignment) @punctuation.special
```

---

### 3. Footnotes

**Syntax:**
```markdown
Here's a sentence with a footnote[^1].

Another with inline footnote^[This is inline].

[^1]: This is the footnote content.
      It can span multiple paragraphs.
```

**Current State:** Footnote markers are parsed as regular text with brackets.

**Impact:**
- Standard feature in academic and technical writing
- Quarto renders footnotes with automatic numbering
- Important for citations and clarifications

**Required Node Types:**
```
footnote_reference      # [^1] or ^[inline]
footnote_definition     # [^1]: content
inline_footnote         # ^[content]
```

**Highlighting Needs:**
- Footnote references should stand out as links/references
- Footnote definitions should be marked similarly to list items

**Example Usage:**
```scheme
; In highlights.scm
(footnote_reference) @text.reference
(footnote_definition) @markup.list
(inline_footnote) @text.reference
```

---

## High Priority

### 4. Definition Lists

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

### 5. Strikethrough

**Syntax:**
```markdown
~~This text is deleted~~
```

**Current State:** Tildes and text parsed separately.

**Impact:**
- Common text decoration for tracked changes
- Part of GFM (GitHub Flavored Markdown) adopted by Pandoc

**Required Node Types:**
```
strikethrough
```

**Highlighting Needs:**
```scheme
(strikethrough) @markup.strikethrough
```

---

### 6. Subscript and Superscript

**Syntax:**
```markdown
H~2~O for water
E = mc^2^ for energy
```

**Current State:** Tildes/carets and text parsed separately.

**Impact:**
- Essential for scientific notation
- Chemical formulas, mathematical notation

**Required Node Types:**
```
subscript
superscript
```

**Highlighting Needs:**
```scheme
(subscript) @text.subscript
(superscript) @text.superscript
```

---

## Medium Priority

### 7. Spans with Attributes

**Syntax:**
```markdown
[This text has a class]{.important}
[Text with ID]{#myid}
[Multiple attributes]{.class1 .class2 #id key="value"}
```

**Current State:** Brackets and braces parsed separately.

**Impact:**
- Allows inline styling and custom HTML/CSS classes
- Essential for fine-grained document control
- Quarto uses this for conditional content

**Required Node Types:**
```
span
span_content
span_attributes  # Reuse attribute_list
```

**Highlighting Needs:**
```scheme
(span) @markup.inline
(span_attributes) @attribute
```

---

### 8. Raw Inline and Raw Blocks

**Syntax:**
```markdown
Inline HTML: `<div class="special">`{=html}
Inline LaTeX: `\newcommand{\R}{\mathbb{R}}`{=latex}

```{=html}
<div class="custom">
  <p>HTML block</p>
</div>
```

```{=latex}
\begin{equation}
  E = mc^2
\end{equation}
```
```

**Current State:** Parsed as code blocks/spans without format awareness.

**Impact:**
- Allows format-specific output
- Essential for custom HTML/LaTeX in documents
- Quarto uses this for cross-format compatibility

**Required Node Types:**
```
raw_inline
raw_block
raw_format  # html, latex, etc.
```

**Highlighting Needs:**
```scheme
(raw_inline) @markup.raw.inline
(raw_block) @markup.raw.block
(raw_format) @keyword

; Inject appropriate language
((raw_block
  (raw_format) @format
  (code_content) @injection.content)
 (#match? @format "html")
 (#set! injection.language "html"))
```

---

### 9. Line Blocks

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

### 10. Grid Tables

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

### 11. Simple Tables

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

---

### 12. Metadata Blocks (Percent Syntax)

**Syntax:**
```markdown
% Title
% Author1; Author2
% Date
```

**Impact:** Alternative to YAML frontmatter, less common in Quarto.

---

## Implementation Recommendations

### Phase 1D (Immediate Next Steps)
1. **Inline/Display Math** - Core feature for scientific documents
2. **Pipe Tables** - Essential for data presentation
3. **Footnotes** - Academic standard

### Phase 1E (Follow-up)
4. **Definition Lists** - Documentation feature
5. **Strikethrough** - Common text decoration
6. **Subscript/Superscript** - Scientific notation

## Quarto-Specific Features

### Currently Supported
- Cross-references (`@fig:name`, `@tbl:data`) via Quarto crossref support
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
- [Phase 1 Grammar Plan](https://github.com/ck37/tree-sitter-pandoc-markdown/blob/feat/phase-1-pandoc-grammar/plan.md)

---

## Notes

- Features are ordered by **impact on Quarto authoring experience** rather than parsing complexity
- Critical, high, and medium priority sections focus on standard Pandoc Markdown extensions
- This ensures the grammar remains useful for all Pandoc markdown users, while Quarto-only features are tracked separately
- Quarto-specific features may ultimately be implemented in a future `tree-sitter-quarto` grammar or via downstream tooling
- Current grammar baseline: `ck37/tree-sitter-pandoc-markdown@e602eb65` (adds YAML front matter support)
