# Cross-References

## Purpose
Distinguish Quarto cross-references (`@fig-plot`, `@tbl-data`) from citations (`@smith2020`) at parse time for semantic highlighting and navigation.

## Requirements

### Requirement: Cross-Reference Pattern Recognition
The parser SHALL recognize cross-references with type prefixes.

#### Scenario: Figure reference
- **WHEN** parsing `@fig-plot`
- **THEN** creates cross_reference node
- **AND** captures type as "fig"
- **AND** captures id as "plot"

#### Scenario: Table reference
- **WHEN** parsing `@tbl-data`
- **THEN** creates cross_reference node
- **AND** captures type as "tbl"
- **AND** captures id as "data"

#### Scenario: Equation reference
- **WHEN** parsing `@eq-linear`
- **THEN** creates cross_reference node
- **AND** captures type as "eq"
- **AND** captures id as "linear"

#### Scenario: Section reference
- **WHEN** parsing `@sec-intro`
- **THEN** creates cross_reference node
- **AND** captures type as "sec"
- **AND** captures id as "intro"

#### Scenario: Listing reference
- **WHEN** parsing `@lst-code`
- **THEN** creates cross_reference node
- **AND** captures type as "lst"
- **AND** captures id as "code"

### Requirement: Citation Pattern Recognition
The parser SHALL recognize citations without type prefixes as distinct from cross-references.

#### Scenario: Author-year citation
- **WHEN** parsing `@smith2020`
- **THEN** creates citation node
- **AND** not cross_reference node
- **AND** captures id as "smith2020"

#### Scenario: Citation without hyphen
- **WHEN** citation id has no hyphen
- **THEN** creates citation node
- **AND** distinguished from cross-reference

#### Scenario: Citation with underscore
- **WHEN** parsing `@smith_jones_2020`
- **THEN** creates citation node
- **AND** underscores allowed in citation ids

### Requirement: Type Prefix Distinction
The parser SHALL distinguish cross-reference type prefixes from regular citation ids.

#### Scenario: Known type prefixes are cross-references
- **WHEN** @ is followed by fig-, tbl-, eq-, sec-, or lst-
- **THEN** parsed as cross_reference
- **AND** not as citation

#### Scenario: Unknown prefixes are citations
- **WHEN** parsing `@author-2020`
- **THEN** "author" is not a known type prefix
- **AND** parsed as citation
- **AND** not as cross_reference

#### Scenario: Type prefix is case-sensitive
- **WHEN** parsing `@Fig-plot` or `@FIG-plot`
- **THEN** may not match type prefix pattern
- **AND** behavior defined by grammar rules

### Requirement: Reference ID Parsing
The parser SHALL parse reference IDs following type prefixes.

#### Scenario: Simple alphanumeric ID
- **WHEN** parsing `@fig-plot1`
- **THEN** id is "plot1"
- **AND** includes letters and numbers

#### Scenario: ID with hyphens
- **WHEN** parsing `@fig-my-plot-1`
- **THEN** id is "my-plot-1"
- **AND** hyphens allowed in id

#### Scenario: ID with underscores
- **WHEN** parsing `@fig-my_plot`
- **THEN** id is "my_plot"
- **AND** underscores allowed in id

#### Scenario: ID boundary at punctuation
- **WHEN** parsing `See @fig-plot.`
- **THEN** id is "plot"
- **AND** period terminates reference
- **AND** not included in id

### Requirement: Node Structure
The parser SHALL provide structured nodes for cross-references with separate type and id fields.

#### Scenario: Type field extraction
- **WHEN** querying cross_reference node
- **THEN** can extract type field
- **AND** type is one of: fig, tbl, eq, sec, lst

#### Scenario: ID field extraction
- **WHEN** querying cross_reference node
- **THEN** can extract id field
- **AND** id is full identifier after hyphen

### Requirement: Inline Context
The parser SHALL parse cross-references and citations in inline contexts.

#### Scenario: Reference in paragraph
- **WHEN** parsing `See @fig-plot for details.`
- **THEN** cross_reference is child of paragraph
- **AND** surrounded by text nodes

#### Scenario: Multiple references in sentence
- **WHEN** parsing `See @fig-plot and @tbl-data.`
- **THEN** creates two cross_reference nodes
- **AND** both in same paragraph

#### Scenario: Mixed references and citations
- **WHEN** parsing `@fig-plot shows results from @smith2020.`
- **THEN** creates one cross_reference node
- **AND** creates one citation node
- **AND** types correctly distinguished

## Syntax Highlighting Support

### Requirement: Enables Distinct Styling
The parser SHALL structure references to enable distinct highlighting.

#### Scenario: Cross-references styled differently
- **WHEN** editor applies syntax highlighting
- **THEN** cross_reference nodes can have unique style
- **AND** different from citation style

#### Scenario: Type prefix highlighted
- **WHEN** highlighting cross-reference
- **THEN** type prefix can be styled separately
- **AND** from reference id

## Navigation Support

### Requirement: Enables Jump-to-Definition
The parser SHALL provide sufficient structure for jump-to-definition features.

#### Scenario: Navigate to figure definition
- **WHEN** user jumps from `@fig-plot`
- **THEN** editor can extract type "fig" and id "plot"
- **AND** search for label matching "fig-plot"
- **AND** navigate to figure definition

#### Scenario: Navigate to table definition
- **WHEN** user jumps from `@tbl-data`
- **THEN** editor can search for label "tbl-data"
- **AND** find table caption or chunk option

## Edge Cases

### Requirement: Escaped At Signs
The parser SHALL handle escaped @ symbols correctly.

#### Scenario: Literal at sign
- **WHEN** parsing `\@fig-plot`
- **THEN** escaped @ is literal text
- **AND** not parsed as cross-reference

#### Scenario: Email addresses
- **WHEN** parsing `email@example.com`
- **THEN** not parsed as citation or cross-reference
- **AND** parsed as plain text or link

### Requirement: Ambiguous Patterns
The parser SHALL handle ambiguous patterns consistently.

#### Scenario: Short IDs
- **WHEN** parsing `@fig-a`
- **THEN** parsed as cross_reference
- **AND** single-letter id is valid

#### Scenario: Numeric IDs
- **WHEN** parsing `@fig-1`
- **THEN** parsed as cross_reference
- **AND** numeric-only id is valid

#### Scenario: Empty ID after prefix
- **WHEN** parsing `@fig-`
- **THEN** parser handles incomplete reference
- **AND** may create ERROR or incomplete node

## Validation Support

### Requirement: Enables Reference Validation
The parser SHALL structure references to support validation.

#### Scenario: Undefined reference detection
- **WHEN** language server validates references
- **THEN** can extract all cross_reference nodes
- **AND** check if label exists in document
- **AND** warn on undefined references

#### Scenario: Reference type mismatch
- **WHEN** `@fig-plot` references table label
- **THEN** validator can check target type
- **AND** warn on type mismatch

#### Scenario: Autocomplete reference IDs
- **WHEN** user types `@fig-`
- **THEN** editor can query existing labels
- **AND** suggest matching figure labels
- **AND** filter by type prefix

## Compatibility Requirements

### Requirement: Citation Format Compatibility
The parser SHALL remain compatible with Pandoc citation syntax.

#### Scenario: Square bracket citations
- **WHEN** parsing `[@smith2020]`
- **THEN** citation parsed correctly
- **AND** compatible with Pandoc

#### Scenario: Suppress author citations
- **WHEN** parsing `[-@smith2020]`
- **THEN** citation parsed correctly
- **AND** compatible with Pandoc

#### Scenario: Citation with locator
- **WHEN** parsing `[@smith2020, p. 42]`
- **THEN** citation parsed correctly
- **AND** locator preserved
