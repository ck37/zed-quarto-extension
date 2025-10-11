;
; Inject YAML into frontmatter blocks
;

((minus_metadata) @injection.content
  (#set! injection.language "yaml")
  (#set! injection.include-children))

;
; Inject executable chunks into the embedded language they declare.
;

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

