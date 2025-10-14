; Pandoc Markdown Folding Queries
; Defines foldable regions for better document navigation

; ============================================================================
; Headings
; ============================================================================

; Fold headings with their content
; Note: The heading itself is not folded, only its content
(atx_heading) @fold

(setext_heading) @fold

; ============================================================================
; Code Blocks
; ============================================================================

; Fold fenced code blocks
(fenced_code_block) @fold

; Fold raw blocks
(raw_block) @fold

; ============================================================================
; Containers
; ============================================================================

; Fold fenced divs (Pandoc containers)
(fenced_div) @fold

; Fold block quotes
(block_quote) @fold

; ============================================================================
; Lists
; ============================================================================

; Fold lists (both ordered and unordered)
(list) @fold

; Optionally fold individual list items with nested content
; (list_item) @fold

; ============================================================================
; Front Matter & Metadata
; ============================================================================

; Fold YAML front matter
(yaml_front_matter) @fold

; Fold percent metadata
(percent_metadata) @fold

; ============================================================================
; Definitions
; ============================================================================

; Fold long footnote definitions
(footnote_definition) @fold

; Link reference definitions don't typically need folding (single line)
; But included for completeness
; (link_reference_definition) @fold

; ============================================================================
; Tables
; ============================================================================

; Fold tables (when supported)
(pipe_table) @fold

; ============================================================================
; HTML
; ============================================================================

; Fold HTML blocks
(html_block) @fold
