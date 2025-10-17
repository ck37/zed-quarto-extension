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
  (#eq? @_lang "python"))
 (#set! injection.language "python"))

; R
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "r"))
 (#set! injection.language "r"))

; Julia
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "julia"))
 (#set! injection.language "julia"))

; JavaScript
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "javascript"))
 (#set! injection.language "javascript"))

((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "js"))
 (#set! injection.language "javascript"))

; TypeScript
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "typescript"))
 (#set! injection.language "typescript"))

((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "ts"))
 (#set! injection.language "typescript"))

; Bash
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "bash"))
 (#set! injection.language "bash"))

((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "sh"))
 (#set! injection.language "bash"))

; SQL
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "sql"))
 (#set! injection.language "sql"))

; JSON
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "json"))
 (#set! injection.language "json"))

; YAML
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "yaml"))
 (#set! injection.language "yaml"))

; TOML
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "toml"))
 (#set! injection.language "toml"))

; HTML
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "html"))
 (#set! injection.language "html"))

; CSS
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "css"))
 (#set! injection.language "css"))

; Markdown
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "markdown"))
 (#set! injection.language "markdown"))

; ============================================================================
; OTHER EMBEDDED CONTENT
; ============================================================================

; YAML Front Matter
; -----------------

((yaml_front_matter
  (yaml_front_matter_content) @injection.content)
 (#set! injection.language "yaml"))

; HTML Blocks
; -----------

((html_block
  (html_block_content) @injection.content)
 (#set! injection.language "html"))

; Raw Blocks
; ----------
; Note: Cannot inject based on format since it's embedded in delimiter
