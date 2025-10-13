; Pandoc Markdown Locals Queries
; Define scopes and references for semantic analysis and go-to-definition

; ============================================================================
; Link References
; ============================================================================

; Link reference definitions create local "variables"
(link_reference_definition
  (link_label) @definition.link)

; Link references use these "variables"
(link
  (link_label) @reference.link)

; Image references also use link definitions
(image
  (link_label) @reference.link)

; ============================================================================
; Footnotes
; ============================================================================

; Footnote definitions
(footnote_definition
  (footnote_label) @definition.footnote)

; Footnote references
(footnote_reference) @reference.footnote

; ============================================================================
; Scopes
; ============================================================================

; Document root is the top-level scope
(document) @scope

; Fenced divs create nested scopes
(fenced_div) @scope

; Block quotes create nested scopes
(block_quote) @scope

; Lists create nested scopes
(list) @scope

; Headings with content create section scopes
(atx_heading) @scope
(setext_heading) @scope

; ============================================================================
; Notes
; ============================================================================

; This file enables:
; - Go-to-definition for link and footnote references
; - Hover tooltips showing definition content
; - Rename refactoring for references
; - Scope-aware semantic analysis
; - LSP integration for references

; Scopes define where references are valid:
; - Link references are typically document-scoped
; - Footnotes are document-scoped
; - Some editors may implement section-local references
