# Inline Attributes Test Coverage

## ADDED Requirements

### Requirement: ID Attribute Parsing
The parser MUST correctly parse ID attributes using the `{#id}` syntax within attribute lists.

#### Scenario: Single ID attribute
- **GIVEN** markdown content `[text]{#myid}`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - An attribute_list node
  - An attribute_id field with value `myid`

#### Scenario: ID attribute with hyphens and underscores
- **GIVEN** markdown content `[text]{#my-custom_id-123}`
- **WHEN** parsing the content
- **THEN** the AST SHALL correctly parse the full ID
- **AND** preserve hyphens and underscores in the identifier

### Requirement: Class Attribute Parsing
The parser MUST correctly parse class attributes using the `.classname` syntax within attribute lists.

#### Scenario: Single class attribute
- **GIVEN** markdown content `[text]{.myclass}`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - An attribute_list node
  - An attribute_class field with value `myclass`

#### Scenario: Multiple class attributes
- **GIVEN** markdown content `[text]{.class1 .class2 .class3}`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - Multiple attribute_class fields
  - All class names preserved in order

#### Scenario: Class with period in name
- **GIVEN** markdown content `[text]{.my.class.name}`
- **WHEN** parsing the content
- **THEN** the parser SHALL handle the period correctly
- **AND** treat it as a single class name

### Requirement: Key-Value Attribute Parsing
The parser MUST correctly parse key-value attributes in both quoted and unquoted forms.

#### Scenario: Unquoted key-value
- **GIVEN** markdown content `[text]{key=value}`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - A key-value attribute pair
  - Key: `key`
  - Value: `value`

#### Scenario: Quoted key-value
- **GIVEN** markdown content `[text]{key="value with spaces"}`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - A key-value attribute pair with quoted value
  - Spaces preserved in the value

#### Scenario: Multiple key-value attributes
- **GIVEN** markdown content `[text]{k1="v1" k2="v2"}`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - Multiple key-value pairs
  - All keys and values preserved

#### Scenario: Escaped quote in value
- **GIVEN** markdown content `[text]{key="val\"ue"}`
- **WHEN** parsing the content
- **THEN** the AST SHALL:
  - Parse the escaped quote correctly
  - Preserve the quote in the value

### Requirement: Mixed Attribute Parsing
The parser MUST correctly parse attribute lists containing multiple attribute types.

#### Scenario: ID, class, and key-value combined
- **GIVEN** markdown content `[text]{#id .class1 .class2 key="value"}`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - An attribute_id field
  - Multiple attribute_class fields
  - A key-value pair
  - All in the correct order

#### Scenario: Empty attribute list
- **GIVEN** markdown content `[text]{}`
- **WHEN** parsing the content
- **THEN** the AST SHALL contain:
  - An attribute_list node
  - No attribute values

#### Scenario: Attribute list with whitespace
- **GIVEN** markdown content `[text]{ #id .class key="value" }`
- **WHEN** parsing the content
- **THEN** the parser SHALL:
  - Ignore leading/trailing whitespace
  - Parse all attributes correctly

### Requirement: Inline Attributes Test Coverage
The test suite MUST include comprehensive tests for all attribute syntax variations.

#### Scenario: Inline attributes test file exists
- **GIVEN** the test corpus directory `test/corpus/`
- **WHEN** checking for attribute tests
- **THEN** a file `inline-attributes.txt` SHALL exist
- **AND** it SHALL contain at least 10 test cases

#### Scenario: All attribute tests pass
- **GIVEN** the inline attributes test corpus
- **WHEN** running `npx tree-sitter test`
- **THEN** all attribute tests SHALL pass
- **AND** the success rate SHALL remain 100%
