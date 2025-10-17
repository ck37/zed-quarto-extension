# Footnotes Test Coverage

## ADDED Requirements

### Requirement: Inline Footnote Parsing
The parser MUST correctly parse inline footnotes using the `^[note]` syntax and produce structured AST nodes.

#### Scenario: Basic inline footnote
- **GIVEN** markdown content `Text with ^[inline note] here`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - A paragraph node
  - Inline content with text nodes
  - A footnote node (type determined by grammar rule)
  - The note content as a child of the footnote node

#### Scenario: Nested inline footnotes
- **GIVEN** markdown content `^[outer^[inner]]`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - A footnote node for the outer footnote
  - A nested footnote node for the inner footnote
  - Proper nesting hierarchy preserved

### Requirement: Footnote Reference Parsing
The parser MUST correctly parse footnote references using the `[^id]` syntax.

#### Scenario: Basic footnote reference
- **GIVEN** markdown content `Text with reference[^1]`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - A paragraph node
  - A footnote reference node
  - The reference identifier `1` as part of the node

#### Scenario: Multiple footnote references
- **GIVEN** markdown content `First[^1] and second[^2] references`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - Multiple footnote reference nodes
  - Each with distinct identifiers
  - Proper ordering preserved

### Requirement: Footnote Definition Parsing
The parser MUST correctly parse footnote definitions using the `[^id]: content` syntax.

#### Scenario: Basic footnote definition
- **GIVEN** markdown content:
  ```
  [^1]: This is a footnote definition
  ```
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - A footnote_definition node (as defined in grammar.js)
  - The identifier `1` as the marker
  - The definition content as a child node

#### Scenario: Footnote definition with inline formatting
- **GIVEN** markdown content:
  ```
  [^note]: This has **bold** and *italic* text
  ```
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - A footnote_definition node
  - Inline content with formatted text nodes
  - Emphasis and strong_emphasis nodes preserved

### Requirement: Footnote Test Coverage
The test suite MUST include tests for all footnote syntax variations supported by the grammar.

#### Scenario: Footnote test file exists
- **GIVEN** the test corpus directory `test/corpus/`
- **WHEN** checking for footnote tests
- **THEN** a file `footnotes.txt` SHALL exist
- **AND** it SHALL contain at least 6 test cases

#### Scenario: All footnote tests pass
- **GIVEN** the footnote test corpus
- **WHEN** running `npx tree-sitter test`
- **THEN** all footnote tests SHALL pass
- **AND** the success rate SHALL remain 100%
