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

