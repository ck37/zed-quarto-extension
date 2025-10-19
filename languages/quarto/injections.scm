; Language injection queries for tree-sitter-quarto
; Based on openspec/specs/language-injection/spec.md
;
; Enables syntax highlighting for code within executable cells

; ============================================================================
; EXECUTABLE CODE CELLS
; ============================================================================

; Python
; ------

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "python")
  content: (cell_content) @injection.content)
 (#set! injection.language "python"))

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "python3")
  content: (cell_content) @injection.content)
 (#set! injection.language "python"))

; R
; -

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "r")
  content: (cell_content) @injection.content)
 (#set! injection.language "r"))

; Julia
; -----

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "julia")
  content: (cell_content) @injection.content)
 (#set! injection.language "julia"))

; SQL
; ---

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "sql")
  content: (cell_content) @injection.content)
 (#set! injection.language "sql"))

; Bash/Shell
; ----------

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "bash")
  content: (cell_content) @injection.content)
 (#set! injection.language "bash"))

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "sh")
  content: (cell_content) @injection.content)
 (#set! injection.language "bash"))

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "shell")
  content: (cell_content) @injection.content)
 (#set! injection.language "bash"))

; JavaScript/TypeScript
; ---------------------

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "javascript")
  content: (cell_content) @injection.content)
 (#set! injection.language "javascript"))

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "js")
  content: (cell_content) @injection.content)
 (#set! injection.language "javascript"))

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "typescript")
  content: (cell_content) @injection.content)
 (#set! injection.language "typescript"))

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "ts")
  content: (cell_content) @injection.content)
 (#set! injection.language "typescript"))

; Observable JS
; -------------

((executable_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "ojs")
  content: (cell_content) @injection.content)
 (#set! injection.language "javascript"))

; ============================================================================
; INLINE CODE CELLS
; ============================================================================

; Python Inline
; -------------

((inline_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "python")
  content: (cell_content) @injection.content)
 (#set! injection.language "python"))

; R Inline
; --------

((inline_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "r")
  content: (cell_content) @injection.content)
 (#set! injection.language "r"))

; Julia Inline
; ------------

((inline_code_cell
  language: (language_name) @_lang
  (#eq? @_lang "julia")
  content: (cell_content) @injection.content)
 (#set! injection.language "julia"))

; ============================================================================
; REGULAR FENCED CODE BLOCKS (Non-executable)
; ============================================================================

; These are standard Markdown code blocks without {language} syntax

; Python
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "python")
  (code_line) @injection.content)
 (#set! injection.language "python")
 (#set! injection.combined))

; R
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "r")
  (code_line) @injection.content)
 (#set! injection.language "r")
 (#set! injection.combined))

; Julia
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "julia")
  (code_line) @injection.content)
 (#set! injection.language "julia")
 (#set! injection.combined))

; JavaScript
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "javascript")
  (code_line) @injection.content)
 (#set! injection.language "javascript")
 (#set! injection.combined))

((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "js")
  (code_line) @injection.content)
 (#set! injection.language "javascript")
 (#set! injection.combined))

; TypeScript
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "typescript")
  (code_line) @injection.content)
 (#set! injection.language "typescript")
 (#set! injection.combined))

((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "ts")
  (code_line) @injection.content)
 (#set! injection.language "typescript")
 (#set! injection.combined))

; Bash
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "bash")
  (code_line) @injection.content)
 (#set! injection.language "bash")
 (#set! injection.combined))

((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "sh")
  (code_line) @injection.content)
 (#set! injection.language "bash")
 (#set! injection.combined))

; SQL
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "sql")
  (code_line) @injection.content)
 (#set! injection.language "sql")
 (#set! injection.combined))

; JSON
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "json")
  (code_line) @injection.content)
 (#set! injection.language "json")
 (#set! injection.combined))

; YAML
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "yaml")
  (code_line) @injection.content)
 (#set! injection.language "yaml")
 (#set! injection.combined))

; TOML
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "toml")
  (code_line) @injection.content)
 (#set! injection.language "toml")
 (#set! injection.combined))

; HTML
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "html")
  (code_line) @injection.content)
 (#set! injection.language "html")
 (#set! injection.combined))

; CSS
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "css")
  (code_line) @injection.content)
 (#set! injection.language "css")
 (#set! injection.combined))

; Markdown
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "markdown")
  (code_line) @injection.content)
 (#set! injection.language "markdown")
 (#set! injection.combined))

; ============================================================================
; OTHER EMBEDDED CONTENT
; ============================================================================

; YAML Front Matter
; -----------------
; (YAML is now parsed structurally, no injection needed)

; HTML Blocks
; -----------

((html_block
  (html_block_content) @injection.content)
 (#set! injection.language "html"))

; Raw Blocks
; ----------
; Note: Cannot inject based on format since it's embedded in delimiter
