# Chunk Options

## Purpose
Parse chunk options (`#| key: value`) within executable code cells as structured data for semantic highlighting and validation.

## Requirements

### Requirement: Basic Chunk Option Syntax
The parser SHALL parse chunk options that begin with `#|` at the start of executable cells.

#### Scenario: Single chunk option
- **WHEN** parsing `#| label: fig-plot`
- **THEN** creates chunk_option node
- **AND** captures key as "label"
- **AND** captures value as "fig-plot"

#### Scenario: Multiple chunk options
- **WHEN** cell contains multiple `#|` lines
- **THEN** creates chunk_options container node
- **AND** includes multiple chunk_option children
- **AND** preserves order of options

#### Scenario: Chunk option with boolean value
- **WHEN** parsing `#| echo: false`
- **THEN** captures key as "echo"
- **AND** captures value as "false"

### Requirement: Chunk Option Position
The parser SHALL only recognize chunk options at the start of cell content.

#### Scenario: Options at cell start
- **WHEN** `#|` appears immediately after opening fence
- **THEN** parsed as chunk_option
- **AND** included in chunk_options node

#### Scenario: Hash comment after code
- **WHEN** `#` appears after non-option content
- **THEN** parsed as code comment
- **AND** not as chunk_option

#### Scenario: Options must be contiguous
- **WHEN** blank line separates `#|` lines
- **THEN** only first group parsed as chunk options
- **AND** subsequent `#|` lines are code comments

### Requirement: Key-Value Parsing
The parser SHALL parse chunk option keys and values as separate nodes.

#### Scenario: Key extraction
- **WHEN** parsing `#| key: value`
- **THEN** captures key using field('key', $.chunk_option_key)
- **AND** key matches pattern /[a-zA-Z][a-zA-Z0-9-]*/
- **AND** supports hyphenated keys like "fig-cap"

#### Scenario: Value extraction
- **WHEN** parsing `#| key: value`
- **THEN** captures value using field('value', $.chunk_option_value)
- **AND** value is everything after colon to newline
- **AND** preserves leading/trailing whitespace in value

#### Scenario: Values with special characters
- **WHEN** value contains quotes, commas, or symbols
- **THEN** entire value string is captured
- **AND** no special escaping required

### Requirement: Multi-line Values
The parser SHALL support multi-line chunk option values using pipe syntax.

#### Scenario: Multi-line value with pipe
- **WHEN** parsing:
  ```
  #| fig-cap: |
  #|   Line 1
  #|   Line 2
  ```
- **THEN** creates single chunk_option
- **AND** value includes all continuation lines
- **AND** preserves relative indentation

#### Scenario: Pipe continuation marker
- **WHEN** value line ends with `|`
- **THEN** parser expects continuation lines
- **AND** continuation lines start with `#|`
- **AND** stops at first non-continuation line

### Requirement: External Scanner Detection
The parser SHALL use external scanner to distinguish chunk options from comments.

#### Scenario: Scanner checks cell context
- **WHEN** scanner encounters `#|` token
- **THEN** checks if currently in executable cell
- **AND** checks if at start of cell content
- **AND** emits CHUNK_OPTION_MARKER if valid

#### Scenario: Regular comment outside cell
- **WHEN** scanner encounters `#` outside cell
- **THEN** does not emit CHUNK_OPTION_MARKER
- **AND** parsed as regular markdown heading or code comment

#### Scenario: Comment after code in cell
- **WHEN** scanner encounters `#` after code lines
- **THEN** cell context shows past options section
- **AND** does not emit CHUNK_OPTION_MARKER

### Requirement: Common Chunk Options
The parser SHALL handle all standard Quarto chunk options.

#### Scenario: Label option
- **WHEN** parsing `#| label: fig-plot`
- **THEN** key is "label"
- **AND** value is "fig-plot"

#### Scenario: Echo option
- **WHEN** parsing `#| echo: false`
- **THEN** key is "echo"
- **AND** value is "false"

#### Scenario: Figure caption option
- **WHEN** parsing `#| fig-cap: "Sample plot"`
- **THEN** key is "fig-cap"
- **AND** value includes quotes: `"Sample plot"`

#### Scenario: Output option
- **WHEN** parsing `#| output: asis`
- **THEN** key is "output"
- **AND** value is "asis"

#### Scenario: Warning option
- **WHEN** parsing `#| warning: false`
- **THEN** key is "warning"
- **AND** value is "false"

## Integration Requirements

### Requirement: Works Within Executable Cells
The parser SHALL parse chunk options as children of executable_code_cell nodes.

#### Scenario: Cell with options and code
- **WHEN** cell contains:
  ```
  ```{python}
  #| label: fig-plot
  #| echo: false
  import matplotlib.pyplot as plt
  plt.plot([1, 2, 3])
  ```
  ```
- **THEN** executable_code_cell contains chunk_options node
- **AND** chunk_options has two chunk_option children
- **AND** cell_content contains Python code only

#### Scenario: Cell without options
- **WHEN** cell has no `#|` lines
- **THEN** no chunk_options node created
- **AND** all content is cell_content

### Requirement: Enables Syntax Highlighting
The parser SHALL structure options to enable distinct highlighting.

#### Scenario: Keys highlighted differently than values
- **WHEN** editor applies syntax highlighting
- **THEN** chunk_option_key nodes can be styled separately
- **AND** chunk_option_value nodes styled differently
- **AND** `#|` marker styled as punctuation

## Edge Cases

### Requirement: Malformed Options
The parser SHALL handle malformed chunk options gracefully.

#### Scenario: Missing colon
- **WHEN** parsing `#| label fig-plot`
- **THEN** attempts to parse as option
- **AND** may create ERROR node
- **AND** continues parsing next line

#### Scenario: Empty key
- **WHEN** parsing `#| : value`
- **THEN** marks error on empty key
- **AND** continues parsing

#### Scenario: Empty value
- **WHEN** parsing `#| key:`
- **THEN** captures empty value
- **AND** chunk_option remains valid

### Requirement: Whitespace Handling
The parser SHALL handle various whitespace patterns.

#### Scenario: Spaces around colon
- **WHEN** parsing `#| key : value`
- **THEN** colon separates key and value
- **AND** whitespace handled correctly

#### Scenario: Indented options
- **WHEN** `#|` lines have leading spaces
- **THEN** indentation ignored
- **AND** options parsed normally

#### Scenario: Tab characters
- **WHEN** tabs appear in option lines
- **THEN** treated as whitespace
- **AND** parsing succeeds

## Validation Support

### Requirement: Enables Option Validation
The parser SHALL structure options to support downstream validation.

#### Scenario: Option name validation
- **WHEN** language server checks options
- **THEN** can extract all chunk_option_key nodes
- **AND** validate against known option names
- **AND** warn on typos like "lable" instead of "label"

#### Scenario: Option value type checking
- **WHEN** language server checks values
- **THEN** can extract chunk_option_value for each key
- **AND** validate value type (boolean, string, numeric)
- **AND** warn on invalid values

#### Scenario: Language-specific options
- **WHEN** checking options for Python cell
- **THEN** can access cell's language_name
- **AND** validate options are valid for Python
- **AND** warn on R-specific options in Python cell
