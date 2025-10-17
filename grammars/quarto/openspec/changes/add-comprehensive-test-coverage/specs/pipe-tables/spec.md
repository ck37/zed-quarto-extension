# Pipe Tables Test Coverage

## ADDED Requirements

### Requirement: Basic Pipe Table Parsing
The parser MUST correctly parse pipe tables with headers, delimiter rows, and data rows using the grammar's pipe_table rules.

#### Scenario: Simple pipe table
- **GIVEN** markdown content:
  ```
  | Name | Age |
  |------|-----|
  | John | 30  |
  ```
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - A pipe_table node
  - A pipe_table_header node
  - A pipe_table_delimiter node
  - A pipe_table_row node

#### Scenario: Table with multiple data rows
- **GIVEN** a pipe table with:
  - 1 header row
  - 1 delimiter row
  - 3 data rows
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - All row nodes in correct order
  - Proper table structure maintained

### Requirement: Column Alignment Parsing
The parser MUST correctly parse column alignment indicators in pipe table delimiter rows.

#### Scenario: Left-aligned column
- **GIVEN** delimiter row `|:---|`
- **WHEN** parsing the table
- **THEN** the AST SHALL indicate:
  - Left alignment for the column
  - Proper delimiter structure

#### Scenario: Center-aligned column
- **GIVEN** delimiter row `|:---:|`
- **WHEN** parsing the table
- **THEN** the AST SHALL indicate:
  - Center alignment for the column

#### Scenario: Right-aligned column
- **GIVEN** delimiter row `|---:|`
- **WHEN** parsing the table
- **THEN** the AST SHALL indicate:
  - Right alignment for the column

#### Scenario: Mixed column alignment
- **GIVEN** delimiter row `|:---|:---:|---:|`
- **WHEN** parsing the table
- **THEN** the AST SHALL indicate:
  - Distinct alignment for each column
  - Left, center, and right alignments correctly identified

### Requirement: Special Character Handling in Tables
The parser MUST correctly handle special characters within table cells.

#### Scenario: Escaped pipe in cell
- **GIVEN** table cell content `text \| more text`
- **WHEN** parsing the table
- **THEN** the AST SHALL:
  - Treat `\|` as literal pipe character
  - Not interpret it as column delimiter
  - Preserve the escaped pipe in cell content

#### Scenario: Empty cell
- **GIVEN** table row `| Name | | Age |`
- **WHEN** parsing the table
- **THEN** the AST SHALL:
  - Recognize three columns
  - Include empty cell node for middle column

#### Scenario: Whitespace-only cell
- **GIVEN** table row with cell containing only spaces
- **WHEN** parsing the table
- **THEN** the AST SHALL:
  - Preserve the cell structure
  - Handle whitespace appropriately

### Requirement: Table Context Parsing
The parser MUST correctly parse pipe tables in various document contexts.

#### Scenario: Table followed by paragraph
- **GIVEN** a pipe table followed by a paragraph
- **WHEN** parsing the document
- **THEN** the AST SHALL:
  - Contain distinct pipe_table and paragraph nodes
  - Not merge content across boundaries

#### Scenario: Table followed by block quote
- **GIVEN** a pipe table followed by a block quote
- **WHEN** parsing the document
- **THEN** the AST SHALL:
  - Contain distinct pipe_table and block_quote nodes
  - Properly separate the blocks

### Requirement: Pipe Tables Test Coverage
The test suite MUST include comprehensive tests for pipe table syntax and edge cases.

#### Scenario: Pipe tables test file exists
- **GIVEN** the test corpus directory `test/corpus/`
- **WHEN** checking for pipe table tests
- **THEN** a file `pipe-tables.txt` SHALL exist
- **AND** it SHALL contain at least 10 test cases

#### Scenario: All pipe table tests pass
- **GIVEN** the pipe tables test corpus
- **WHEN** running `npx tree-sitter test`
- **THEN** all pipe table tests SHALL pass
- **AND** the success rate SHALL remain 100%
