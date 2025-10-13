; Pandoc Markdown Language Injection Queries
; Quarto-specific patterns for code chunks with chunk options

; ============================================================================
; Inline Content (Pandoc inline grammar)
; ============================================================================

; Inject inline grammar for all inline content
; This requires Zed PR #40063 for extension-to-extension grammar injection
((inline) @injection.content
  (#set! injection.language "pandoc_markdown_inline"))

; ============================================================================
; YAML Front Matter
; ============================================================================

(yaml_front_matter
  (yaml_front_matter_content) @injection.content
  (#set! injection.language "yaml"))

; ============================================================================
; Quarto Code Chunks (with chunk options like {python} or {r, echo=FALSE})
; ============================================================================

; These patterns match Quarto's special syntax with curly braces and chunk options

; Python chunks: {python}, {python, echo=TRUE}, etc.
((fenced_code_block
  (info_string) @injection.language
  (code_fence_content) @injection.content)
  (#match? @injection.language "(?i)^\\{?python(?:[\\s,}]|$)")
  (#set! injection.language "python"))

; R chunks: {r}, {r, echo=FALSE}, etc.
((fenced_code_block
  (info_string) @injection.language
  (code_fence_content) @injection.content)
  (#match? @injection.language "(?i)^\\{?r(?:[\\s,}]|$)")
  (#set! injection.language "r"))

; Julia chunks: {julia}, {julia, eval=TRUE}, etc.
((fenced_code_block
  (info_string) @injection.language
  (code_fence_content) @injection.content)
  (#match? @injection.language "(?i)^\\{?julia(?:[\\s,}]|$)")
  (#set! injection.language "julia"))

; SQL chunks: {sql}, {sql, connection=con}, etc.
((fenced_code_block
  (info_string) @injection.language
  (code_fence_content) @injection.content)
  (#match? @injection.language "(?i)^\\{?sql(?:[\\s,}]|$)")
  (#set! injection.language "sql"))

; ============================================================================
; Standard Markdown Code Blocks (simple language names)
; ============================================================================

; Generic fallback for simple language names (javascript, rust, etc.)
; This handles standard markdown code blocks without Quarto chunk options
(fenced_code_block
  (info_string
    (language) @injection.language)?
  (code_fence_content) @injection.content)

; ============================================================================
; Math (LaTeX)
; ============================================================================

; Display math - inject LaTeX
((display_math
  (math_content) @injection.content)
  (#set! injection.language "latex"))

; Inline math - inject LaTeX
((inline_math
  (math_content) @injection.content)
  (#set! injection.language "latex"))

; ============================================================================
; Raw Blocks with Format Markers
; ============================================================================

; Raw HTML blocks
((raw_block
  (raw_format) @_format
  (raw_block_content) @injection.content)
  (#eq? @_format "{=html}")
  (#set! injection.language "html"))

; Raw LaTeX blocks
((raw_block
  (raw_format) @_format
  (raw_block_content) @injection.content)
  (#eq? @_format "{=latex}")
  (#set! injection.language "latex"))

; Raw TeX blocks
((raw_block
  (raw_format) @_format
  (raw_block_content) @injection.content)
  (#eq? @_format "{=tex}")
  (#set! injection.language "latex"))
