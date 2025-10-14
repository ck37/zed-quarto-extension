# Language Injection

## Purpose
Enable syntax highlighting for multiple programming languages within a single Quarto document through tree-sitter injection queries.

## Requirements

### Requirement: Injection Query Infrastructure
The parser SHALL provide injection queries that enable language-specific syntax highlighting.

#### Scenario: Injection query file exists
- **WHEN** parser is installed
- **THEN** includes queries/injections.scm file
- **AND** defines injection rules for supported languages

#### Scenario: Query targets cell content
- **WHEN** injection query is evaluated
- **THEN** identifies cell_content nodes
- **AND** extracts language_name
- **AND** sets injection.language appropriately

### Requirement: Python Code Injection
The parser SHALL enable Python syntax highlighting in Python cells.

#### Scenario: Python executable cell
- **WHEN** parsing:
  ```
  ```{python}
  import numpy as np
  result = np.mean([1, 2, 3])
  ```
  ```
- **THEN** cell_content receives Python injection
- **AND** Python syntax highlighted
- **AND** Python keywords recognized

#### Scenario: Python inline cell
- **WHEN** parsing `` `{python} np.mean(x)` ``
- **THEN** cell_content receives Python injection
- **AND** Python syntax highlighted inline

### Requirement: R Code Injection
The parser SHALL enable R syntax highlighting in R cells.

#### Scenario: R executable cell
- **WHEN** parsing:
  ```
  ```{r}
  library(ggplot2)
  mean(x)
  ```
  ```
- **THEN** cell_content receives R injection
- **AND** R syntax highlighted
- **AND** R functions recognized

#### Scenario: R inline cell
- **WHEN** parsing `` `{r} mean(x)` `` or `` `r mean(x)` ``
- **THEN** cell_content receives R injection
- **AND** R syntax highlighted

### Requirement: Julia Code Injection
The parser SHALL enable Julia syntax highlighting in Julia cells.

#### Scenario: Julia executable cell
- **WHEN** parsing:
  ```
  ```{julia}
  using Statistics
  mean([1, 2, 3])
  ```
  ```
- **THEN** cell_content receives Julia injection
- **AND** Julia syntax highlighted

### Requirement: SQL Code Injection
The parser SHALL enable SQL syntax highlighting in SQL cells.

#### Scenario: SQL executable cell
- **WHEN** parsing:
  ```
  ```{sql}
  SELECT * FROM users WHERE active = 1;
  ```
  ```
- **THEN** cell_content receives SQL injection
- **AND** SQL syntax highlighted

### Requirement: Bash Code Injection
The parser SHALL enable Bash syntax highlighting in Bash cells.

#### Scenario: Bash executable cell
- **WHEN** parsing:
  ```
  ```{bash}
  echo "Hello, World!"
  ls -la
  ```
  ```
- **THEN** cell_content receives Bash injection
- **AND** Bash commands highlighted

### Requirement: Multi-Language Document Support
The parser SHALL support multiple different languages in a single document.

#### Scenario: Python and R in same document
- **WHEN** document contains both Python and R cells
- **THEN** Python cells get Python injection
- **AND** R cells get R injection
- **AND** languages don't interfere with each other

#### Scenario: Multiple language types
- **WHEN** document has Python, R, Julia, SQL cells
- **THEN** each cell gets correct language injection
- **AND** all languages work simultaneously

### Requirement: Injection Query Pattern
The parser SHALL use tree-sitter injection query syntax correctly.

#### Scenario: Language conditional injection
- **WHEN** injection query checks language_name
- **THEN** uses (#eq? @_lang "python") predicate
- **AND** sets (#set! injection.language "python")
- **AND** targets cell_content node

#### Scenario: Query pattern format
- **WHEN** defining injection rules
- **THEN** follows tree-sitter query syntax
- **AND** uses correct node names from grammar
- **AND** uses field names where defined

### Requirement: Chunk Options Excluded
The parser SHALL NOT apply language injection to chunk options.

#### Scenario: Options before code
- **WHEN** cell contains:
  ```
  ```{python}
  #| label: fig-plot
  import numpy as np
  ```
  ```
- **THEN** chunk options not Python-highlighted
- **AND** only code content receives injection

#### Scenario: Cell content is code only
- **WHEN** cell_content node defined
- **THEN** excludes chunk_options lines
- **AND** includes only actual code

## Editor Integration

### Requirement: Works with nvim-treesitter
The parser SHALL provide injections compatible with nvim-treesitter.

#### Scenario: Neovim highlights multiple languages
- **WHEN** document opened in Neovim with treesitter
- **THEN** Python cells use Python highlighting
- **AND** R cells use R highlighting
- **AND** markdown uses markdown highlighting

### Requirement: Works with Zed Editor
The parser SHALL provide injections compatible with Zed.

#### Scenario: Zed highlights cell content
- **WHEN** document opened in Zed
- **THEN** cell content receives language injection
- **AND** syntax highlighting works

### Requirement: Works with Helix
The parser SHALL provide injections compatible with Helix editor.

#### Scenario: Helix highlights multiple languages
- **WHEN** document opened in Helix
- **THEN** language injections work
- **AND** multiple languages highlighted

## Fallback Handling

### Requirement: Unknown Language Handling
The parser SHALL handle unknown or unsupported languages gracefully.

#### Scenario: Unknown language specified
- **WHEN** cell has language_name not in injection queries
- **THEN** no injection applied
- **AND** content remains plain text
- **AND** no parsing errors

#### Scenario: Language parser not installed
- **WHEN** language specified but parser unavailable
- **THEN** injection query exists but inactive
- **AND** falls back to plain text highlighting
- **AND** no errors in editor

### Requirement: Injection Query Maintenance
The parser SHALL document supported languages in injection queries.

#### Scenario: Query comments document languages
- **WHEN** reading injections.scm
- **THEN** comments list supported languages
- **AND** explain how to add new languages

#### Scenario: Easy to extend with new languages
- **WHEN** new language support needed
- **THEN** can add new injection rule
- **AND** follows existing pattern
- **AND** no grammar changes required

## Performance Requirements

### Requirement: Efficient Multi-Language Parsing
The parser SHALL maintain performance with multiple injected languages.

#### Scenario: Document with many cells
- **WHEN** document has 50+ code cells
- **THEN** parsing remains under 100ms
- **AND** injection doesn't degrade performance

#### Scenario: Large cells
- **WHEN** cell contains 500+ lines of code
- **THEN** injection works efficiently
- **AND** syntax highlighting responsive

## Query File Structure

### Requirement: Injection Query Format
The parser SHALL structure injections.scm according to tree-sitter conventions.

#### Scenario: Executable cell injection pattern
- **WHEN** injections.scm defines cell injection
- **THEN** uses pattern:
  ```scheme
  ((executable_code_cell
    (language_name) @_lang
    (#eq? @_lang "python")
    (cell_content) @injection.content)
   (#set! injection.language "python"))
  ```

#### Scenario: Inline cell injection pattern
- **WHEN** injections.scm defines inline injection
- **THEN** uses pattern:
  ```scheme
  ((inline_code_cell
    (language_name) @_lang
    (#eq? @_lang "python")
    (cell_content) @injection.content)
   (#set! injection.language "python"))
  ```

### Requirement: Combined Language Queries
The parser SHALL support combined language injection contexts.

#### Scenario: All languages in one query file
- **WHEN** injections.scm is loaded
- **THEN** includes rules for all supported languages
- **AND** each language has dedicated pattern
- **AND** patterns don't conflict

## Validation and Debugging

### Requirement: Query Validation
The parser SHALL validate injection queries during build.

#### Scenario: Query syntax checked
- **WHEN** running tree-sitter generate
- **THEN** validates injections.scm syntax
- **AND** reports errors if invalid

#### Scenario: Test injection in practice
- **WHEN** running tree-sitter test
- **THEN** can verify injections work
- **AND** catch regressions

### Requirement: Debugging Support
The parser SHALL support debugging injection queries.

#### Scenario: Query testing with real document
- **WHEN** running tree-sitter parse with injections
- **THEN** can see which injections applied
- **AND** debug injection matching

## Language-Specific Requirements

### Requirement: Language Aliases
The parser SHALL support common language aliases.

#### Scenario: Python alias
- **WHEN** cell uses "python" or "python3"
- **THEN** both receive Python injection
- **AND** mapped to same language parser

#### Scenario: Bash aliases
- **WHEN** cell uses "bash", "sh", or "shell"
- **THEN** all receive Bash injection

### Requirement: Language Case Sensitivity
The parser SHALL handle language name case consistently.

#### Scenario: Case handling defined
- **WHEN** language_name extracted
- **THEN** case handling is consistent
- **AND** documented (lowercase, case-insensitive, etc.)

#### Scenario: Query matches actual usage
- **WHEN** Quarto uses lowercase language names
- **THEN** injection queries match lowercase
- **AND** work with real documents
