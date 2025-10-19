# GitHub Issue: Add Support for Pandoc Inline Formatting Extensions

**Repository**: https://github.com/ck37/tree-sitter-quarto/issues

---

## Title

Add support for Pandoc inline formatting extensions (strikethrough, highlight, subscript, superscript)

## Labels

`enhancement`, `parser`, `pandoc-markdown`

## Body

### Summary

tree-sitter-quarto currently supports basic Markdown inline formatting (bold, italic) but lacks support for several commonly-used Pandoc Markdown extensions for inline text formatting. These features are heavily used in scientific and academic writing with Quarto.

### Missing Features

The following Pandoc Markdown inline formatting features are not currently parsed:

1. **Strikethrough**: `~~text~~`
2. **Highlight/Mark**: `==text==`
3. **Subscript**: `H~2~O`
4. **Superscript**: `x^2^`
5. **Underline** (via attributes): `[text]{.underline}` (may already work via inline attributes?)

### Current Behavior

When parsing documents with these syntaxes, they are either:
- Treated as plain text (strikethrough, highlight)
- Produce parse errors (superscript with `^`)
- Not highlighted with semantic scopes

**Example test:**
```markdown
This is ~~strikethrough~~ text.
This is ==highlighted== text.
Chemical formula: H~2~O
Math notation: x^2^
```

**Current parse result:**
```
(paragraph
  content: (inline
    (text "This is ~~strikethrough~~ text.")))
(paragraph
  content: (inline
    (text "This is ==highlighted== text.")))
(paragraph
  content: (inline
    (text "Chemical formula: H~2~O")))
(ERROR ...)  ; x^2^ causes parse error
```

### Expected Behavior

These should parse as distinct node types with proper tree structure:

```
(paragraph
  content: (inline
    (text "This is ")
    (strikethrough
      delimiter: "~~"
      content: (text "strikethrough")
      delimiter: "~~")
    (text " text.")))

(paragraph
  content: (inline
    (text "This is ")
    (highlight
      delimiter: "=="
      content: (text "highlighted")
      delimiter: "==")
    (text " text.")))

(paragraph
  content: (inline
    (text "Chemical formula: H")
    (subscript
      delimiter: "~"
      content: (text "2")
      delimiter: "~")
    (text "O")))

(paragraph
  content: (inline
    (text "Math notation: x")
    (superscript
      delimiter: "^"
      content: (text "2")
      delimiter: "^")))
```

### Use Cases

**Scientific Writing:**
- Chemical formulas: H~2~O, CO~2~
- Mathematical notation: x^2^, E=mc^2^
- Highlighted important terms in methodology

**Academic Writing:**
- Strikethrough for tracking revisions
- Highlight for marking key findings
- Subscript/superscript for references and footnotes

**Real-world example** from our test fixtures:
```markdown
**Mathematical notation**: H~2~O, x^2^, ==highlighted text==, [underlined text]{.underline}
```

### Implementation Notes

**Pandoc Specification:**
These features are documented in the [Pandoc User's Guide](https://pandoc.org/MANUAL.html#strikeout):

- Strikethrough: Text between `~~` markers
- Subscript: Text between single `~` markers (not starting with whitespace)
- Superscript: Text between `^` markers (not starting with whitespace)
- Highlight: Extension feature, text between `==` markers (requires `+mark` extension)

**Challenges:**

1. **Subscript/Superscript ambiguity**: Single `~` and `^` have other meanings in Markdown
   - `~` is used for `~~strikethrough~~`
   - `^` is used for footnote references `[^1]`
   - Parser needs to disambiguate based on context (e.g., no whitespace after opening delimiter)

2. **Nesting**: These can be nested with other inline formatting:
   - `**bold ~~strikethrough~~ bold**`
   - `*italic ==highlight== italic*`

3. **External scanner**: May require external scanner tokens to properly detect these patterns without conflicts

### Proposed Node Types

```javascript
// In grammar.js
strikethrough: $ => seq('~~', $.inline, '~~'),
highlight: $ => seq('==', $.inline, '=='),
subscript: $ => seq('~', /[^\s~]+/, '~'),  // No whitespace after opening ~
superscript: $ => seq('^', /[^\s^]+/, '^'), // No whitespace after opening ^
```

### Proposed Highlight Queries

**Modern scopes** (queries/highlights.scm):
```scheme
(strikethrough) @markup.strikethrough
(highlight) @markup.mark
(subscript) @markup.subscript
(superscript) @markup.superscript
```

**Zed-compatible scopes** (queries/zed/highlights.scm):
```scheme
(strikethrough) @text.strike
(highlight) @text.highlight
(subscript) @text.subscript
(superscript) @text.super
```

### Test Cases Needed

Comprehensive corpus tests covering:

1. Basic usage
2. Nested formatting
3. Edge cases (adjacent to punctuation, whitespace handling)
4. Conflicts with other syntax (footnotes, strikethrough double-tilde)
5. Multi-word content
6. Unicode content

### References

- Pandoc User's Guide: https://pandoc.org/MANUAL.html#strikeout
- Quarto documentation mentions these features for scientific writing
- Used in zed-quarto-extension test fixtures: `tests/fixtures/longitudinal-clustering.qmd:271`

### Priority

**Medium-High**: These features are commonly used in scientific/academic Quarto documents. While not critical for basic functionality, their absence affects the editing experience for a significant portion of Quarto users.

### Related

- Inline attributes support was recently added (commit 28bb053), which provides a foundation for attribute-based formatting
- This would complete Pandoc inline formatting support

---

## Optional: Additional Context

If you'd like to discuss the implementation approach before I start work on this, I'm happy to coordinate. These features would significantly improve the grammar's coverage of Pandoc Markdown.

I can also provide sample documents from real-world Quarto usage that demonstrate these features if that would be helpful for testing.
