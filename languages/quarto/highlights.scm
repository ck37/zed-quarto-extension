; Pandoc markdown highlights (from tree-sitter-pandoc-markdown)
; This file uses the grammar's upstream queries to ensure proper highlighting.

(atx_heading
  (inline) @text.title)

(atx_heading_marker) @punctuation.special

(setext_heading
  (inline) @text.title)

(setext_heading_marker) @punctuation.special

(fenced_code_block) @text.literal
(fenced_code_block_delimiter) @punctuation.delimiter
(code_fence_content) @text.literal
(code_fence_line_text) @text.literal
(chunk_option) @comment

(yaml_front_matter_start) @comment
(yaml_front_matter_delimiter) @punctuation.special
(yaml_front_matter_content) @comment

(inline_math
  (math_content)? @string)

(display_math
  (math_content)? @string)

(math_delimiter) @punctuation.special

(footnote_label) @text.reference
(footnote_reference) @text.reference
(inline_footnote) @comment

(pipe_table_header_cell) @text.title

(pipe_table_cell) @string

(pipe_table_alignment_marker) @punctuation.special

(fenced_div_delimiter) @punctuation.special

(list_marker) @punctuation.special
(block_quote_marker) @punctuation.special
(thematic_break) @punctuation.special

; Emphasis and strong emphasis
(emphasis) @text.emphasis
(strong_emphasis) @emphasis.strong
(strikethrough) @text.strike
(highlight) @text.highlight
(subscript) @text.subscript
(superscript) @text.super
(underline) @text.underline
(code_span) @text.literal
(code_span_content) @text.literal

(link
  (link_text) @text.reference
  (link_destination) @text.uri)

(link
  (link_label) @text.reference)

(html_open_tag) @tag
(html_close_tag) @tag
(html_block_content) @text.literal

(image
  (link_text) @text.reference
  (link_destination)? @text.uri)

(image
  (link_label) @text.reference)

(link_reference_definition
  (link_label) @text.reference
  (link_destination)? @text.uri
  (link_title)? @string)

(autolink) @text.uri

; Quarto / Pandoc specific constructs
(citation_group) @text.reference
(citation) @text.reference
(cross_reference) @text.reference
(shortcode) @constant.macro

(html_inline) @tag

(language) @type
(attribute_span
  (inline)? @text)
(attribute_span
  (attribute_list) @property)

(attribute_list) @property
(info_string_text) @string

; Raw blocks and raw inline content (Pandoc-specific)
(raw_block) @text.literal
(raw_block_delimiter) @punctuation.delimiter
(raw_block_content) @text.literal
(raw_inline) @text.literal
(raw_inline_content) @text.literal
(raw_format) @property

; Percent metadata
(percent_metadata_title) @text.title
(percent_metadata_author) @comment
(percent_metadata_date) @comment
