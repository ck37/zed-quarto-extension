; Indentation queries for tree-sitter-quarto
; Defines automatic indentation behavior in editors

; ============================================================================
; INDENT
; ============================================================================

; Lists
[
  (ordered_list)
  (unordered_list)
] @indent

; Block quotes
(block_quote) @indent

; Fenced divs
(fenced_div) @indent

; ============================================================================
; DEDENT
; ============================================================================

; Note: Zed doesn't support @dedent capture, so this is disabled
; Closing delimiters would be marked here if supported
; [
;   (fenced_div_delimiter)
;   (code_fence_delimiter)
; ] @dedent
