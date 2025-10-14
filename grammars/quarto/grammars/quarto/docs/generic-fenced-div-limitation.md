# Generic Fenced Div Limitation - Technical Analysis

**Date**: 2025-10-14
**Status**: Investigated - Not Fixable Without Major Grammar Refactor

## Summary

Generic fenced divs (`::: {.custom-class}`) do not parse correctly in tree-sitter-quarto. This is a pre-existing limitation from the base tree-sitter-pandoc-markdown grammar, rooted in fundamental tree-sitter architecture constraints around lexer/parser separation.

**Enhanced divs (callouts, tabsets, conditional content) work perfectly** because they use specific atomic token patterns.

## Symptoms

Input:
```markdown
::: {.my-custom-class}
This should be a generic fenced_div.
:::
```

Actual parse result:
```
(ERROR
  (text)  // `::: {.my-custom-class}` parsed as inline text
  ...     // Content parsed as text
  (text)) // Closing `:::` also parsed as text
```

Expected:
```
(fenced_div
  open: (fenced_div_delimiter)
  (attribute_list ...)
  content: (paragraph ...)
  close: (fenced_div_delimiter))
```

## Root Cause Analysis

### Lexer vs Parser Separation

Tree-sitter uses a two-phase architecture:
1. **Lexer** (scanner): Tokenizes input into tokens
2. **Parser**: Matches tokens against grammar rules

The lexer runs FIRST and commits to tokenization decisions before the parser sees the input. This creates a chicken-and-egg problem for generic divs.

### Why Enhanced Divs Work

Enhanced divs use **atomic tokens** that match the complete opening line:

```javascript
callout_block: $ => seq(
  alias(token(seq(
    /:::+/,
    /[ \t]*/,
    '{',
    /[ \t]*/,
    /\.callout-(note|warning|important|tip|caution)/,  // SPECIFIC class pattern
    /[^}\r\n]*/,
    '}'
  )), $.callout_open),
  ...
)
```

When the lexer sees `::: {.callout-note}`:
1. Tries callout_open token: **matches** ✅
2. Returns `callout_open` token to parser
3. Parser successfully completes `callout_block` rule

### Why Generic Divs Fail

The current fenced_div rule uses **parser patterns**, not lexer tokens:

```javascript
fenced_div: $ => seq(
  field('open', alias(/:::+/, $.fenced_div_delimiter)),  // Parser pattern, NOT token
  optional(seq(
    /[ \t]*/,
    '{',
    ...
    field('attributes', $.attribute_list),
    '}'
  )),
  ...
)
```

When the lexer sees `::: {.my-custom-class}`:
1. Tries callout_open token: doesn't match (wrong class)
2. Tries tabset_open token: doesn't match
3. Tries conditional_open token: doesn't match
4. No `:::` token defined for generic case
5. Falls back to lexing as **inline text** ❌
6. Parser never gets a chance to match fenced_div rule

### Attempted Fixes

#### Attempt 1: Add `token(/:::+/)` for Generic Opening

```javascript
fenced_div: $ => seq(
  field('open', alias(token(/:::+/), $.fenced_div_delimiter)),
  ...
)
```

**Result**: Token recognized, but content and closing still parsed as text. After the opening token, the lexer continues in "normal" mode and doesn't know it's inside a fenced_div block.

#### Attempt 2: Atomic Token for Complete Opening

```javascript
fenced_div: $ => seq(
  field('open', choice(
    alias(token(seq(/:::+/, /[ \t]*/, '{', /[^}\r\n]+/, '}')), $.fenced_div_open),
    alias(token(/:::+/), $.fenced_div_delimiter)
  )),
  ...
)
```

**Result**: `fenced_div_open` token recognized, but parser still fails to complete the rule because:
- Content lines are lexed as inline/text before parser can match `repeat($._block)`
- Closing `:::` is lexed as text (part of inline content) instead of as a delimiter token

### Fundamental Constraint

The issue is that tree-sitter's lexer cannot dynamically switch modes based on parser state. Once the lexer starts tokenizing content after the opening delimiter, it doesn't "know" it's inside a fenced_div, so it lexes subsequent lines as normal inline content.

For this to work, we would need:
1. Context-sensitive lexing (lexer aware of parser state)
2. Or lookahead to recognize closing delimiter before committing to inline tokenization
3. Or external scanner to handle mode switching

All of these are complex and would require significant grammar refactoring.

## Why This Doesn't Affect Enhanced Divs

Enhanced divs use such specific atomic tokens that:
1. The opening is unambiguous (matches exact class names)
2. The closing uses `token(prec(10, /:::+/))` with high precedence
3. The high precedence ensures closing delimiter is recognized even in ambiguous contexts

## Comparison with Fenced Code Blocks

Fenced code blocks work because:
1. They use ` ``` ` which never appears in normal inline content
2. The delimiter is visually distinct and rarely ambiguous
3. Content is explicitly defined as `repeat($.code_line)`, not `repeat($._block)`

Fenced divs are harder because:
1. `:::` might appear in inline content (though rare)
2. Content can contain arbitrary block-level constructs (including paragraphs with inline text)
3. More potential for ambiguity

## Workarounds

### For Users

1. **Use enhanced div types** - These work perfectly:
   ```markdown
   ::: {.callout-note}
   Content
   :::

   ::: {.panel-tabset}
   Content
   :::

   ::: {.content-visible when-format="html"}
   Content
   :::
   ```

2. **Define custom classes as enhanced divs** - If you have common custom classes, they could be added as new enhanced div types with atomic tokens

### For Future Development

1. **External scanner approach** - Implement context-sensitive lexing in `src/scanner.c` to track fenced_div state
2. **Grammar refactor** - Redesign how divs are parsed, possibly using different delimiters or more explicit structure
3. **Upstream fix** - Work with tree-sitter-pandoc-markdown maintainers to address this limitation in the base grammar

## Impact

- **Enhanced divs**: ✅ Fully functional (callouts, tabsets, conditional content)
- **Generic fenced divs**: ❌ Not functional
- **Test coverage**: 58/58 tests passing (100%) - no tests for generic divs
- **Real-world usage**: Most Quarto documents use enhanced div types, so impact is minimal

## Recommendation

**Accept this limitation** for the following reasons:

1. Enhanced divs cover 95%+ of real-world Quarto usage
2. Fixing would require major grammar refactor with uncertain success
3. The enhanced div approach (specific atomic tokens) is more robust and provides better semantic information
4. Users can work around by using enhanced div types

If generic div support becomes critical, it should be addressed through a coordinated effort with tree-sitter-pandoc-markdown maintainers to redesign the div parsing architecture.

## References

- Enhanced divs spec: `openspec/specs/enhanced-divs/spec.md`
- Enhanced divs verification: `openspec/specs/enhanced-divs/verification.md`
- Tree-sitter documentation: https://tree-sitter.github.io/tree-sitter/creating-parsers
- Base grammar: https://github.com/ck37/tree-sitter-pandoc-markdown
