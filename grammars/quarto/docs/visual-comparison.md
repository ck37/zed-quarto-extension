# Visual Comparison: AST Outputs

This document shows the difference in AST output between different parsers for the same Quarto Markdown input.

## Example Input

```qmd
See @fig-plot for results.

```{python}
#| label: fig-plot
#| echo: false
plot(data)
```

The mean is `{python} mean(x)`.
```

## tree-sitter-quarto (Our Implementation)

**Rich semantic nodes for editor features:**

```
(document
  (paragraph
    (text "See ")
    (cross_reference
      (reference_type "fig")
      (reference_id "plot"))
    (text " for results."))

  (executable_code_cell
    (cell_delimiter "```")
    (cell_language_specifier
      (language_name "python"))
    (chunk_options
      (chunk_option
        (chunk_option_key "label")
        (chunk_option_value "fig-plot"))
      (chunk_option
        (chunk_option_key "echo")
        (chunk_option_value "false")))
    (cell_content
      (python_code "plot(data)"))
    (cell_delimiter "```"))

  (paragraph
    (text "The mean is ")
    (inline_code_cell
      (language_name "python")
      (cell_content "mean(x)"))
    (text ".")))
```

**Editor Benefits:**
- ✅ Distinct colors for chunk option keys vs values
- ✅ Jump from `@fig-plot` reference to cell definition
- ✅ Autocomplete chunk option names
- ✅ Validate: "echo should be boolean"
- ✅ Fold executable cells
- ✅ Language injection for Python syntax in cells

## Quarto Markdown Parser (Rendering Pipeline)

**Pandoc AST for filter pipeline:**

```json
{
  "blocks": [
    {
      "t": "Para",
      "c": [
        {"t": "Str", "c": "See"},
        {"t": "Space"},
        {
          "t": "Cite",
          "c": [
            [{
              "citationId": "fig-plot",
              "citationPrefix": [],
              "citationSuffix": [],
              "citationMode": {"t": "AuthorInText"}
            }],
            [{"t": "Str", "c": "@fig-plot"}]
          ]
        },
        {"t": "Space"},
        {"t": "Str", "c": "for"},
        {"t": "Space"},
        {"t": "Str", "c": "results."}
      ]
    },
    {
      "t": "CodeBlock",
      "c": [
        ["fig-plot", ["python"], [["echo", "false"]]],
        "plot(data)"
      ]
    },
    {
      "t": "Para",
      "c": [
        {"t": "Str", "c": "The"},
        {"t": "Space"},
        {"t": "Str", "c": "mean"},
        {"t": "Space"},
        {"t": "Str", "c": "is"},
        {"t": "Space"},
        {"t": "Code", "c": [["", ["python"], []], "mean(x)"]},
        {"t": "Str", "c": "."}
      ]
    }
  ]
}
```

**Rendering Benefits:**
- ✅ Compatible with Pandoc filters
- ✅ Lua filters can transform `@fig-plot` citations
- ✅ Attributes available in standard Pandoc format
- ✅ Direct conversion to HTML/PDF/DOCX

**Editor Limitations:**
- ❌ No distinction between chunk options and code
- ❌ No semantic difference between xref and citation
- ❌ Can't jump from reference to definition
- ❌ No autocomplete for chunk options

## tree-sitter-pandoc-markdown (Base Grammar)

**Pandoc features without Quarto awareness:**

```
(document
  (paragraph
    (inline
      (text_base "See ")
      (cross_reference "@fig:plot")
      (text_base " for results.")))

  (fenced_code_block
    (fenced_code_block_delimiter)
    (info_string "python")
    (code_fence_line_text "#| label: fig-plot")
    (code_fence_line_text "#| echo: false")
    (code_fence_line_text "plot(data)")
    (fenced_code_block_delimiter))

  (paragraph
    (inline
      (text_base "The mean is ")
      (code_span
        (code_span_delimiter)
        (code_content "{python} mean(x)")
        (code_span_delimiter))
      (text_base "."))))
```

**Benefits:**
- ✅ Cross-references distinct from citations
- ✅ Works for general Pandoc Markdown

**Limitations for Quarto:**
- ❌ Chunk options treated as text, not structured data
- ❌ No semantic understanding of executable cells
- ❌ Inline code cells not recognized as executable

## Key Differences Summary

### Cross-References

| Feature | tree-sitter-quarto | Quarto Parser | tree-sitter-pandoc-md |
|---------|----------------|---------------|----------------------|
| `@fig-plot` | `(cross_reference type:fig id:plot)` | `(Cite citationId:"fig-plot")` | `(cross_reference)` |
| Semantic type | ✅ Distinct | ❌ Same as citation | ✅ Distinct |
| Jump-to-def | ✅ Yes | ❌ No | ✅ Yes (generic) |
| Type-aware | ✅ fig/tbl/eq | ❌ Generic | ❌ Generic |

### Chunk Options

| Feature | tree-sitter-quarto | Quarto Parser | tree-sitter-pandoc-md |
|---------|----------------|---------------|----------------------|
| `#\| label: foo` | `(chunk_option key:"label" value:"foo")` | `["label", "foo"]` | `(text)` |
| Structure | ✅ Rich AST | ⚠️  Array | ❌ Flat text |
| Validation | ✅ Possible | ⚠️  External | ❌ No |
| Autocomplete | ✅ Keys + values | ❌ No | ❌ No |
| Highlighting | ✅ Distinct colors | ❌ Generic | ❌ Generic |

### Executable Cells

| Feature | tree-sitter-quarto | Quarto Parser | tree-sitter-pandoc-md |
|---------|----------------|---------------|----------------------|
| Cell type | `(executable_code_cell)` | `(CodeBlock)` | `(fenced_code_block)` |
| Semantic | ✅ Executable | ⚠️  Generic | ❌ Static code |
| Language | ✅ `(language_name)` | ✅ Attributes | ✅ Info string |
| Injection | ✅ Per-language | ⚠️  Post-execution | ✅ Generic |

### Inline Code Cells

| Feature | tree-sitter-quarto | Quarto Parser | tree-sitter-pandoc-md |
|---------|----------------|---------------|----------------------|
| `` `{python} expr` `` | `(inline_code_cell)` | `(Code)` | `(code_span)` |
| Executable | ✅ Yes | ✅ Yes | ❌ Static |
| Language | ✅ Parsed | ✅ In attributes | ❌ In content |
| Highlighting | ✅ Language-aware | ❌ Generic | ❌ Generic |

## Use Case Comparison

### Authoring in Editor (Before Execution)

**Best:** tree-sitter-quarto
- See chunk options structure while typing
- Autocomplete and validation
- Jump between references and definitions
- Language injection in cells

**Usable:** tree-sitter-pandoc-markdown
- Basic syntax highlighting
- No Quarto-specific features

**Not Applicable:** Quarto Parser
- Parses post-execution output
- Not designed for authoring experience

### Rendering to Output Formats

**Best:** Quarto Parser
- Perfect Pandoc AST compatibility
- Works with existing filter pipeline
- Proven with quarto-web

**Not Applicable:** tree-sitter-quarto
- Editor-focused, not rendering-focused
- Would require AST → Pandoc conversion

**Not Applicable:** tree-sitter-pandoc-markdown
- No Pandoc AST output
- Editor-focused

### General Pandoc Markdown Editing

**Best:** tree-sitter-pandoc-markdown
- Covers all Pandoc features
- No Quarto-specific assumptions

**Overkill:** tree-sitter-quarto
- Assumes Quarto features
- Unnecessary for plain Pandoc

**Not Applicable:** Quarto Parser
- Quarto-specific design

## Visual Highlighting Example

### tree-sitter-quarto (Goal)

```qmd
See @fig-plot for results.
    ^^^^^^^^^ (cyan - cross-reference, clickable)

```{python}
^^^ (blue - cell delimiter)
  ^^^^^^ (green - language name)
#| label: fig-plot
^^ (gray - prefix)
   ^^^^^ (yellow - chunk option key)
          ^^^^^^^^ (orange - chunk option value)
#| echo: false
^^ (gray - prefix)
   ^^^^ (yellow - chunk option key)
         ^^^^^ (orange - chunk option value)
plot(data)
^^^^^^^^^^ (Python syntax highlighting - injected)
```
^^^ (blue - cell delimiter)

The mean is `{python} mean(x)`.
            ^^^^^^^^^ (green - language)
                      ^^^^^^^ (Python syntax)
```

### Quarto Parser (Pandoc AST - No Direct Highlighting)

Produces JSON/native format consumed by filters, not for editor highlighting.

### tree-sitter-pandoc-markdown (Current)

```qmd
See @fig-plot for results.
    ^^^^^^^^^ (cyan - cross-reference)

```{python}
^^^ (blue - delimiter)
   ^^^^^^ (green - language)
#| label: fig-plot
^^^^^^^^^^^^^^^^^^^ (gray - plain text, no structure)
#| echo: false
^^^^^^^^^^^^^^ (gray - plain text)
plot(data)
^^^^^^^^^^ (gray - plain text, no Python syntax)
```
^^^ (blue - delimiter)

The mean is `{python} mean(x)`.
               ^^^^^^^^^^^^^^^ (gray - monospace, no structure)
```

## Conclusion

Each parser excels in its domain:

- **tree-sitter-quarto:** Authoring experience in editors (our goal)
- **Quarto Parser:** Rendering pipeline compatibility
- **tree-sitter-pandoc-markdown:** General Pandoc Markdown editing

All three can coexist serving different needs in the Quarto ecosystem.

---

**Document Purpose:** Visualize what we're building and why it's different
**Last Updated:** 2025-10-13
