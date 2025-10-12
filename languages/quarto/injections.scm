;
; Inject executable chunks into the embedded language they declare.
;
; Language injection reference: https://zed.dev/docs/extensions/languages
;

; Inject Pandoc's inline grammar for full emphasis highlighting
; This requires Zed fix: https://github.com/zed-industries/zed/pull/[PR_NUMBER]
; Provides 100% coverage: bold, italic, links, mixed content, all Pandoc inline features
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))

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

