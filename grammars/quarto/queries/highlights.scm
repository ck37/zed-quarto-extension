; Syntax highlighting queries for tree-sitter-quarto
; Based on openspec/specs/language-injection/spec.md
;
; PATCHED FOR ZED COMPATIBILITY
; This file has been modified from the upstream tree-sitter-quarto grammar to use
; Zed-compatible scope names. The following substitutions have been made:
;   @markup.heading -> @text.title
;   @markup.italic -> @text.emphasis
;   @markup.bold -> @emphasis.strong
;   @markup.raw.* -> @text.literal
;   @markup.link.text -> @text.reference
;   @markup.link.url -> @text.uri
;   @markup.quote -> @comment
;   @markup.math.* -> @string
;
; Original source: https://github.com/ck37/tree-sitter-quarto
; Commit: b1b4cbd88fc6f787c660bf52b0e23879a8fc66c2

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
  (atx_heading_marker) @punctuation.special) @text.title

(setext_heading
  (setext_heading_marker) @punctuation.special) @text.title

; Emphasis
; --------

(emphasis) @text.emphasis

(strong_emphasis) @emphasis.strong

; Code
; ----

(code_span) @text.literal

(code_span_delimiter) @punctuation.delimiter

(fenced_code_block) @text.literal

(code_fence_delimiter) @punctuation.delimiter

(info_string) @label

; Links & Images
; --------------

(link
  text: (_) @text.reference
  destination: (link_destination) @text.uri)

(image
  alt: (_) @text.reference
  source: (image_source) @text.uri)

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
  label: (_) @text.reference
  destination: (link_destination) @text.uri)

(link_title) @string

; Pipe Tables
; -----------

(pipe_table_header
  "|" @punctuation.delimiter)

(pipe_table_delimiter
  "|" @punctuation.delimiter
  (table_delimiter_cell) @punctuation.special)

(pipe_table_row
  "|" @punctuation.delimiter)

(table_cell) @none

; Text
; ----

(text) @text

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
