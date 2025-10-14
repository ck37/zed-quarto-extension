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

; Closing delimiters
[
  (fenced_div_delimiter)
  (code_fence_delimiter)
] @dedent
