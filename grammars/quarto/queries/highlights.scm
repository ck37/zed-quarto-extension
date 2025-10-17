; Quarto Syntax Highlighting for Zed Editor
;
; This file defines syntax highlighting using Zed-compatible scope names.
; All scopes are validated against Zed's supported token types.
;
; ============================================================================
; DOCUMENTATION & SCOPE REFERENCE
; ============================================================================
;
; Complete list of Zed-supported scopes:
;   docs/zed-syntax-scopes.md
;   - 39 core token types (attribute, boolean, comment, emphasis, etc.)
;   - Hierarchical scope patterns (emphasis.strong, punctuation.delimiter, etc.)
;   - Language-specific conventions (.markup, .rust, etc.)
;
; Scope validation tests:
;   tests/zed_scope_validation.rs
;   - Automated validation that all scopes are Zed-compatible
;   - Prevents nvim-treesitter @markup.* scopes
;   - Ensures recommended markdown scopes are present
;   Run: cargo test --test zed_scope_validation
;
; Scope usage summary:
;   docs/scope-validation-summary.md
;   - Analysis of all scopes used in this file
;   - Historical fixes (e.g., @text.reference -> @link_text.markup)
;   - Maintenance guidelines
;
; Original scope naming decision:
;   docs/scope-naming-decision.md
;
; ============================================================================
; KEY SCOPE CONVENTIONS
; ============================================================================
;
; Zed uses different scope names than nvim-treesitter:
;   Zed                    nvim-treesitter
;   ----                   ---------------
;   @title                 @markup.heading
;   @emphasis              @markup.italic
;   @emphasis.strong       @markup.bold
;   @text.literal          @markup.raw.inline / @markup.raw.block
;   @link_text.markup      @markup.link.label
;   @link_uri.markup       @markup.link.url
;   @comment               @markup.quote (for block quotes)
;   @punctuation.special   @markup.list.marker (for lists)
;   @string                @markup.math (for math content)
;
; IMPORTANT: This file must use Zed scopes, not nvim-treesitter scopes!
;
; ============================================================================
; GRAMMAR INFORMATION
; ============================================================================
;
; Syntax highlighting queries for tree-sitter-quarto
; Grammar: https://github.com/ck37/tree-sitter-quarto
; Based on: openspec/specs/language-injection/spec.md

; ============================================================================
; QUARTO-SPECIFIC HIGHLIGHTS
; ============================================================================

; Executable Code Cells
; ----------------------

(executable_code_cell
  (code_fence_delimiter) @punctuation.delimiter)

(executable_code_cell
  (language_name) @function.builtin)

; Chunk Options
; -------------

(chunk_option_key) @property

(chunk_option_value) @string

"#|" @punctuation.special

; Cross-References (Quarto-specific)
; -----------------------------------

(cross_reference
  "@" @punctuation.special
  type: (reference_type) @constant.builtin
  "-" @punctuation.delimiter
  id: (reference_id) @variable.parameter)

; Inline Code Cells
; -----------------

(inline_code_cell
  (language_name) @function.builtin)

(inline_cell_delimiter) @punctuation.bracket
(inline_cell_brace) @punctuation.bracket

; ============================================================================
; PANDOC MARKDOWN HIGHLIGHTS
; ============================================================================

; Headings
; --------

(atx_heading
  (atx_heading_marker) @punctuation.special
  content: (inline) @text.title)

; Also capture text inside heading inline content
(atx_heading
  content: (inline (text) @text.title))

(atx_heading
  content: (inline (_) @text.title))

(setext_heading
  content: (inline) @text.title
  (setext_heading_marker) @punctuation.special)

(setext_heading
  content: (inline (text) @text.title))

(setext_heading
  content: (inline (_) @text.title))

; Emphasis/Strong
; ---------------

; Capture both the parent node and child text explicitly
(emphasis) @text.emphasis
(emphasis (text) @text.emphasis)
(emphasis (_) @text.emphasis)

(strong_emphasis) @emphasis.strong
(strong_emphasis (text) @emphasis.strong)
(strong_emphasis (_) @emphasis.strong)

; Pandoc Inline Formatting Extensions
; ------------------------------------

(strikethrough) @text.strike
(strikethrough (text) @text.strike)

(highlight) @text.highlight
(highlight (text) @text.highlight)

(subscript) @text.subscript
(subscript (subscript_content) @text.subscript)

(superscript) @text.super
(superscript (superscript_content) @text.super)

; Code
; ----

(code_span) @text.literal

(code_span_delimiter) @punctuation.delimiter

(fenced_code_block) @text.literal

(code_fence_delimiter) @punctuation.delimiter

(info_string) @label

; Links & Images
; --------------

; Capture link components
(link
  text: (link_text) @link_text.markup
  destination: (link_destination) @link_uri.markup)

(link (link_destination) @link_uri.markup)
(link (link_text) @link_text.markup)

; Images use different field names
(image_source) @link_uri.markup

"[" @punctuation.bracket
"]" @punctuation.bracket
"(" @punctuation.bracket
")" @punctuation.bracket
"!" @punctuation.special

; Citations (Pandoc)
; ------------------

(citation
  "@" @punctuation.special
  key: (citation_key) @variable.parameter)

; Block Quotes
; ------------

(block_quote) @comment
(block_quote_marker) @punctuation.special

; Lists
; -----

(list_marker) @punctuation.special

(ordered_list_item
  (list_marker) @punctuation.special)

(unordered_list_item
  (list_marker) @punctuation.special)

; Thematic Breaks
; ---------------

(thematic_break) @punctuation.special

; Fenced Divs
; -----------

(fenced_div
  (fenced_div_delimiter) @punctuation.delimiter)

(fenced_div
  attributes: (attribute_list) @attribute)

; Attributes
; ----------

(attribute_id) @attribute
(attribute_class) @attribute
(attribute_key) @property
(attribute_value) @string

; Shortcodes
; ----------

(shortcode_block
  (shortcode_open) @punctuation.special
  (shortcode_name) @function
  (shortcode_arguments) @parameter
  (shortcode_close) @punctuation.special)

(shortcode_inline
  (shortcode_open) @punctuation.special
  (shortcode_name) @function
  (shortcode_arguments) @parameter
  (shortcode_close) @punctuation.special)

; Math
; ----

(inline_math
  (math_delimiter) @punctuation.delimiter
  (math_content) @string)

(display_math
  (math_delimiter) @punctuation.delimiter
  (math_content) @string)

; YAML Front Matter
; -----------------

(yaml_front_matter
  (yaml_front_matter_start) @punctuation.delimiter
  (yaml_front_matter_delimiter) @punctuation.delimiter)

(yaml_front_matter_content) @embedded

; HTML
; ----

(html_block
  (html_open_tag) @tag
  (html_close_tag) @tag)

(html_block_content) @embedded

; Raw Blocks
; ----------

(raw_block
  (raw_block_delimiter) @punctuation.delimiter)

(raw_block_content) @embedded

; Footnotes
; ---------

(footnote_definition
  (footnote_marker) @punctuation.special)

; Link References
; ---------------

(link_reference_definition
  label: (_) @link_text.markup
  destination: (link_destination) @link_uri.markup)

(link_title) @string

; Pipe Tables
; -----------

(pipe_table_header
  "|" @punctuation.delimiter)

(pipe_table_delimiter
  "|" @punctuation.delimiter
  (table_delimiter_cell) @punctuation.special)

; Note: pipe_table_row is a token with no internal structure,
; so we can't highlight individual delimiters within rows
(pipe_table_row) @none

(table_cell) @none

; Text
; ----
; NOTE: Catch-all (text) @text removed because it overrides parent scopes
; Child text nodes inherit styling from their parent (emphasis, strong, heading, etc.)

; Blank Lines
; -----------

(blank_line) @none

; ============================================================================
; PRIORITY RULES
; ============================================================================

; Higher priority for Quarto constructs
((cross_reference) @constant.builtin
  (#set! "priority" 110))

((executable_code_cell
  (language_name) @function.builtin)
  (#set! "priority" 110))

((chunk_option_key) @property
  (#set! "priority" 110))
