; Pandoc Markdown Text Objects Queries
; For nvim-treesitter text object selection

; ============================================================================
; Code Blocks (@block)
; ============================================================================

; Entire code block including delimiters
(fenced_code_block) @block.outer

; Just the code content
(code_fence_content) @block.inner

; Raw blocks
(raw_block) @block.outer
(raw_block_content) @block.inner

; ============================================================================
; Links (@link)
; ============================================================================

; Entire link including brackets
(link) @link.outer

; Just the link text
(link_text) @link.inner

; ============================================================================
; Images (@image)
; ============================================================================

; Entire image syntax
(image) @image.outer

; Image alt text
(image
  (link_text) @image.inner)

; ============================================================================
; Emphasis (@emphasis)
; ============================================================================

; Both emphasis and strong emphasis as emphasis objects
(emphasis) @emphasis.outer
(strong_emphasis) @emphasis.outer

; ============================================================================
; Headings (@heading)
; ============================================================================

; Entire heading including marker
(atx_heading) @heading.outer
(setext_heading) @heading.outer

; Just the heading content
(atx_heading
  (inline) @heading.inner)

(setext_heading
  (inline) @heading.inner)

; ============================================================================
; Lists (@list)
; ============================================================================

; Entire list
(list) @list.outer

; Individual list item
(list_item) @list.inner

; ============================================================================
; Block Quotes (@quote)
; ============================================================================

; Entire block quote
(block_quote) @quote.outer

; Block quote content (without markers)
; Note: This may need refinement based on grammar structure
(block_quote) @quote.inner

; ============================================================================
; Fenced Divs (@div or @container)
; ============================================================================

; Entire fenced div including delimiters
(fenced_div) @div.outer

; Div content without delimiters
; Note: Content is composed of nested blocks
(fenced_div) @div.inner

; ============================================================================
; Tables (@table)
; ============================================================================

; Entire table
(pipe_table) @table.outer

; Table cell
(pipe_table_cell) @table.inner
(pipe_table_header_cell) @table.inner

; ============================================================================
; Footnotes (@footnote)
; ============================================================================

; Footnote definition
(footnote_definition) @footnote.outer

; Footnote reference
(footnote_reference) @footnote.inner

; ============================================================================
; Math (@math)
; ============================================================================

; Display math
(display_math) @math.outer
(display_math
  (math_content) @math.inner)

; ============================================================================
; Paragraphs (@paragraph)
; ============================================================================

; Entire paragraph
(paragraph) @paragraph.outer

; Paragraph content
(paragraph
  (inline) @paragraph.inner)

; ============================================================================
; Usage Examples
; ============================================================================

; In Neovim with nvim-treesitter-textobjects:
; - `vaf` - Visual select around function/block (code block)
; - `vil` - Visual select inner link (link text only)
; - `vah` - Visual select around heading (including marker)
; - `vih` - Visual select inner heading (content only)
; - `]b` - Jump to next block start
; - `[b` - Jump to previous block start
; - `dah` - Delete around heading (whole heading)
; - `cil` - Change inner link (change link text)
