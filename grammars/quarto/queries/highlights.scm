; Quarto Syntax Highlighting (tree-sitter-quarto)
;
; This file uses modern nvim-treesitter scope naming conventions (@markup.*)
; for compatibility with standard tree-sitter tooling and editors like Neovim.
;
; For Zed editor compatibility, use queries/zed/highlights.scm which provides
; legacy scope names (@text.*, @emphasis.strong) that work with Zed's current themes.

; Syntax highlighting queries for tree-sitter-quarto
; Based on openspec/specs/language-injection/spec.md

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
  content: (inline) @markup.heading)

(setext_heading
  content: (inline) @markup.heading
  (setext_heading_marker) @punctuation.special)

; Emphasis/Strong
; ---------------

(emphasis) @markup.italic

(strong_emphasis) @markup.bold

; Inline Formatting (Pandoc extensions)
; -------------------------------------

(strikethrough) @markup.strikethrough

(highlight) @markup.mark

(subscript) @markup.subscript

(superscript) @markup.superscript

; Code
; ----

(code_span) @markup.raw.inline

(code_span_delimiter) @punctuation.delimiter

(fenced_code_block) @markup.raw.block

(code_fence_delimiter) @punctuation.delimiter

(info_string) @label

; Links & Images
; --------------

(link
  text: (_) @markup.link.label
  destination: (link_destination) @markup.link.url)

(image
  alt: (_) @markup.link.label
  source: (image_source) @markup.link.url)

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

(block_quote) @markup.quote
(block_quote_marker) @punctuation.special

; Lists
; -----

(list_marker) @markup.list.marker

(ordered_list_item
  (list_marker) @markup.list.marker)

(unordered_list_item
  (list_marker) @markup.list.marker)

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
  (math_content) @markup.math.inline)

(display_math
  (math_delimiter) @punctuation.delimiter
  (math_content) @markup.math.block)

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
  label: (_) @markup.link.label
  destination: (link_destination) @markup.link.url)

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
