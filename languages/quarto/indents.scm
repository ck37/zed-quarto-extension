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
; NOTE: Dedent queries removed
; ============================================================================

; Zed does not recognize dedent-related capture names in indent queries.
; Previously, this file used closing delimiter queries with @_dedent,
; but these were removed to eliminate warnings in Zed logs.
; See tests/indents_query_validation.rs for details.
