# Executable Code Cells

## Purpose
Parse executable code cells with language specifiers and cell attributes, distinguishing them from regular code blocks.

## Requirements

### Requirement: Basic Cell Structure
The parser SHALL parse executable code cells with language specifiers in curly braces.

#### Scenario: Python cell with language specifier
- **WHEN** parsing `` ```{python} ``
- **THEN** creates executable_code_cell node
- **AND** captures language_name as "python"
- **AND** distinguishes from regular code block

#### Scenario: R cell with language specifier
- **WHEN** parsing `` ```{r} ``
- **THEN** creates executable_code_cell node
- **AND** captures language_name as "r"

#### Scenario: Julia cell with language specifier
- **WHEN** parsing `` ```{julia} ``
- **THEN** creates executable_code_cell node
- **AND** captures language_name as "julia"

### Requirement: Cell Delimiters
The parser SHALL recognize cell boundaries using fence delimiters.

#### Scenario: Standard three-backtick fence
- **WHEN** parsing `` ``` `` as cell delimiter
- **THEN** marks cell_delimiter at start
- **AND** matches closing `` ``` ``
- **AND** captures content between delimiters

#### Scenario: Extended fence lengths
- **WHEN** parsing `` ```` `` or longer fences
- **THEN** matches closing fence of same length
- **AND** allows shorter fences in content

### Requirement: Cell Attributes
The parser SHALL parse optional cell attributes in curly braces.

#### Scenario: Cell with attributes
- **WHEN** parsing `` ```{python #fig-plot .class} ``
- **THEN** captures cell_attributes
- **AND** includes id "fig-plot"
- **AND** includes class "class"

#### Scenario: Cell without attributes
- **WHEN** parsing `` ```{python} ``
- **THEN** cell_attributes is optional/empty
- **AND** cell still parses successfully

### Requirement: Cell Content Capture
The parser SHALL capture cell content as a distinct node for language injection.

#### Scenario: Multi-line cell content
- **WHEN** parsing cell with multiple code lines
- **THEN** creates cell_content node
- **AND** captures all lines between delimiters
- **AND** preserves exact indentation and whitespace

#### Scenario: Empty cell content
- **WHEN** parsing cell with no content
- **THEN** creates empty cell_content node
- **AND** cell remains valid

#### Scenario: Cell with chunk options and code
- **WHEN** cell contains chunk options followed by code
- **THEN** cell_content includes both
- **AND** chunk options are separately parsed
- **AND** remaining content available for language injection

### Requirement: Language Support
The parser SHALL support all Quarto-compatible execution engines.

#### Scenario: Common languages recognized
- **WHEN** language is python, r, julia, sql, or bash
- **THEN** parser creates executable_code_cell
- **AND** language_name is captured correctly

#### Scenario: Uncommon languages handled
- **WHEN** language is mermaid, dot, or other supported type
- **THEN** parser creates executable_code_cell
- **AND** language_name is captured for injection

#### Scenario: Unknown languages accepted
- **WHEN** language is not in known list
- **THEN** parser still creates executable_code_cell
- **AND** allows editors to handle unknown types

### Requirement: Distinction from Code Blocks
The parser SHALL distinguish executable cells from regular code blocks.

#### Scenario: Regular code block parsing
- **WHEN** parsing `` ```python `` without braces
- **THEN** creates regular code_block node
- **AND** not executable_code_cell

#### Scenario: Executable cell has curly braces
- **WHEN** parsing `` ```{python} ``
- **THEN** creates executable_code_cell node
- **AND** not regular code_block

### Requirement: Cell Context Tracking
The parser SHALL maintain cell context for chunk option detection.

#### Scenario: Track when inside executable cell
- **WHEN** parsing begins after `` ```{python} ``
- **THEN** scanner tracks "in_cell" state
- **AND** enables chunk option detection
- **AND** clears state at closing fence

#### Scenario: Regular code blocks don't track context
- **WHEN** parsing begins after `` ```python ``
- **THEN** scanner does not set "in_cell" state
- **AND** chunk options not detected

## Edge Cases

### Requirement: Nested Fences
The parser SHALL handle nested fence constructs correctly.

#### Scenario: Code example inside cell
- **WHEN** cell content contains `` ``` `` fences
- **THEN** inner fences are part of content
- **AND** only matching outer fence closes cell

#### Scenario: Different fence lengths
- **WHEN** opening fence is `` ```` ``
- **THEN** cell closes on matching `` ```` ``
- **AND** shorter `` ``` `` fences remain content

### Requirement: Incomplete Cells
The parser SHALL handle incomplete or malformed cells.

#### Scenario: Missing closing fence
- **WHEN** document ends without closing fence
- **THEN** creates incomplete executable_code_cell
- **AND** includes ERROR node
- **AND** captures available content

#### Scenario: Malformed language specifier
- **WHEN** parsing `` ```{python ``
- **THEN** attempts to parse as cell
- **AND** marks error on incomplete specifier
- **AND** recovers for subsequent content

## Integration Requirements

### Requirement: Works with Chunk Options
The parser SHALL coordinate with chunk option parsing within cells.

#### Scenario: Cell with chunk options
- **WHEN** executable cell contains `#|` lines
- **THEN** chunk options are parsed as child nodes
- **AND** options appear before code content
- **AND** cell_content excludes option lines

#### Scenario: Cell without chunk options
- **WHEN** executable cell has no `#|` lines
- **THEN** cell_content contains all code
- **AND** no chunk_options node present

### Requirement: Enables Language Injection
The parser SHALL structure cells to enable language-specific syntax highlighting.

#### Scenario: Python code injection
- **WHEN** cell has language_name "python"
- **THEN** injection query can target cell_content
- **AND** Python parser highlights code

#### Scenario: Multiple languages in document
- **WHEN** document has Python and R cells
- **THEN** each cell_content gets correct injection
- **AND** languages don't interfere
