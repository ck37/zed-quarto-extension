; Pandoc Markdown Tags Queries
; Extract symbols for code navigation and document outline

; ============================================================================
; Headings (Primary navigation structure)
; ============================================================================

; ATX headings as navigable tags
(atx_heading
  (inline) @name) @definition.heading

; Setext headings as navigable tags
(setext_heading
  (inline) @name) @definition.heading

; ============================================================================
; Reference Definitions
; ============================================================================

; Link reference definitions
(link_reference_definition
  (link_label) @name) @definition.link

; ============================================================================
; Footnotes
; ============================================================================

; Footnote definitions
(footnote_definition
  (footnote_label) @name) @definition.footnote

; ============================================================================
; Fenced Divs with IDs
; ============================================================================

; Fenced divs with ID attributes (e.g., :::{#myid})
; Extract the ID from attribute list for navigation
(fenced_div
  (attribute_list) @name
  (#match? @name "#[a-zA-Z]")) @definition.section

; ============================================================================
; Code Blocks with Labels
; ============================================================================

; Labeled code blocks (e.g., with chunk options #| label: my-plot)
; Note: This requires chunk_option content extraction which may need refinement
; (fenced_code_block
;   (chunk_option) @name
;   (#match? @name "label:")) @definition.code

; ============================================================================
; HTML Elements with IDs
; ============================================================================

; HTML blocks might contain id attributes
; This is a basic pattern; may need refinement
; (html_block
;   (html_open_tag) @name
;   (#match? @name "id=")) @definition.html
