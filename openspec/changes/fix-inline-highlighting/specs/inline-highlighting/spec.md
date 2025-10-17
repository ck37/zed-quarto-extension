# Spec: Inline Content Highlighting

## Overview

Define requirements for properly highlighting inline text content within emphasis, strong emphasis, headings, and Pandoc inline formatting extensions in Zed editor.

## ADDED Requirements

### Requirement: Emphasis Content Highlighting
**ID:** inline-highlight-emphasis

Text inside single-asterisk or single-underscore emphasis markers must be visually styled to indicate emphasis (typically italic).

#### Scenario: Single-asterisk emphasis
```markdown
*italic text*
```
- GIVEN a Quarto document with `*italic text*`
- WHEN the file is opened in Zed with the quarto extension
- THEN the text "italic text" (without markers) is highlighted with `@text.emphasis` styling
- AND the `*` markers are highlighted with `@punctuation.delimiter` styling

#### Scenario: Single-underscore emphasis
```markdown
_italic text_
```
- GIVEN a Quarto document with `_italic text_`
- WHEN the file is opened in Zed
- THEN the text "italic text" is highlighted with `@text.emphasis` styling

#### Scenario: Emphasis with nested inline elements
```markdown
*italic with `code` inside*
```
- GIVEN emphasis containing other inline elements like code spans
- WHEN rendered in Zed
- THEN the emphasis styling applies to text portions
- AND nested elements (like `code`) maintain their own styling

---

### Requirement: Strong Emphasis Content Highlighting
**ID:** inline-highlight-strong

Text inside double-asterisk or double-underscore strong emphasis markers must be visually styled to indicate strong emphasis (typically bold).

#### Scenario: Double-asterisk strong emphasis
```markdown
**bold text**
```
- GIVEN a Quarto document with `**bold text**`
- WHEN the file is opened in Zed
- THEN the text "bold text" is highlighted with `@emphasis.strong` styling
- AND the `**` markers are highlighted with `@punctuation.delimiter` styling

#### Scenario: Double-underscore strong emphasis
```markdown
__bold text__
```
- GIVEN a Quarto document with `__bold text__`
- WHEN the file is opened in Zed
- THEN the text "bold text" is highlighted with `@emphasis.strong` styling

---

### Requirement: Combined Emphasis Content Highlighting
**ID:** inline-highlight-combined

Text inside triple-asterisk markers (bold + italic combination) must be styled appropriately.

#### Scenario: Triple-asterisk combined emphasis
```markdown
***bold italic text***
```
- GIVEN a Quarto document with `***bold italic text***`
- WHEN the file is opened in Zed
- THEN the text "bold italic text" receives combined styling
- OR the text is highlighted with both `@text.emphasis` and `@emphasis.strong` scopes
- AND the grammar correctly parses this as nested emphasis nodes

---

### Requirement: Heading Content Highlighting
**ID:** inline-highlight-heading

Text content inside ATX headings (after the `#` markers) must be styled distinctly as heading text.

#### Scenario: ATX heading level 1
```markdown
# This is a heading
```
- GIVEN a Quarto document with `# This is a heading`
- WHEN the file is opened in Zed
- THEN "This is a heading" is highlighted with `@text.title` styling
- AND the `#` marker is highlighted with `@punctuation.special` styling

#### Scenario: ATX heading with multiple words
```markdown
## Multi-word heading with spaces
```
- GIVEN a heading with multiple words and special characters
- WHEN rendered in Zed
- THEN all text content after the markers receives `@text.title` styling

#### Scenario: Setext heading
```markdown
Heading Text
============
```
- GIVEN a setext-style heading with underline
- WHEN the file is opened in Zed
- THEN "Heading Text" is highlighted with `@text.title` styling
- AND the `====` underline is highlighted with `@punctuation.special` styling

---

### Requirement: Pandoc Inline Formatting Extensions
**ID:** inline-highlight-pandoc-extensions

Text inside Pandoc inline formatting extensions (strikethrough, highlight, subscript, superscript) must be styled appropriately.

#### Scenario: Strikethrough text
```markdown
~~strikethrough text~~
```
- GIVEN a Quarto document with `~~strikethrough text~~`
- WHEN the file is opened in Zed
- THEN "strikethrough text" is highlighted with `@text.strike` styling

#### Scenario: Highlighted text
```markdown
==highlighted text==
```
- GIVEN a Quarto document with `==highlighted text==`
- WHEN the file is opened in Zed
- THEN "highlighted text" is highlighted with `@text.highlight` styling

#### Scenario: Subscript text
```markdown
H~2~O
```
- GIVEN a Quarto document with subscript like `H~2~O`
- WHEN the file is opened in Zed
- THEN "2" is highlighted with `@text.subscript` styling

#### Scenario: Superscript text
```markdown
E=mc^2^
```
- GIVEN a Quarto document with superscript like `E=mc^2^`
- WHEN the file is opened in Zed
- THEN "2" is highlighted with `@text.super` styling

---

### Requirement: Automated Highlight Coverage Testing
**ID:** inline-highlight-testing

Automated tests must validate that inline content highlighting works correctly for all formatting patterns.

#### Scenario: Test suite validates emphasis highlighting
- GIVEN a test that parses `*italic*` with tree-sitter-quarto grammar
- WHEN running highlight queries from `languages/quarto/highlights.scm`
- THEN the test verifies that text nodes inside emphasis receive appropriate highlight scopes
- AND the test fails if content is not captured

#### Scenario: Test suite validates heading highlighting
- GIVEN a test that parses `# Heading Text` with tree-sitter-quarto grammar
- WHEN running highlight queries
- THEN the test verifies that the inline content captures `@text.title`
- AND the test validates that markers capture `@punctuation.special`

#### Scenario: Visual test fixture for manual validation
- GIVEN a comprehensive test file `tests/fixtures/inline-highlighting.qmd`
- WHEN opened in Zed with the extension installed
- THEN developers can visually verify all inline formatting styles
- AND the fixture includes examples of all covered inline formatting patterns

---

## Implementation Notes

### Query Structure
Tree-sitter highlight queries may need to:
1. Explicitly capture child `(text)` nodes within emphasis/strong/heading nodes
2. Use priority rules to ensure parent styling applies to children
3. Handle nested inline elements correctly (e.g., `*text with `code` inside*`)

### Potential Solutions
- **Explicit child captures**: Add queries like `(emphasis (text) @text.emphasis)`
- **Priority rules**: Use `(#set! "priority" N)` predicates to control capture precedence
- **Inheritance testing**: Validate whether Zed themes inherit parent node styles

### Related Documentation
- `docs/bold-highlighting-investigation/` - Previous investigation into dual-grammar inline injection
- `docs/scope-naming-decision.md` - Why we use Zed-compatible legacy scopes
- `languages/quarto/highlights.scm:85-98` - Current emphasis/strong/Pandoc extension queries
