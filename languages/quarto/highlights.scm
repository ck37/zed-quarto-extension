; Pandoc markdown highlights (from tree-sitter-pandoc-markdown)
(atx_heading (inline) @text.title)
(setext_heading (paragraph) @text.title)

[
  (atx_h1_marker)
  (atx_h2_marker)
  (atx_h3_marker)
  (atx_h4_marker)
  (atx_h5_marker)
  (atx_h6_marker)
  (setext_h1_underline)
  (setext_h2_underline)
] @punctuation.special

[
  (link_title)
  (indented_code_block)
  (fenced_code_block)
] @text.literal

[
  (fenced_code_block_delimiter)
] @punctuation.delimiter

[
  (link_destination)
] @text.uri

[
  (link_label)
] @text.reference

[
  (list_marker_plus)
  (list_marker_minus)
  (list_marker_star)
  (list_marker_dot)
  (list_marker_parenthesis)
  (thematic_break)
] @punctuation.special

[
  (block_continuation)
  (block_quote_marker)
] @punctuation.special

[
  (backslash_escape)
] @string.escape

;
; Quarto / Pandoc specific constructs
;

;; Div or callout fences (:::
((inline
   ":" @punctuation.special
   ":" @punctuation.special
   ":" @punctuation.special
   "{" @punctuation.delimiter
   (_)*
   "}" @punctuation.delimiter
   (_)*))

;; Closing ::: fences
((inline
   (_)+
   ":" @punctuation.special
   ":" @punctuation.special
   ":" @punctuation.special))

;; Shortcodes {{< ... >}} / {{% ... %}}
((inline
   "{" @punctuation.delimiter
   "{" @punctuation.delimiter
   "<" @punctuation.delimiter
   (_)*
   ">" @punctuation.delimiter
   "}" @punctuation.delimiter
   "}" @punctuation.delimiter))

((inline
   "{" @punctuation.delimiter
   "{" @punctuation.delimiter
   "%" @punctuation.delimiter
   (_)*
   "%" @punctuation.delimiter
   "}" @punctuation.delimiter
   "}" @punctuation.delimiter))

;; Cross-reference marker
((inline "@" @text.reference))
