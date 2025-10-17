## ADDED Requirements

### Requirement: Strikethrough Formatting
The parser SHALL recognize strikethrough text delimited by double tildes (`~~`).

#### Scenario: Basic strikethrough
- **WHEN** parsing `~~deleted text~~`
- **THEN** produce a strikethrough node
- **AND** node contains delimiters and content
- **AND** content can include whitespace

#### Scenario: Strikethrough with nested formatting
- **WHEN** parsing `~~**bold deleted**~~`
- **THEN** produce strikethrough node with bold child
- **AND** maintain proper nesting structure

#### Scenario: Adjacent strikethrough
- **WHEN** parsing text with multiple strikethrough spans
- **THEN** each `~~text~~` is parsed as separate strikethrough node
- **AND** plain text between spans is preserved

### Requirement: Highlight/Mark Formatting
The parser SHALL recognize highlighted text delimited by double equals (`==`).

#### Scenario: Basic highlight
- **WHEN** parsing `==important text==`
- **THEN** produce a highlight node
- **AND** node contains delimiters and content
- **AND** content can include whitespace

#### Scenario: Highlight with nested formatting
- **WHEN** parsing `==*emphasized important*==`
- **THEN** produce highlight node with emphasis child
- **AND** maintain proper nesting structure

### Requirement: Subscript Formatting
The parser SHALL recognize subscript text delimited by single tildes (`~`), following Pandoc's subscript rules.

#### Scenario: Chemical formula subscript
- **WHEN** parsing `H~2~O`
- **THEN** produce subscript node for `2`
- **AND** preserve surrounding text `H` and `O`

#### Scenario: Multi-character subscript
- **WHEN** parsing `C~6~H~12~O~6~`
- **THEN** produce three separate subscript nodes
- **AND** preserve text between subscripts

#### Scenario: Subscript with whitespace after opening tilde
- **WHEN** parsing `~ not subscript~`
- **THEN** treat tildes as plain text
- **AND** do not create subscript node

#### Scenario: Disambiguation from strikethrough
- **WHEN** parsing `~~strikethrough~~` vs `~subscript~`
- **THEN** double tildes create strikethrough
- **AND** single tildes create subscript
- **AND** no ambiguity in parsing

### Requirement: Superscript Formatting
The parser SHALL recognize superscript text delimited by carets (`^`), following Pandoc's superscript rules.

#### Scenario: Mathematical superscript
- **WHEN** parsing `x^2^`
- **THEN** produce superscript node for `2`
- **AND** preserve surrounding text

#### Scenario: Multi-character superscript
- **WHEN** parsing `E=mc^2^`
- **THEN** produce superscript node for `2`
- **AND** preserve equation text

#### Scenario: Superscript with whitespace after opening caret
- **WHEN** parsing `^ not superscript^`
- **THEN** treat carets as plain text
- **AND** do not create superscript node

#### Scenario: Disambiguation from footnote reference
- **WHEN** parsing `[^1]` vs `x^2^`
- **THEN** brackets with caret create footnote reference
- **AND** standalone carets create superscript
- **AND** no ambiguity in parsing

### Requirement: Inline Formatting Integration
The parser SHALL integrate new formatting types into existing inline parsing rules.

#### Scenario: Formatting within paragraphs
- **WHEN** parsing paragraph with mixed formatting
- **THEN** all inline formatting types are recognized
- **AND** formatting can be nested appropriately
- **AND** formatting can appear adjacent to each other

#### Scenario: Formatting with punctuation
- **WHEN** parsing formatted text adjacent to punctuation
- **THEN** punctuation does not interfere with delimiter matching
- **AND** formatted spans are correctly bounded

### Requirement: Semantic Node Types
The parser SHALL produce distinct node types for each formatting style.

#### Scenario: Node type distinction
- **WHEN** querying parse tree
- **THEN** strikethrough produces `(strikethrough)` node
- **AND** highlight produces `(highlight)` node
- **AND** subscript produces `(subscript)` node
- **AND** superscript produces `(superscript)` node

#### Scenario: Node structure
- **WHEN** examining formatting nodes
- **THEN** each node contains opening delimiter
- **AND** content as child nodes
- **AND** closing delimiter

### Requirement: Syntax Highlighting
The parser SHALL provide highlight queries for semantic scoping of inline formatting.

#### Scenario: Modern semantic scopes
- **WHEN** using queries/highlights.scm
- **THEN** strikethrough maps to @markup.strikethrough
- **AND** highlight maps to @markup.mark
- **AND** subscript maps to @markup.subscript
- **AND** superscript maps to @markup.superscript

#### Scenario: Zed editor compatibility
- **WHEN** using queries/zed/highlights.scm
- **THEN** strikethrough maps to @text.strike
- **AND** highlight maps to @text.highlight
- **AND** subscript maps to @text.subscript
- **AND** superscript maps to @text.super

### Requirement: Error Handling
The parser SHALL handle malformed inline formatting gracefully.

#### Scenario: Unmatched delimiters
- **WHEN** parsing text with unclosed formatting delimiters
- **THEN** parser continues parsing
- **AND** unclosed delimiters treated as plain text
- **AND** subsequent content parsed normally

#### Scenario: Nested delimiter conflicts
- **WHEN** parsing `~~text~with~nested~tildes~~`
- **THEN** outermost delimiters define span
- **AND** inner tildes follow subscript rules if applicable
- **AND** no parse errors generated

### Requirement: Performance
The parser SHALL maintain fast parsing performance with inline formatting.

#### Scenario: Large documents with extensive formatting
- **WHEN** parsing document with 500+ formatted spans
- **THEN** parsing completes in <100ms
- **AND** incremental reparsing remains efficient
- **AND** memory usage stays reasonable
