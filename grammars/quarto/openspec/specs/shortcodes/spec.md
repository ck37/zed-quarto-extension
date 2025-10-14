# Shortcodes

## Purpose
Parse Quarto shortcodes (`{{< name args >}}`) to enable syntax highlighting, autocomplete, and validation of shortcode usage.

## Requirements

### Requirement: Basic Shortcode Syntax
The parser SHALL parse shortcodes with double brace and angle bracket delimiters.

#### Scenario: Simple shortcode without arguments
- **WHEN** parsing `{{< video >}}`
- **THEN** creates shortcode node
- **AND** captures name as "video"
- **AND** no arguments captured

#### Scenario: Shortcode with single argument
- **WHEN** parsing `{{< video https://example.com/video.mp4 >}}`
- **THEN** creates shortcode node
- **AND** captures name as "video"
- **AND** captures arguments as "https://example.com/video.mp4"

#### Scenario: Shortcode with multiple arguments
- **WHEN** parsing `{{< embed notebook.ipynb#fig-plot >}}`
- **THEN** creates shortcode node
- **AND** captures name as "embed"
- **AND** captures arguments as "notebook.ipynb#fig-plot"

### Requirement: Block vs Inline Shortcodes
The parser SHALL distinguish block-level from inline shortcodes based on context.

#### Scenario: Block-level shortcode
- **WHEN** parsing shortcode on its own line
- **THEN** creates shortcode_block node
- **AND** consumes newline after closing delimiter

#### Scenario: Inline shortcode
- **WHEN** parsing shortcode within paragraph
- **THEN** creates shortcode_inline node
- **AND** does not consume trailing content

#### Scenario: Shortcode at start of line in paragraph
- **WHEN** line starts with shortcode but has trailing text
- **THEN** may create inline or block shortcode
- **AND** behavior is consistent with Quarto

### Requirement: Shortcode Name Parsing
The parser SHALL parse shortcode names as identifiers.

#### Scenario: Simple name
- **WHEN** parsing `{{< video >}}`
- **THEN** name is "video"

#### Scenario: Name with hyphens
- **WHEN** parsing `{{< my-shortcode >}}`
- **THEN** name is "my-shortcode"
- **AND** hyphens allowed in name

#### Scenario: Name with underscores
- **WHEN** parsing `{{< my_shortcode >}}`
- **THEN** name is "my_shortcode"
- **AND** underscores allowed in name

#### Scenario: Name must start with letter
- **WHEN** parsing `{{< 123invalid >}}`
- **THEN** may fail to parse as shortcode
- **OR** create error node

### Requirement: Shortcode Arguments Parsing
The parser SHALL capture shortcode arguments as raw text.

#### Scenario: URL argument
- **WHEN** parsing `{{< video https://example.com/video.mp4 >}}`
- **THEN** arguments is "https://example.com/video.mp4"
- **AND** preserves full URL

#### Scenario: File path argument
- **WHEN** parsing `{{< include _content.qmd >}}`
- **THEN** arguments is "_content.qmd"

#### Scenario: Multiple space-separated arguments
- **WHEN** parsing `{{< embed notebook.ipynb cell=code >}}`
- **THEN** arguments is "notebook.ipynb cell=code"
- **AND** parser doesn't split arguments

#### Scenario: Arguments with special characters
- **WHEN** arguments contain `#`, `=`, `/`, `.`, `-`
- **THEN** all characters preserved
- **AND** captured as raw string

#### Scenario: Empty arguments
- **WHEN** parsing `{{< shortcode >}}`
- **THEN** arguments field is empty/optional
- **AND** shortcode still valid

### Requirement: Delimiter Handling
The parser SHALL match shortcode delimiters exactly.

#### Scenario: Opening delimiter with optional space
- **WHEN** parsing `{{<video>}}` or `{{< video >}}`
- **THEN** both parse correctly
- **AND** space after `{{<` is optional

#### Scenario: Closing delimiter with optional space
- **WHEN** parsing `{{< video>}}` or `{{< video >}}`
- **THEN** both parse correctly
- **AND** space before `>}}` is optional

#### Scenario: Requires angle brackets
- **WHEN** parsing `{{ video }}` without angle brackets
- **THEN** not parsed as shortcode
- **AND** treated as regular text

### Requirement: Common Quarto Shortcodes
The parser SHALL handle all standard Quarto shortcodes.

#### Scenario: video shortcode
- **WHEN** parsing `{{< video url >}}`
- **THEN** name is "video"
- **AND** arguments captured

#### Scenario: embed shortcode
- **WHEN** parsing `{{< embed notebook.ipynb#fig-plot >}}`
- **THEN** name is "embed"
- **AND** fragment identifier preserved

#### Scenario: include shortcode
- **WHEN** parsing `{{< include _content.qmd >}}`
- **THEN** name is "include"
- **AND** file path captured

#### Scenario: meta shortcode
- **WHEN** parsing `{{< meta title >}}`
- **THEN** name is "meta"
- **AND** metadata key captured

#### Scenario: var shortcode
- **WHEN** parsing `{{< var variable.name >}}`
- **THEN** name is "var"
- **AND** variable path captured

### Requirement: Node Structure
The parser SHALL provide structured nodes for shortcodes with separate name and arguments fields.

#### Scenario: Name field extraction
- **WHEN** querying shortcode node
- **THEN** can extract name field
- **AND** name is identifier

#### Scenario: Arguments field extraction
- **WHEN** querying shortcode node
- **THEN** can extract arguments field
- **AND** arguments is raw string

#### Scenario: Block vs inline distinction
- **WHEN** querying shortcode
- **THEN** can distinguish shortcode_block from shortcode_inline
- **AND** node type indicates context

## Syntax Highlighting Support

### Requirement: Enables Distinct Styling
The parser SHALL structure shortcodes to enable distinct highlighting.

#### Scenario: Delimiters styled separately
- **WHEN** editor applies syntax highlighting
- **THEN** `{{<` and `>}}` can be styled as punctuation
- **AND** distinct from shortcode name

#### Scenario: Shortcode name highlighted
- **WHEN** highlighting shortcode
- **THEN** name can be styled as function/keyword
- **AND** different from arguments

#### Scenario: Arguments highlighted
- **WHEN** highlighting shortcode
- **THEN** arguments can be styled as string/parameter
- **AND** different from name

## Validation Support

### Requirement: Enables Shortcode Validation
The parser SHALL structure shortcodes to support validation.

#### Scenario: Known shortcode names
- **WHEN** language server validates shortcodes
- **THEN** can extract all shortcode names
- **AND** check against known shortcodes
- **AND** warn on unknown shortcode names

#### Scenario: Required arguments validation
- **WHEN** language server checks shortcode
- **THEN** can extract arguments field
- **AND** validate required arguments present
- **AND** warn on missing required arguments

#### Scenario: Argument format validation
- **WHEN** validating video shortcode
- **THEN** can check argument is valid URL
- **AND** warn on invalid URL format

## Edge Cases

### Requirement: Malformed Shortcodes
The parser SHALL handle malformed shortcodes gracefully.

#### Scenario: Missing closing delimiter
- **WHEN** parsing `{{< video https://example.com`
- **THEN** may create ERROR node
- **AND** continues parsing subsequent content

#### Scenario: Unclosed angle bracket
- **WHEN** parsing `{{< video https://example.com }}`
- **THEN** may parse as shortcode or error
- **AND** handles gracefully

#### Scenario: Empty shortcode
- **WHEN** parsing `{{< >}}`
- **THEN** may create error or empty shortcode
- **AND** continues parsing

### Requirement: Nested Delimiters
The parser SHALL handle content that looks like delimiters.

#### Scenario: Angle brackets in arguments
- **WHEN** arguments contain `<` or `>`
- **THEN** may cause parsing issues
- **AND** behavior documented

#### Scenario: Curly braces in arguments
- **WHEN** arguments contain `{` or `}`
- **THEN** should not interfere with delimiter matching
- **AND** braces preserved in arguments

### Requirement: Whitespace Handling
The parser SHALL handle various whitespace patterns.

#### Scenario: No spaces around name
- **WHEN** parsing `{{<video>}}`
- **THEN** parses correctly
- **AND** whitespace optional

#### Scenario: Multiple spaces
- **WHEN** parsing `{{<  video  url  >}}`
- **THEN** parses correctly
- **AND** extra spaces preserved in arguments

#### Scenario: Newlines in arguments
- **WHEN** shortcode spans multiple lines
- **THEN** may not parse correctly
- **AND** single-line shortcodes preferred

## Integration Requirements

### Requirement: Works in Block Context
The parser SHALL parse block-level shortcodes as block nodes.

#### Scenario: Shortcode as standalone paragraph
- **WHEN** shortcode on its own line
- **THEN** creates shortcode_block
- **AND** is child of document/_block

#### Scenario: Multiple shortcodes in sequence
- **WHEN** document has multiple block shortcodes
- **THEN** each creates separate shortcode_block
- **AND** all parsed correctly

### Requirement: Works in Inline Context
The parser SHALL parse inline shortcodes within paragraphs.

#### Scenario: Shortcode in paragraph
- **WHEN** parsing `See {{< video url >}} for details.`
- **THEN** creates shortcode_inline
- **AND** is child of paragraph/inline

#### Scenario: Multiple inline shortcodes
- **WHEN** paragraph has multiple shortcodes
- **THEN** each creates shortcode_inline node
- **AND** all in same paragraph

### Requirement: Works with Other Constructs
The parser SHALL parse shortcodes alongside other Quarto features.

#### Scenario: Shortcode near code cell
- **WHEN** document has shortcode and executable cell
- **THEN** both parse correctly
- **AND** no interference

#### Scenario: Shortcode in div
- **WHEN** shortcode inside fenced div
- **THEN** parses as child of div
- **AND** context preserved

## Performance Requirements

### Requirement: Efficient Shortcode Parsing
The parser SHALL parse shortcodes efficiently.

#### Scenario: Many shortcodes in document
- **WHEN** document has 50+ shortcodes
- **THEN** parsing remains fast
- **AND** no performance degradation

#### Scenario: Long argument strings
- **WHEN** shortcode has 500+ character arguments
- **THEN** parses efficiently
- **AND** captures full arguments

## Autocomplete Support

### Requirement: Enables Shortcode Autocomplete
The parser SHALL structure shortcodes to support autocomplete.

#### Scenario: Autocomplete shortcode names
- **WHEN** user types `{{< `
- **THEN** editor can suggest known shortcode names
- **AND** based on available shortcodes

#### Scenario: Autocomplete arguments
- **WHEN** user types `{{< video `
- **THEN** editor can suggest argument patterns
- **AND** based on shortcode requirements

## Documentation Requirements

### Requirement: Shortcodes Documented
The parser SHALL document shortcode syntax and usage.

#### Scenario: Syntax examples
- **WHEN** consulting documentation
- **THEN** shortcode syntax is explained
- **AND** examples provided

#### Scenario: Supported shortcodes listed
- **WHEN** consulting documentation
- **THEN** common shortcode names documented
- **AND** argument formats explained
