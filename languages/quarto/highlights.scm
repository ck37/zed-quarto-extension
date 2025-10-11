; Pandoc markdown highlights (from tree-sitter-pandoc-markdown)
(atx_heading (inline) @text.title)
(setext_heading (inline) @text.title)

[
  (atx_heading_marker)
  (setext_heading_marker)
] @punctuation.special

[(fenced_code_block) (code_span)] @text.literal
[(fenced_code_block_delimiter)] @punctuation.delimiter

[(link)] @text.uri
[(list_marker) (thematic_break) (block_quote_marker)] @punctuation.special

; YAML front matter
(yaml_front_matter_content) @comment.documentation

[
  (yaml_front_matter_start)
  (yaml_front_matter_delimiter)
] @punctuation.delimiter

; Emphasis
; Note: stick with `text.emphasis`/`text.strong` to match upstream Markdown
; so editor themes highlight consistently between .md and .qmd files.
(emphasis) @text.emphasis
(strong_emphasis) @text.strong

;
; Quarto / Pandoc specific constructs
; NOTE: These node types are from tree-sitter-pandoc-markdown Phase 1C.
;

; Fenced divs (:::{.callout-note})
(fenced_div) @markup.raw.block

; Attribute lists {.class #id key=value}
(attribute_list) @attribute

; Citations [@smith2024] and @smith2024
[
  (citation_group)
  (citation)
] @string.special.symbol

; Cross-references @fig:plot, @tbl:data
(cross_reference) @string.special.symbol

; Shortcodes {{< include file.qmd >}}
[
  (shortcode_block)
  (shortcode)
] @function.macro

; Chunk options #| echo: false
(chunk_option) @comment.documentation
