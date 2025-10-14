# Inline Code Cells

## Purpose
Parse inline executable code cells with language specifiers (`` `{python} expr` ``) to enable language injection and distinguish from regular inline code.

## Requirements

### Requirement: Basic Inline Cell Syntax
The parser SHALL parse inline code cells with curly brace language specifiers.

#### Scenario: Python inline cell
- **WHEN** parsing `` `{python} mean([1, 2, 3])` ``
- **THEN** creates inline_code_cell node
- **AND** captures language_name as "python"
- **AND** captures cell_content as "mean([1, 2, 3])"

#### Scenario: R inline cell
- **WHEN** parsing `` `{r} mean(x)` ``
- **THEN** creates inline_code_cell node
- **AND** captures language_name as "r"
- **AND** captures cell_content as "mean(x)"

#### Scenario: Julia inline cell
- **WHEN** parsing `` `{julia} sum(arr)` ``
- **THEN** creates inline_code_cell node
- **AND** captures language_name as "julia"

### Requirement: Shorthand Syntax
The parser SHALL support shorthand syntax without braces for default language.

#### Scenario: Shorthand R syntax
- **WHEN** parsing `` `r mean(x)` ``
- **THEN** creates inline_code_cell node
- **AND** language_name is "r"
- **AND** content follows language specifier

#### Scenario: Other shorthand languages
- **WHEN** parsing `` `python expr` `` (if supported)
- **THEN** creates inline_code_cell or regular code_span
- **AND** behavior matches Quarto conventions

### Requirement: Distinction from Code Spans
The parser SHALL distinguish inline code cells from regular inline code.

#### Scenario: Regular inline code
- **WHEN** parsing `` `code` `` without language
- **THEN** creates regular code_span node
- **AND** not inline_code_cell

#### Scenario: Inline cell with language
- **WHEN** parsing `` `{python} expr` ``
- **THEN** creates inline_code_cell node
- **AND** not regular code_span

#### Scenario: Code span with literal braces
- **WHEN** parsing `` `{not a language}` ``
- **THEN** may create code_span if not valid language
- **AND** or inline_code_cell if language detected

### Requirement: Cell Content Capture
The parser SHALL capture inline cell content for language injection.

#### Scenario: Simple expression
- **WHEN** parsing `` `{python} 2 + 2` ``
- **THEN** cell_content is "2 + 2"
- **AND** preserves exact spacing

#### Scenario: Complex expression
- **WHEN** parsing `` `{python} df['column'].sum()` ``
- **THEN** cell_content is "df['column'].sum()"
- **AND** preserves brackets and quotes

#### Scenario: Expression with backticks
- **WHEN** content contains escaped backticks
- **THEN** cell_content includes escaped characters
- **AND** maintains proper escaping

### Requirement: Language Detection
The parser SHALL recognize supported inline execution languages.

#### Scenario: Common languages
- **WHEN** language is python, r, julia, sql
- **THEN** creates inline_code_cell
- **AND** language_name captured for injection

#### Scenario: Unknown language handling
- **WHEN** language not in known list
- **THEN** may create inline_code_cell
- **OR** fall back to regular code_span
- **AND** behavior is consistent

### Requirement: Inline Context Integration
The parser SHALL parse inline cells within paragraph and inline contexts.

#### Scenario: Inline cell in paragraph
- **WHEN** parsing `The result is `{python} compute()`.`
- **THEN** inline_code_cell is child of paragraph
- **AND** surrounded by text nodes

#### Scenario: Multiple inline cells in sentence
- **WHEN** parsing `Sum: `{python} sum(x)`, Mean: `{python} mean(x)``
- **THEN** creates two inline_code_cell nodes
- **AND** both in same paragraph

#### Scenario: Inline cells in emphasis
- **WHEN** parsing `*The value `{r} x` is important*`
- **THEN** inline_code_cell nested in emphasis
- **AND** parsed correctly

## Language Injection Support

### Requirement: Enables Inline Syntax Highlighting
The parser SHALL structure inline cells to enable language-specific highlighting.

#### Scenario: Python syntax in inline cell
- **WHEN** cell has language_name "python"
- **THEN** injection query targets cell_content
- **AND** Python parser highlights expression

#### Scenario: Multiple languages inline
- **WHEN** paragraph has Python and R inline cells
- **THEN** each gets correct language injection
- **AND** languages don't interfere

## Edge Cases

### Requirement: Delimiter Handling
The parser SHALL handle backtick delimiters correctly.

#### Scenario: Single backtick delimiters
- **WHEN** parsing `` `{python} expr` ``
- **THEN** single backticks mark cell boundaries
- **AND** content between delimiters

#### Scenario: Nested backticks in content
- **WHEN** content includes backticks
- **THEN** requires proper escaping
- **AND** parser handles escaped characters

### Requirement: Empty and Whitespace Content
The parser SHALL handle edge cases with spacing.

#### Scenario: Empty cell content
- **WHEN** parsing `` `{python}` ``
- **THEN** creates inline_code_cell
- **AND** cell_content is empty

#### Scenario: Whitespace-only content
- **WHEN** parsing `` `{python}   ` ``
- **THEN** creates inline_code_cell
- **AND** preserves whitespace in content

#### Scenario: Whitespace around language
- **WHEN** parsing `` `{ python } expr` ``
- **THEN** handles spacing around language name
- **AND** extracts "python" correctly

### Requirement: Malformed Inline Cells
The parser SHALL handle malformed inline cells gracefully.

#### Scenario: Missing closing backtick
- **WHEN** parsing `` `{python} expr ``
- **THEN** may create ERROR node
- **AND** recovers for subsequent content

#### Scenario: Incomplete language specifier
- **WHEN** parsing `` `{python expr` ``
- **THEN** may parse as code_span
- **OR** create incomplete inline_code_cell

## Integration Requirements

### Requirement: Works with Citations and Cross-References
The parser SHALL correctly precedence inline cells with other inline constructs.

#### Scenario: Inline cell near citation
- **WHEN** parsing `Value `{r} x` from @smith2020`
- **THEN** inline_code_cell and citation both parsed
- **AND** no interference between constructs

#### Scenario: Inline cell near cross-reference
- **WHEN** parsing `See `{python} result` and @fig-plot`
- **THEN** inline_code_cell and cross_reference both parsed

### Requirement: Works with Emphasis and Links
The parser SHALL handle inline cells within other inline formatting.

#### Scenario: Inline cell in bold
- **WHEN** parsing `**Value: `{r} x`**`
- **THEN** inline_code_cell nested in strong emphasis
- **AND** parsed correctly

#### Scenario: Inline cell in link text
- **WHEN** parsing `[Result: `{python} val`](url)`
- **THEN** inline_code_cell nested in link
- **AND** may require special handling

## Performance Requirements

### Requirement: Efficient Inline Parsing
The parser SHALL parse inline cells efficiently in long paragraphs.

#### Scenario: Many inline cells in paragraph
- **WHEN** paragraph contains 10+ inline cells
- **THEN** parsing remains fast
- **AND** no performance degradation

#### Scenario: Long inline expressions
- **WHEN** inline cell contains 100+ characters
- **THEN** parses efficiently
- **AND** captures full content

## Validation Support

### Requirement: Enables Expression Validation
The parser SHALL structure cells to support validation.

#### Scenario: Language-specific validation
- **WHEN** language server checks inline cell
- **THEN** can extract language_name
- **AND** validate expression syntax
- **AND** warn on invalid expressions

#### Scenario: Undefined variable detection
- **WHEN** inline cell references variable
- **THEN** validator can check if variable defined
- **AND** warn on undefined references
