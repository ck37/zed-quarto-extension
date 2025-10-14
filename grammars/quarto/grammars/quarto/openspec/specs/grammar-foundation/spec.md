# Grammar Foundation

## Purpose
Core grammar infrastructure that extends tree-sitter-pandoc-markdown with Quarto-specific parsing capabilities.

## Requirements

### Requirement: Grammar Extension Architecture
The parser SHALL extend tree-sitter-pandoc-markdown grammar using the "Copy & Extend" strategy.

#### Scenario: Grammar imports base rules
- **WHEN** grammar.js is loaded
- **THEN** it includes all Pandoc Markdown base rules
- **AND** adds Quarto-specific extensions on top

#### Scenario: Base features remain compatible
- **WHEN** parsing standard Pandoc Markdown constructs
- **THEN** they parse identically to tree-sitter-pandoc-markdown
- **AND** no Pandoc features are removed or broken

### Requirement: External Scanner Support
The parser SHALL use an external C scanner for context-sensitive parsing.

#### Scenario: Scanner detects chunk option markers
- **WHEN** parsing `#|` at the start of a cell
- **THEN** the scanner emits CHUNK_OPTION_MARKER token
- **AND** distinguishes it from `#` comment syntax

#### Scenario: Scanner detects cell boundaries
- **WHEN** parsing code fence delimiters
- **THEN** the scanner emits CELL_BOUNDARY token
- **AND** tracks cell context for chunk option detection

### Requirement: Node Type Definitions
The parser SHALL define semantic node types for all Quarto constructs.

#### Scenario: Quarto nodes extend block grammar
- **WHEN** grammar defines _block rule
- **THEN** it includes executable_code_cell as a choice
- **AND** includes all Pandoc block types
- **AND** maintains proper precedence ordering

#### Scenario: Quarto nodes extend inline grammar
- **WHEN** grammar defines inline rules
- **THEN** it includes inline_code_cell as a choice
- **AND** includes cross_reference as a choice
- **AND** maintains proper precedence with citations

### Requirement: Parse Tree Structure
The parser SHALL generate well-formed syntax trees with named nodes and fields.

#### Scenario: Named nodes for semantic constructs
- **WHEN** parsing succeeds
- **THEN** all Quarto constructs have semantic node types
- **AND** nodes use descriptive names (not generic tokens)

#### Scenario: Fields identify important children
- **WHEN** defining complex nodes
- **THEN** important children use field() declarations
- **AND** fields enable query-based navigation

### Requirement: Error Recovery
The parser SHALL recover from errors and continue parsing.

#### Scenario: Partial document parsing
- **WHEN** parsing encounters malformed syntax
- **THEN** parser inserts ERROR node
- **AND** continues parsing subsequent content
- **AND** provides useful error location

#### Scenario: Incomplete constructs
- **WHEN** document ends mid-construct
- **THEN** parser marks incomplete node
- **AND** returns partial tree for editor use

### Requirement: Incremental Parsing
The parser SHALL support incremental reparsing for editor performance.

#### Scenario: Localized edits reparse quickly
- **WHEN** user edits small section of document
- **THEN** parser reuses unchanged portions of tree
- **AND** only reparses affected region
- **AND** completes in <50ms for typical edits

### Requirement: Source Tracking
The parser SHALL track source commit hash from tree-sitter-pandoc-markdown.

#### Scenario: Grammar documents source version
- **WHEN** grammar is copied from tree-sitter-pandoc-markdown
- **THEN** source commit hash is documented in comments
- **AND** documentation includes sync date
- **AND** notes any local modifications

## Performance Requirements

### Requirement: Parse Performance
The parser SHALL parse typical documents in under 100ms.

#### Scenario: Typical document performance
- **WHEN** parsing a 500-line .qmd file
- **THEN** parsing completes in <100ms
- **AND** tree is available for editor queries

#### Scenario: Large document handling
- **WHEN** parsing a 5000-line .qmd file
- **THEN** parsing completes in <1 second
- **AND** remains usable for editor features

## Testing Requirements

### Requirement: Test Infrastructure
The parser SHALL include comprehensive test coverage using tree-sitter corpus format.

#### Scenario: Test files organized by capability
- **WHEN** tests are organized
- **THEN** each capability has dedicated test corpus file
- **AND** tests cover success cases
- **AND** tests cover edge cases
- **AND** tests cover error handling

#### Scenario: Example documents validate features
- **WHEN** examples/sample.qmd exists
- **THEN** it demonstrates all Quarto features
- **AND** parser handles it without errors
- **AND** produces expected AST structure
