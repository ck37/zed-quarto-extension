# Inline Attributes Specification

## Overview

This specification defines parsing support for Pandoc-style inline attributes (bracketed spans) in Quarto Markdown. Inline attributes allow authors to add IDs, CSS classes, and custom key-value attributes to arbitrary spans of text.

**Syntax:** `[text content]{#id .class key="value"}`

**Purpose:** Enable semantic markup, styling, and cross-referencing of text spans within documents.

## ADDED Requirements

### Requirement: Bracketed Span Parsing
The parser MUST recognize and parse bracketed spans with the pattern `[text]{attributes}`.

#### Scenario: Basic bracketed span with ID
- **GIVEN** markdown content `[highlighted text]{#important}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - A `bracketed_span` node
  - Span content containing the text
  - An `attribute_list` node with `attribute_id` = "important"

#### Scenario: Bracketed span with class
- **GIVEN** markdown content `[warning message]{.alert}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - A `bracketed_span` node
  - An `attribute_list` with `attribute_class` = "alert"

#### Scenario: Bracketed span with multiple classes
- **GIVEN** markdown content `[text]{.primary .highlight .bold}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - A `bracketed_span` node
  - Multiple `attribute_class` nodes: "primary", "highlight", "bold"

#### Scenario: Bracketed span with key-value attribute
- **GIVEN** markdown content `[data]{data-value="123"}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - A `bracketed_span` node
  - A `key_value_attribute` with key="data-value" and value="123"

#### Scenario: Bracketed span with mixed attributes
- **GIVEN** markdown content `[content]{#anchor .highlight data-ref="fig-1"}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - `attribute_id` = "anchor"
  - `attribute_class` = "highlight"
  - `key_value_attribute` with key="data-ref" and value="fig-1"

#### Scenario: Empty attribute list
- **GIVEN** markdown content `[text]{}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - A `bracketed_span` node
  - An empty `attribute_list` node

### Requirement: Span Content Parsing
The parser MUST support inline elements within bracketed span content.

#### Scenario: Span with emphasis
- **GIVEN** markdown content `[**bold** and *italic*]{.styled}`
- **WHEN** parsing the document
- **THEN** the span content SHALL contain:
  - `strong_emphasis` node for "bold"
  - `emphasis` node for "italic"
  - `attribute_class` = "styled"

#### Scenario: Span with code
- **GIVEN** markdown content `[see `code` example]{.tech}`
- **WHEN** parsing the document
- **THEN** the span content SHALL contain:
  - Text "see "
  - `code_span` node for "code"
  - Text " example"

#### Scenario: Span with inline math
- **GIVEN** markdown content `[$x^2 + y^2$]{.math}`
- **WHEN** parsing the document
- **THEN** the span content SHALL contain:
  - `inline_math` node
  - `attribute_class` = "math"

### Requirement: Nested Spans
The parser MUST handle nested bracketed spans.

#### Scenario: Simple nested spans
- **GIVEN** markdown content `[[inner]{.a}]{.b}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - Outer `bracketed_span` with class "b"
  - Inner `bracketed_span` with class "a" as content

#### Scenario: Multiple spans in sequence
- **GIVEN** markdown content `[first]{.a} and [second]{.b}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - Two separate `bracketed_span` nodes
  - Text " and " between them

### Requirement: Span vs Link Disambiguation
The parser MUST distinguish bracketed spans from links.

#### Scenario: Bracketed span identified correctly
- **GIVEN** markdown content `[text]{.class}`
- **WHEN** checking for link syntax
- **THEN** the parser SHALL recognize this as a `bracketed_span`
- **AND** NOT attempt to parse as a `link`

#### Scenario: Link identified correctly
- **GIVEN** markdown content `[text](url)`
- **WHEN** checking for span syntax
- **THEN** the parser SHALL recognize this as a `link`
- **AND** NOT attempt to parse as a `bracketed_span`

#### Scenario: Span with bracket in content
- **GIVEN** markdown content `[text [with] brackets]{.class}`
- **WHEN** parsing the document
- **THEN** the parser SHALL:
  - Match the outermost brackets for span boundaries
  - Include "[with]" as part of the content

### Requirement: Heading Attributes
The parser MUST support optional attributes after heading content.

#### Scenario: ATX heading with ID
- **GIVEN** markdown content `# Introduction {#intro}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - An `atx_heading` node
  - Heading content "Introduction"
  - An `attribute_list` with `attribute_id` = "intro"

#### Scenario: ATX heading with class
- **GIVEN** markdown content `## Background {.section-bg}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - An `atx_heading` node (level 2)
  - Heading content "Background"
  - `attribute_class` = "section-bg"

#### Scenario: Heading with mixed attributes
- **GIVEN** markdown content `### Methods {#methods .chapter data-section="3"}`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - `attribute_id` = "methods"
  - `attribute_class` = "chapter"
  - `key_value_attribute` with key="data-section" value="3"

#### Scenario: Setext heading with attributes
- **GIVEN** markdown content:
  ```
  Heading Text {#custom}
  ============
  ```
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - A `setext_heading` node
  - Heading content with attributes

#### Scenario: Heading without attributes
- **GIVEN** markdown content `# Plain Heading`
- **WHEN** parsing the document
- **THEN** the AST SHALL contain:
  - An `atx_heading` node
  - Heading content only
  - NO `attribute_list` node

### Requirement: Attribute Syntax Validation
The parser MUST correctly parse all valid attribute syntaxes.

#### Scenario: ID must start with hash
- **GIVEN** markdown content `[text]{#valid-id}`
- **WHEN** parsing attributes
- **THEN** the ID SHALL be extracted as "valid-id"
- **AND** the hash (#) SHALL NOT be part of the ID value

#### Scenario: Class must start with period
- **GIVEN** markdown content `[text]{.valid-class}`
- **WHEN** parsing attributes
- **THEN** the class SHALL be extracted as "valid-class"
- **AND** the period (.) SHALL NOT be part of the class value

#### Scenario: Key-value with quoted value
- **GIVEN** markdown content `[text]{key="value with spaces"}`
- **WHEN** parsing attributes
- **THEN** the value SHALL preserve spaces
- **AND** quotes SHALL be part of the attribute_value token

#### Scenario: Key-value with single quotes
- **GIVEN** markdown content `[text]{key='value'}`
- **WHEN** parsing attributes
- **THEN** the parser SHALL accept single-quoted values
- **AND** parse equivalently to double-quoted values

#### Scenario: Key-value unquoted
- **GIVEN** markdown content `[text]{key=value123}`
- **WHEN** parsing attributes
- **THEN** the unquoted value SHALL be accepted
- **AND** SHALL NOT contain whitespace

### Requirement: Edge Cases
The parser MUST handle edge cases gracefully.

#### Scenario: Whitespace in attribute list
- **GIVEN** markdown content `[text]{ #id .class key="value" }`
- **WHEN** parsing attributes
- **THEN** leading and trailing whitespace SHALL be ignored
- **AND** all attributes SHALL be parsed correctly

#### Scenario: Span at paragraph boundaries
- **GIVEN** markdown content:
  ```
  [start of paragraph]{.intro}

  [whole paragraph]{.block}

  End with [span]{.end}
  ```
- **WHEN** parsing the document
- **THEN** all spans SHALL parse correctly
- **AND** paragraph boundaries SHALL be maintained

#### Scenario: Adjacent spans
- **GIVEN** markdown content `[first]{.a}[second]{.b}`
- **WHEN** parsing the document
- **THEN** both spans SHALL be recognized
- **AND** no text node between them

#### Scenario: Span with escaped characters
- **GIVEN** markdown content `[text with \] bracket]{.class}`
- **WHEN** parsing the document
- **THEN** the escaped bracket SHALL be part of content
- **AND** SHALL NOT close the span early

### Requirement: Syntax Highlighting Support
The parser MUST provide query points for syntax highlighting of inline attributes.

#### Scenario: Bracketed span highlighting
- **GIVEN** a `queries/highlights.scm` file
- **WHEN** defining highlight patterns
- **THEN** patterns SHALL exist for:
  - `bracketed_span` → `@markup.span`
  - `attribute_id` → `@constant`
  - `attribute_class` → `@tag`
  - `attribute_key` → `@property`
  - `attribute_value` → `@string`

#### Scenario: Heading attribute highlighting
- **GIVEN** a heading with attributes
- **WHEN** syntax highlighting is applied
- **THEN** heading content and attributes SHALL be highlighted separately
- **AND** attributes SHALL use the same highlighting as span attributes

### Requirement: Test Coverage
The test suite MUST comprehensively cover inline attribute functionality.

#### Scenario: Test file exists
- **GIVEN** the test corpus directory
- **WHEN** checking for inline attribute tests
- **THEN** a file `test/corpus/inline-attributes.txt` SHALL exist
- **AND** SHALL contain at least 12 test cases

#### Scenario: All inline attribute tests pass
- **GIVEN** the inline attributes test corpus
- **WHEN** running `npx tree-sitter test`
- **THEN** all inline attribute tests SHALL pass
- **AND** no regressions in existing tests

#### Scenario: Tests cover all requirement scenarios
- **GIVEN** this specification
- **WHEN** reviewing test coverage
- **THEN** each requirement scenario SHALL have at least one test case
- **AND** edge cases SHALL be tested

## Non-Functional Requirements

### Performance
- Bracketed span parsing SHALL NOT significantly impact parse performance
- Target: <100ms for documents with hundreds of spans
- Incremental reparsing SHALL work efficiently with span edits

### Compatibility
- SHALL maintain backward compatibility with existing node types
- SHALL NOT modify behavior of existing features
- SHALL follow tree-sitter best practices

### Maintainability
- Grammar rules SHALL be clearly documented
- Test cases SHALL include descriptive names
- Highlight queries SHALL follow existing patterns
