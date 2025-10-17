# Test Refinements and Language Injection Cleanup

## ADDED Requirements

### Requirement: Citation Variation Test Coverage
The test suite MUST include tests for citation syntax variations beyond basic citations.

#### Scenario: Author suppression citation
- **GIVEN** markdown content `Citation with -@author`
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Parse the citation correctly
  - Indicate author suppression (if grammar supports)
  - Include the citation key

#### Scenario: Bracketed citation
- **GIVEN** markdown content `Citation @{https://example.com}`
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Parse the bracketed citation
  - Handle URL within brackets
  - Distinguish from regular citations

#### Scenario: Suppress author with brackets
- **GIVEN** markdown content `Citation -@{url}`
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Parse both suppression and bracketed syntax
  - Correctly combine both features

### Requirement: Shortcode Edge Case Test Coverage
The test suite MUST include tests for complex shortcode syntax variations.

#### Scenario: Escaped shortcode
- **GIVEN** markdown content `{{{< call >}}}`
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Treat this as escaped/literal text
  - Not parse it as a shortcode node

#### Scenario: Nested shortcode
- **GIVEN** markdown content `{{< outer {{< inner >}} >}}`
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Parse the outer shortcode
  - Include inner shortcode as argument
  - Maintain proper nesting structure

#### Scenario: Single-quoted string with escape
- **GIVEN** markdown content `{{< video 'url\'here' >}}`
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Parse the shortcode
  - Handle escaped single quote in string
  - Preserve the quote in the value

#### Scenario: Double-quoted string with escape
- **GIVEN** markdown content `{{< video "url\"here" >}}`
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Parse the shortcode
  - Handle escaped double quote in string
  - Preserve the quote in the value

### Requirement: Code Block Attribute Test Coverage
The test suite MUST include tests for code block attributes beyond executable cells.

#### Scenario: Code block with single-character class
- **GIVEN** fenced code block ` ```{.r}`
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Parse the code block
  - Include class attribute `.r`
  - Distinguish from executable cell syntax

#### Scenario: Code block with ID and class
- **GIVEN** fenced code block ` ```{#mycode .python}`
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Parse the code block
  - Include both ID and class attributes
  - Not treat as executable cell (no language in braces)

### Requirement: Fenced Div Edge Case Test Coverage
The test suite MUST include tests for edge cases in fenced div syntax.

#### Scenario: Empty fenced div
- **GIVEN** markdown content:
  ```
  ::: {}
  :::
  ```
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Parse the fenced div structure
  - Handle empty attribute list
  - Maintain proper block boundaries

#### Scenario: Fenced div with trailing space
- **GIVEN** closing fence `::: ` (with trailing space)
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Recognize the closing fence
  - Ignore trailing whitespace
  - Properly close the div block

### Requirement: Test Refinements Test Coverage
The test suite MUST include a dedicated file for edge case and refinement tests.

#### Scenario: Test refinements file exists
- **GIVEN** the test corpus directory `test/corpus/`
- **WHEN** checking for refinement tests
- **THEN** a file `test-refinements.txt` SHALL exist
- **AND** it SHALL contain at least 10 test cases

#### Scenario: All refinement tests pass
- **GIVEN** the test refinements corpus
- **WHEN** running `npx tree-sitter test`
- **THEN** all refinement tests SHALL pass
- **AND** the success rate SHALL remain 100%

## MODIFIED Requirements

### Requirement: Language Injection for Executable Code
The parser's language injection queries MUST focus exclusively on executable computation languages used in Quarto data science workflows.

**Previous behavior:** Injection queries included mermaid and dot/graphviz diagram languages.

**New behavior:** Injection queries SHALL only include languages that execute code for data analysis and computation.

#### Scenario: Python code injection
- **GIVEN** executable cell with language `python`
- **WHEN** applying injection queries
- **THEN** Python syntax highlighting SHALL be injected
- **AND** this behavior SHALL be maintained (no change)

#### Scenario: Mermaid code block without injection
- **GIVEN** fenced code block with info string `mermaid`
- **WHEN** applying injection queries
- **THEN** no language injection SHALL occur
- **AND** the content SHALL remain as plain text
- **AND** the code block SHALL still parse correctly

#### Scenario: Dot/Graphviz code block without injection
- **GIVEN** fenced code block with info string `dot`
- **WHEN** applying injection queries
- **THEN** no language injection SHALL occur
- **AND** the content SHALL remain as plain text
- **AND** the code block SHALL still parse correctly

#### Scenario: Supported executable languages maintained
- **GIVEN** the injection query file `queries/injections.scm`
- **WHEN** reviewing language injection rules
- **THEN** injections SHALL exist for:
  - Python (python, python3)
  - R
  - Julia
  - SQL
  - Bash (bash, sh, shell)
  - JavaScript/TypeScript (js, javascript, ts, typescript)
  - OJS (Observable JS)
- **AND** injections SHALL NOT exist for:
  - mermaid
  - dot
  - graphviz

### Requirement: Injection Query File Cleanliness
The injection query file MUST be focused and maintainable by excluding non-executable diagram languages.

#### Scenario: Mermaid injection removed
- **GIVEN** the file `queries/injections.scm`
- **WHEN** searching for mermaid injection rules
- **THEN** no mermaid-specific injection rules SHALL exist
- **AND** previously existing rules SHALL be removed

#### Scenario: Dot/Graphviz injection removed
- **GIVEN** the file `queries/injections.scm`
- **WHEN** searching for dot or graphviz injection rules
- **THEN** no dot/graphviz-specific injection rules SHALL exist
- **AND** previously existing rules SHALL be removed

#### Scenario: Line count reduced
- **GIVEN** the original `queries/injections.scm` with mermaid/dot rules
- **WHEN** removing these rules
- **THEN** the file SHALL be approximately 10-15 lines shorter
- **AND** core executable language injections SHALL remain intact
