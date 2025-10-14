; Pandoc Markdown Inline Highlighting Queries
;
; NOTE: This file overrides the upstream grammar's highlights.scm to use Zed-compatible
; scope names (@text.*, @emphasis.strong) instead of nvim-treesitter conventions (@markup.*).
; See docs/scope-naming-decision.md for rationale and migration path.

; ============================================================================
; Emphasis & Strong
; ============================================================================

; Strategy: Capture container nodes directly (emphasis, strong_emphasis)
; The grammar handles nested emphasis properly, avoiding overlapping ranges.
; This approach is simpler than capturing text nodes directly and matches
; the upstream tree-sitter-pandoc-markdown structure.
(emphasis) @text.emphasis
(strong_emphasis) @emphasis.strong

; Emphasis delimiters (asterisks, underscores)
(emphasis_delimiter) @punctuation.delimiter.emphasis

; ============================================================================
; Code
; ============================================================================

(code_span) @text.literal
(code_span_content) @text.literal

; Raw inline with format markers
(raw_inline) @text.literal
(raw_inline_content) @text.literal
(raw_format) @property

; ============================================================================
; Links & Images
; ============================================================================

(link
  (link_text) @text.reference
  (link_destination) @text.uri)

(link
  (link_label) @text.reference)

(image
  (link_text) @text.reference
  (link_destination)? @text.uri)

(image
  (link_label) @text.reference)

(autolink) @text.uri

; ============================================================================
; Pandoc Extensions
; ============================================================================

; Citations - using @constant (semantically better than @string)
(citation_group) @constant
(citation) @constant

; Cross-references
(cross_reference) @constant

; Footnotes
(footnote_reference) @constant
(inline_footnote) @constant

; ============================================================================
; Special Formatting
; ============================================================================

(strikethrough) @text.strike
(highlight) @text.highlight
(subscript) @text.subscript
(superscript) @text.super
(underline) @text.underline

; ============================================================================
; Attributes
; ============================================================================

(attribute_span
  (inline)? @text)
(attribute_span
  (attribute_list) @property)

(attribute_list) @property

; ============================================================================
; Math
; ============================================================================

(inline_math
  (math_content)? @string)

(math_delimiter) @punctuation.delimiter.math

; ============================================================================
; HTML
; ============================================================================

(html_inline) @tag
