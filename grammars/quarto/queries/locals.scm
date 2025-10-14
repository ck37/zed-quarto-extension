; Locals queries for tree-sitter-quarto
; Defines scopes and local definitions for semantic analysis

; ============================================================================
; SCOPES
; ============================================================================

(document) @scope

(executable_code_cell) @scope

; ============================================================================
; DEFINITIONS
; ============================================================================

; Chunk labels define identifiers
(chunk_option
  key: (chunk_option_key) @_key
  (#eq? @_key "label")
  value: (chunk_option_value) @definition.label)

; Link reference definitions
(link_reference_definition
  label: (_) @definition.reference)

; Footnote definitions
(footnote_definition
  marker: (_) @definition.footnote)

; ============================================================================
; REFERENCES
; ============================================================================

; Cross-references reference labels
(cross_reference
  id: (reference_id) @reference.label)

; Citations reference bibliography
(citation
  key: (citation_key) @reference.citation)
