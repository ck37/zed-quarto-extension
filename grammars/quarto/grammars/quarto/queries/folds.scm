; Code folding queries for tree-sitter-quarto
; Defines regions that can be folded in editors

; ============================================================================
; QUARTO-SPECIFIC FOLDS
; ============================================================================

; Executable Code Cells
; ---------------------

(executable_code_cell
  (code_fence_delimiter) @fold
  (#set! fold.endAt lastChild))

; ============================================================================
; PANDOC MARKDOWN FOLDS
; ============================================================================

; Fenced Code Blocks
; ------------------

(fenced_code_block
  (code_fence_delimiter) @fold
  (#set! fold.endAt lastChild))

; Fenced Divs
; -----------

(fenced_div
  (fenced_div_delimiter) @fold
  (#set! fold.endAt lastChild))

; Block Quotes
; ------------

(block_quote) @fold

; Lists
; -----

(ordered_list) @fold

(unordered_list) @fold

; HTML Blocks
; -----------

(html_block) @fold

; Raw Blocks
; ----------

(raw_block) @fold

; Display Math
; ------------

(display_math) @fold

; ============================================================================
; HEADING-BASED FOLDING
; ============================================================================

; Note: Heading-based folding (sections) typically requires editor-specific
; configuration and may not be fully supported via tree-sitter queries alone.
; Most editors handle this through their own folding logic based on heading levels.
