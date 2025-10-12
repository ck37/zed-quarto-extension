;
; Inject executable chunks into the embedded language they declare.
;
; Language injection reference: https://zed.dev/docs/extensions/languages
;

; Inject Zed's built-in markdown-inline for basic emphasis highlighting
; Works for: bold (**/__), italic (*/_), inline code
; Doesn't work for: mixed content, triple asterisks, links, Pandoc-specific features
((inline) @injection.content
 (#set! injection.language "markdown-inline"))

(yaml_front_matter
  (yaml_front_matter_content) @injection.content
  (#set! injection.language "yaml"))

(fenced_code_block
  (info_string) @injection.language
  (code_fence_content) @injection.content
  (#match? @injection.language "^[A-Za-z0-9_+-]+"))

((fenced_code_block
   (info_string) @injection.language
   (code_fence_content) @injection.content)
 (#match? @injection.language "(?i)^\\{?python(?:[\\s,}]|$)")
 (#set! injection.language "python"))

((fenced_code_block
   (info_string) @injection.language
   (code_fence_content) @injection.content)
 (#match? @injection.language "(?i)^\\{?r(?:[\\s,}]|$)")
 (#set! injection.language "r"))

((fenced_code_block
   (info_string) @injection.language
   (code_fence_content) @injection.content)
 (#match? @injection.language "(?i)^\\{?julia(?:[\\s,}]|$)")
 (#set! injection.language "julia"))

((fenced_code_block
   (info_string) @injection.language
   (code_fence_content) @injection.content)
 (#match? @injection.language "(?i)^\\{?sql(?:[\\s,}]|$)")
 (#set! injection.language "sql"))

; Math content injections
((display_math (math_content) @injection.content)
  (#set! injection.language "latex"))

((inline_math (math_content) @injection.content)
  (#set! injection.language "latex"))

