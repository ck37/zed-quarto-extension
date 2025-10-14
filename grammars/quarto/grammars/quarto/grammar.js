/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

/**
 * Tree-sitter grammar for Quarto Markdown
 *
 * Extends Pandoc Markdown with Quarto-specific features:
 * - Executable code cells with {language} syntax
 * - Chunk options (#| key: value)
 * - Cross-reference distinction (@fig-plot vs @citation)
 * - Inline code cells (`{python} expr`)
 *
 * SOURCE TRACKING (per openspec/specs/grammar-foundation requirement):
 * - Base: tree-sitter-pandoc-markdown
 * - Repository: https://github.com/ck37/tree-sitter-pandoc-markdown
 * - Commit: 95f296eb8a9f28760f3b6ae34084282a1b9dc52a
 * - Date copied: 2025-10-14
 * - Strategy: Copy & Extend (see docs/plan.md)
 *
 * MODIFICATIONS FROM BASE:
 * - Added executable_code_cell node for {language} syntax
 * - Added chunk_options and chunk_option nodes for #| syntax
 * - Added cross_reference node to distinguish from citations
 * - Added inline_code_cell node for inline execution
 * - Extended _block choice to include executable_code_cell
 * - Extended _inline_element choice to include inline_code_cell and cross_reference
 * - Modified conflicts array for new node types
 * - Token-based chunk option parsing with token(prec(2, '#|'))
 * - R shorthand syntax using token(seq('`r', /[ \t]+/))
 */

function thematicLine(char) {
  return token(new RegExp(`${char}(?:[ \t]*${char}){2,}[ \t]*\\r?\\n`));
}

module.exports = grammar({
  name: 'quarto',

  extras: $ => [/\s/],

  externals: $ => [
    $.pipe_table_start,           // From pandoc-markdown
    $._chunk_option_marker,       // Quarto: #| at start of cell
    $._cell_boundary,             // Quarto: Track cell context
  ],

  conflicts: $ => [
    [$._inline_element, $._link_text_element],
    [$.pipe_table, $.paragraph],
    [$.pipe_table_header, $.inline],
    [$.executable_code_cell, $.fenced_code_block],  // Quarto: {python} vs python
    [$.shortcode_block, $.shortcode_inline],        // Shortcode can be block or inline
    [$.inline_code_cell, $.code_span],              // `r expr` vs `code`
    [$.callout_block, $.fenced_div],                // Enhanced divs vs generic divs
    [$.tabset_block, $.fenced_div],
    [$.conditional_block, $.fenced_div],
  ],

  rules: {
    document: $ => choice(
      prec(2, seq($.yaml_front_matter, repeat($._block))),
      prec(2, seq($.percent_metadata, repeat($._block))),
      repeat($._block)
    ),

    _block: $ => choice(
      // Quarto-specific blocks
      $.executable_code_cell,
      // Pandoc Markdown blocks - fenced_div must come before enhanced divs for fallback
      $.fenced_div,
      // Enhanced divs (higher precedence via prec.dynamic)
      $.callout_block,
      $.tabset_block,
      $.conditional_block,
      // Other Pandoc Markdown blocks
      $.atx_heading,
      $.setext_heading,
      $.block_quote,
      $.footnote_definition,
      $.link_reference_definition,
      $.display_math,
      $.pipe_table,
      $.shortcode_block,
      $.raw_block,
      $.html_block,
      $.fenced_code_block,
      $.list,
      $.thematic_break,
      $.paragraph,
      $.blank_line
    ),

    // ============================================================================
    // QUARTO-SPECIFIC RULES
    // ============================================================================

    /**
     * Executable Code Cell
     *
     * ```{python}
     * #| label: fig-plot
     * #| echo: false
     * import matplotlib.pyplot as plt
     * plt.plot([1, 2, 3])
     * ```
     *
     * Spec: openspec/specs/executable-cells/spec.md
     */
    executable_code_cell: $ => prec(1, seq(
      field('open_delimiter', alias(token(/```+/), $.code_fence_delimiter)),
      field('language_specifier', seq(
        '{',
        field('language', alias(/[a-zA-Z][a-zA-Z0-9_-]*/, $.language_name)),
        optional(field('attributes', $.attribute_list)),
        '}'
      )),
      /\r?\n/,
      optional(field('chunk_options', $.chunk_options)),
      optional(field('content', $.cell_content)),
      field('close_delimiter', alias(token(/```+/), $.code_fence_delimiter)),
      /\r?\n/
    )),

    /**
     * Chunk Options
     *
     * #| label: fig-plot
     * #| echo: false
     * #| fig-cap: "Sample plot"
     *
     * Spec: openspec/specs/chunk-options/spec.md
     */
    chunk_options: $ => repeat1($.chunk_option),

    chunk_option: $ => seq(
      token(prec(2, '#|')),
      optional(/[ \t]*/),
      field('key', alias(/[a-zA-Z][a-zA-Z0-9-]*/, $.chunk_option_key)),
      ':',
      optional(seq(
        optional(/[ \t]*/),
        field('value', alias(/[^\r\n]+/, $.chunk_option_value))
      )),
      /\r?\n/
    ),

    /**
     * Cell Content
     *
     * Code content within executable cell, excluding chunk options.
     * Used for language injection.
     */
    cell_content: $ => repeat1(seq(
      alias(/[^\r\n]+/, $.code_line),
      /\r?\n/
    )),

    // ============================================================================
    // PANDOC MARKDOWN RULES (BASELINE)
    // ============================================================================

    // YAML Front Matter
    // NOTE: Standalone `---` at document start is ambiguous with thematic break.
    // This grammar treats it as (invalid) YAML front matter.
    yaml_front_matter: $ => prec(-1, seq(
      field('start', alias(token(seq('---', /\r?\n/)), $.yaml_front_matter_start)),
      repeat(seq(
        alias(
          token(prec(-1, /[ \t]*[^\r\n-][^\r\n]*/)),
          $.yaml_front_matter_content
        ),
        /\r?\n/
      )),
      field('close', alias(token(prec(1, choice('---', '...'))), $.yaml_front_matter_delimiter)),
      /\r?\n/
    )),

    // Percent Metadata (Pandoc extension)
    percent_metadata: $ => repeat1(seq(
      token('%'),
      optional(alias(/[^\r\n]+/, $.metadata_line)),
      /\r?\n/
    )),

    // Headings
    atx_heading: $ => seq(
      field('marker', alias(token(prec(1, /#{1,6}[ \t]*/)), $.atx_heading_marker)),
      optional(field('content', $.inline)),
      /\r?\n/
    ),

    setext_heading: $ => seq(
      field('content', $.inline),
      /\r?\n/,
      field('underline', alias(choice(token(/=+/), token(/-+/)), $.setext_heading_marker)),
      /\r?\n/
    ),

    // Block Quote
    block_quote: $ => prec.right(seq(
      $.block_quote_line,
      repeat($.block_quote_line)
    )),

    block_quote_line: $ => seq(
      field('marker', alias(token(prec(1, seq('>', optional(/[ \t]/)))), $.block_quote_marker)),
      optional(field('content', $.inline)),
      /\r?\n/
    ),

    // Paragraph
    paragraph: $ => prec.left(-2, seq(
      field('content', $.inline),
      /\r?\n/
    )),

    // Fenced Div
    fenced_div: $ => prec.left(1, seq(
      field('open', alias(/:::+/, $.fenced_div_delimiter)),
      optional(seq(
        /[ \t]*/,
        '{',
        /[ \t]*/,
        field('attributes', $.attribute_list),
        /[ \t]*/,
        '}'
      )),
      /\r?\n/,
      repeat($._block),
      field('close', alias(/:::+/, $.fenced_div_delimiter)),
      /\r?\n/
    )),

    // Fenced Code Block (regular, non-executable)
    fenced_code_block: $ => prec(-1, seq(
      field('open', alias(token(/```+/), $.code_fence_delimiter)),
      optional(field('info', $.info_string)),
      /\r?\n/,
      repeat(seq(alias(/[^\r\n]+/, $.code_line), /\r?\n/)),
      field('close', alias(token(/```+/), $.code_fence_delimiter)),
      /\r?\n/
    )),

    info_string: $ => /[^\r\n{]+/,

    // HTML Block
    html_block: $ => seq(
      field('open', alias(token(prec(1, /<[^>\s]+[^>]*>/)), $.html_open_tag)),
      repeat(seq(alias(/[^<\r\n][^\r\n]*/, $.html_block_content), /\r?\n/)),
      field('close', alias(token(/<\/[A-Za-z][^>]*>/), $.html_close_tag)),
      /\r?\n/
    ),

    // Raw Block
    raw_block: $ => seq(
      field('open', alias(token(prec(1, /```\{=\w+\}/)), $.raw_block_delimiter)),
      /\r?\n/,
      repeat(seq(alias(/[^\r\n]+/, $.raw_block_content), /\r?\n/)),
      field('close', alias(token(/```/), $.raw_block_delimiter)),
      /\r?\n/
    ),

    // Display Math
    display_math: $ => seq(
      field('open', alias(token('$$'), $.math_delimiter)),
      optional(field('content', alias(/[^$]+/, $.math_content))),
      field('close', alias(token('$$'), $.math_delimiter)),
      /\r?\n/
    ),

    // Shortcode Block
    shortcode_block: $ => prec(1, seq(
      alias(token(/\{\{<[ \t]*/), $.shortcode_open),
      field('name', alias(/[a-zA-Z][a-zA-Z0-9_-]*/, $.shortcode_name)),
      optional(field('arguments', alias(/[ \t]+[^ \t\r\n>][^>\r\n]*/, $.shortcode_arguments))),
      alias(token(/[ \t]*>\}\}\r?\n/), $.shortcode_close)
    )),

    /**
     * Callout Block
     *
     * ::: {.callout-note}
     * Content here
     * :::
     *
     * Spec: openspec/specs/enhanced-divs/spec.md
     */
    callout_block: $ => prec.dynamic(3, seq(
      alias(token(seq(
        /:::+/,
        /[ \t]*/,
        '{',
        /[ \t]*/,
        /\.callout-(note|warning|important|tip|caution)/,
        /[^}\r\n]*/,
        '}'
      )), $.callout_open),
      /\r?\n/,
      field('content', repeat($._block)),
      field('close', alias(token(prec(10, /:::+/)), $.fenced_div_delimiter)),
      /\r?\n/
    )),

    /**
     * Tabset Block
     *
     * ::: {.panel-tabset}
     * ## Tab 1
     * Content
     * :::
     *
     * Spec: openspec/specs/enhanced-divs/spec.md
     */
    tabset_block: $ => prec.dynamic(3, seq(
      alias(token(seq(
        /:::+/,
        /[ \t]*/,
        '{',
        /[ \t]*/,
        /\.panel-tabset/,
        /[^}\r\n]*/,
        '}'
      )), $.tabset_open),
      /\r?\n/,
      field('content', repeat($._block)),
      field('close', alias(token(prec(10, /:::+/)), $.fenced_div_delimiter)),
      /\r?\n/
    )),

    /**
     * Conditional Content Block
     *
     * ::: {.content-visible when-format="html"}
     * HTML-only content
     * :::
     *
     * Spec: openspec/specs/enhanced-divs/spec.md
     */
    conditional_block: $ => prec.dynamic(3, seq(
      alias(token(seq(
        /:::+/,
        /[ \t]*/,
        '{',
        /[ \t]*/,
        /\.content-(visible|hidden)/,
        /[^}\r\n]*/,
        '}'
      )), $.conditional_open),
      /\r?\n/,
      field('content', repeat($._block)),
      field('close', alias(token(prec(10, /:::+/)), $.fenced_div_delimiter)),
      /\r?\n/
    )),

    // Lists
    list: $ => choice(
      $.ordered_list,
      $.unordered_list
    ),

    ordered_list: $ => prec.right(repeat1($.ordered_list_item)),
    unordered_list: $ => prec.right(repeat1($.unordered_list_item)),

    ordered_list_item: $ => seq(
      field('marker', alias(token(/\d+[.)]/), $.list_marker)),
      /[ \t]+/,
      optional(field('content', $.inline)),
      /\r?\n/
    ),

    unordered_list_item: $ => seq(
      field('marker', alias(token(/[-*+]/), $.list_marker)),
      /[ \t]+/,
      optional(field('content', $.inline)),
      /\r?\n/
    ),

    // Thematic Break
    thematic_break: $ => choice(
      thematicLine('\\-'),
      thematicLine('\\*'),
      thematicLine('_')
    ),

    // Blank Line
    blank_line: $ => /\r?\n/,

    // Footnote Definition
    footnote_definition: $ => seq(
      field('marker', alias(token(/\[\^[^\]]+\]:/), $.footnote_marker)),
      /[ \t]+/,
      field('content', $.inline),
      /\r?\n/
    ),

    // Link Reference Definition
    link_reference_definition: $ => seq(
      field('label', seq('[', alias(/[^\]]+/, $.reference_label), ']:')),
      /[ \t]+/,
      field('destination', alias(/\S+/, $.link_destination)),
      optional(seq(
        /[ \t]+/,
        field('title', alias(/"[^"]*"/, $.link_title))
      )),
      /\r?\n/
    ),

    // Pipe Table
    pipe_table: $ => prec.right(seq(
      $.pipe_table_start,
      $.pipe_table_header,
      $.pipe_table_delimiter,
      repeat($.pipe_table_row)
    )),

    pipe_table_header: $ => seq(
      token('|'),
      repeat1(seq(
        field('content', alias(/[^|\r\n]+/, $.table_cell)),
        token('|')
      )),
      /\r?\n/
    ),

    pipe_table_delimiter: $ => seq(
      token('|'),
      repeat1(seq(
        alias(/[ \t]*:?-+:?[ \t]*/, $.table_delimiter_cell),
        token('|')
      )),
      /\r?\n/
    ),

    pipe_table_row: $ => seq(
      token('|'),
      repeat1(seq(
        field('content', alias(/[^|\r\n]+/, $.table_cell)),
        token('|')
      )),
      /\r?\n/
    ),

    // Attribute List (for divs, cells, etc.)
    attribute_list: $ => choice(
      seq(
        field('id', alias(/#[a-zA-Z][a-zA-Z0-9_-]*/, $.attribute_id)),
        repeat(field('class', alias(/\.[a-zA-Z][a-zA-Z0-9_-]*/, $.attribute_class))),
        repeat(field('attribute', $.key_value_attribute))
      ),
      seq(
        repeat1(field('class', alias(/\.[a-zA-Z][a-zA-Z0-9_-]*/, $.attribute_class))),
        repeat(field('attribute', $.key_value_attribute))
      ),
      repeat1(field('attribute', $.key_value_attribute))
    ),

    key_value_attribute: $ => seq(
      field('key', alias(/[a-zA-Z][a-zA-Z0-9_-]*/, $.attribute_key)),
      '=',
      field('value', choice(
        alias(/"[^"]*"/, $.attribute_value),
        alias(/'[^']*'/, $.attribute_value),
        alias(/[^\s}]+/, $.attribute_value)
      ))
    ),

    // ============================================================================
    // INLINE RULES
    // ============================================================================

    inline: $ => repeat1($._inline_element),

    _inline_element: $ => choice(
      $.text,
      $.inline_code_cell,       // Quarto: `{python} expr` - check before code_span
      $.code_span,
      $.inline_math,
      $.emphasis,
      $.strong_emphasis,
      $.link,
      $.image,
      $.citation,
      $.cross_reference,        // Quarto: @fig-plot
      $.shortcode_inline
    ),

    text: $ => /[^\r\n`*_\[@<${]+/,

    // Text inside link brackets - excludes ] to allow proper link parsing
    link_text: $ => /[^\r\n`*_\[@<${\]]+/,

    code_span: $ => seq(
      alias(token('`'), $.code_span_delimiter),
      alias(/[^`]+/, $.code_span_content),
      alias(token('`'), $.code_span_delimiter)
    ),

    inline_math: $ => seq(
      alias(token('$'), $.math_delimiter),
      alias(/[^$]+/, $.math_content),
      alias(token('$'), $.math_delimiter)
    ),

    emphasis: $ => prec.left(choice(
      seq(token('*'), repeat1($._inline_element), token('*')),
      seq(token('_'), repeat1($._inline_element), token('_'))
    )),

    strong_emphasis: $ => prec.left(choice(
      seq(token('**'), repeat1($._inline_element), token('**')),
      seq(token('__'), repeat1($._inline_element), token('__'))
    )),

    link: $ => seq(
      field('text', seq('[', repeat($._link_text_element), ']')),
      field('destination', seq('(', alias(/[^)]+/, $.link_destination), ')'))
    ),

    _link_text_element: $ => choice(
      $.link_text,
      $.code_span,
      $.emphasis,
      $.strong_emphasis
    ),

    image: $ => seq(
      token('!'),
      field('alt', seq('[', alias(/[^\]]*/, $.image_alt), ']')),
      field('source', seq('(', alias(/[^)]+/, $.image_source), ')'))
    ),

    /**
     * Citation
     *
     * @smith2020 or [@smith2020]
     *
     * Distinguished from cross-reference by lack of type prefix.
     */
    citation: $ => seq(
      token('@'),
      field('key', alias(/[a-zA-Z][a-zA-Z0-9_]*/, $.citation_key))
    ),

    /**
     * Cross-Reference (Quarto-specific)
     *
     * @fig-plot, @tbl-data, @eq-linear, @sec-intro, @lst-code
     *
     * Spec: openspec/specs/cross-references/spec.md
     */
    cross_reference: $ => seq(
      token('@'),
      field('type', alias(choice('fig', 'tbl', 'eq', 'sec', 'lst'), $.reference_type)),
      token('-'),
      field('id', alias(/[a-zA-Z0-9_-]+/, $.reference_id))
    ),

    /**
     * Inline Code Cell (Quarto-specific)
     *
     * `{python} mean([1, 2, 3])`
     * `r mean(x)`
     *
     * Spec: openspec/specs/inline-code-cells/spec.md
     */
    inline_code_cell: $ => prec.dynamic(1, choice(
      // Curly brace syntax: `{python} expr`
      seq(
        alias(token('`{'), $.inline_cell_delimiter),
        field('language', alias(/[a-zA-Z][a-zA-Z0-9_-]*/, $.language_name)),
        alias(token('}'), $.inline_cell_brace),
        optional(/[ \t]+/),
        field('content', alias(/[^`]+/, $.cell_content)),
        alias(token('`'), $.inline_cell_delimiter)
      ),
      // Shorthand syntax: `r expr`
      seq(
        alias(token(seq('`r', /[ \t]+/)), $.inline_cell_delimiter),
        field('content', alias(/[^`]+/, $.cell_content)),
        alias(token('`'), $.inline_cell_delimiter)
      )
    )),

    /**
     * Inline Shortcode
     *
     * {{< video url >}}
     */
    shortcode_inline: $ => seq(
      alias(token(/\{\{<[ \t]*/), $.shortcode_open),
      field('name', alias(/[a-zA-Z][a-zA-Z0-9_-]*/, $.shortcode_name)),
      optional(field('arguments', alias(/[ \t]+[^ \t\r\n>][^>]*/, $.shortcode_arguments))),
      alias(token(/[ \t]*>\}\}/), $.shortcode_close)
    ),
  }
});
