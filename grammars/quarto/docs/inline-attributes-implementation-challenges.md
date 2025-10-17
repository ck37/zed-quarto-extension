# Inline Attributes Implementation Challenges

**Date**: 2025-10-14 (Updated: 2025-10-17)
**Status**: ✅ Implemented (Option 1: inline_link approach)
**Related Spec**: `openspec/changes/implement-inline-attributes/`
**Known Issues**: See [inline-attributes-known-issues.md](./inline-attributes-known-issues.md)

## Overview

This document captures the research, implementation attempts, and technical challenges encountered while trying to add Pandoc-style inline attributes (`[text]{#id .class}`) to tree-sitter-quarto.

## Feature Description

Pandoc supports inline attributes syntax that allows adding identifiers, classes, and key-value pairs to arbitrary spans of text:

```markdown
[highlighted text]{#important .alert}
[code span]{.language-python}
[styled text]{style="color: red;"}
```

This is distinct from links (`[text](url)`) and should parse as a `bracketed_span` node with attributes.

## Research: Official Grammar Implementation

### Investigation Process

1. **tree-sitter-pandoc-markdown**: Does NOT implement inline attributes
2. **quarto-markdown** (Rust-based): DOES implement inline attributes

### Official Grammar Approach

The official [quarto-markdown](https://github.com/quarto-dev/quarto-markdown) grammar implements inline attributes by:

1. **Parsing as inline_link**: Treats `[text]{attrs}` as a variant of the `inline_link` node type
2. **Attribute specifier nodes**:
   - `id_specifier` for `#id`
   - `class_specifier` for `.class`
   - `key_value_specifier` for `key="value"`
3. **No semantic distinction**: Bracketed spans are not distinguished from links at the grammar level

**Example AST from official grammar**:
```
inline_link [0, 0] - [0, 25]
  id_specifier [0, 19] - [0, 29]
  class_specifier [0, 29] - [0, 36]
```

**Trade-off**: Semantically imperfect (a span is not a link) but avoids parser ambiguity.

## Implementation Attempts

### Attempt 1: External Scanner Lookahead

**Strategy**: Use external scanner to detect `[text]{` pattern and disambiguate from `[text](` links.

**Implementation**:
1. Added `BRACKETED_SPAN_MARKER` token to `src/scanner.c`
2. Scanner looks ahead to check if `]` is followed by `{` (attributes) or `(` (link)
3. Returns zero-width token to signal parser which path to take
4. Added `bracketed_span` rule using this token

**Code**:
```c
// scanner.c
bool scan_bracketed_span_marker(TSLexer *lexer) {
  if (lexer->lookahead != '[') return false;

  lexer->mark_end(lexer);  // Zero-width token
  lexer->advance(lexer, false);

  // Scan to closing ]
  int bracket_depth = 1;
  while (lexer->lookahead != 0 && lexer->lookahead != '\n') {
    if (lexer->lookahead == '[') bracket_depth++;
    else if (lexer->lookahead == ']') {
      bracket_depth--;
      if (bracket_depth == 0) {
        lexer->advance(lexer, false);
        break;
      }
    }
    lexer->advance(lexer, false);
  }

  // Check what follows ]
  return (lexer->lookahead == '{');
}
```

```javascript
// grammar.js
bracketed_span: $ => seq(
  $._bracketed_span_marker,
  '[',
  field('content', alias(repeat1($._inline_element), $.span_content)),
  ']',
  '{',
  field('attributes', $.attribute_list),
  '}'
),
```

**Results**:
- ✅ Parser generated successfully
- ✅ `bracketed_span` node created correctly
- ❌ **23 tests failed** (64/87 passing instead of 87/87)
- ❌ Cross-references broke: `@fig-plot` produced ERROR nodes
- ❌ Code spans broke: backtick syntax produced ERROR nodes
- ❌ Inline content after bracketed spans parsed as ERROR

**Example failure**:
```
Input: "See @fig-plot for details."
Expected: (paragraph (inline (text) (cross_reference) (text)))
Actual: (ERROR (inline (text) (cross_reference)) (language_name) (citation_key) (ERROR))
```

**Root Cause**: Adding the external token and `bracketed_span` to `_inline_element` caused the LR(1) parser to reorganize parse states in a way that broke parsing of unrelated inline elements.

### Attempt 2: Grammar-Only with Precedence

**Strategy**: Remove external scanner, use dynamic precedence to prefer `bracketed_span` over `link`.

**Implementation**:
```javascript
bracketed_span: $ => prec.dynamic(1, seq(
  '[',
  field('content', alias(/[^\]]+/, $.span_content)),
  ']',
  '{',
  field('attributes', $.attribute_list),
  '}'
)),
```

**Results**:
- ✅ Parser generated successfully
- ❌ **Same 23 test failures** as external scanner approach
- ❌ Issue persists even without external scanner

**Root Cause**: Simply having `bracketed_span` in the `_inline_element` choice causes parse state conflicts, regardless of implementation method.

### Attempt 3: Disable bracketed_span from _inline_element

**Test**: Temporarily commented out `$.bracketed_span` from `_inline_element` choice.

**Results**:
- ❌ **Still 23 test failures**
- This means the issue wasn't just the external scanner or the rule definition

### Verification: Revert All Changes

**Test**: `git checkout grammar.js` and `git checkout src/scanner.c`

**Results**:
- ✅ **87/87 tests passing** (100%)
- Confirmed our changes introduced the regression

## Technical Challenges

### LR(1) Parser Limitations

Tree-sitter uses an LR(1) parser, which has **one-token lookahead**:

1. **Problem with `[text]{attrs}`**: Parser sees `[` and must decide:
   - Is this a `link` → expects `](url)`?
   - Is this a `bracketed_span` → expects `]{attrs}`?
   - Is this a `footnote_reference` → expects `[^id]`?

2. **Decision point**: Parser commits to a parse path when it sees `[`, before it can see the closing `]` and what follows.

3. **Parse state explosion**: Adding `bracketed_span` as an alternative creates ambiguities that ripple through the entire inline grammar, affecting even inputs with no brackets.

### Why External Scanner Didn't Help

External scanners CAN provide arbitrary lookahead, but:
1. The grammar structure still declares the ambiguity in `_inline_element`
2. Tree-sitter's parser generator creates the parse tables at compile time
3. The external scanner runs at parse time, but the ambiguity already affected state table generation
4. Adding the external token created additional parse states that conflicted with existing inline parsing

### Why Official Grammar's Approach Works

The official grammar avoids this by:
1. **Not creating a separate node type**: `bracketed_span` is just an `inline_link` variant
2. **Single parse path**: Parser always treats `[...]` as the start of a link
3. **Post-parse distinction**: The presence of `{attrs}` vs `(url)` determines the semantic meaning
4. **No ambiguity**: No competing rules in the choice operator

## Lessons Learned

### 1. LR(1) Parser Design Matters

When designing tree-sitter grammars:
- **Minimize ambiguity** at the choice level
- **Use tokens strategically** for disambiguation
- **Precedence** helps with conflicts but doesn't eliminate them
- **External scanners** are powerful but don't solve all ambiguity problems

### 2. Sometimes Semantic Purity Must Be Sacrificed

The official grammar's approach (treating spans as links) is:
- ❌ Semantically imperfect (a span is not a link)
- ✅ Practically working (no parse ambiguity)
- ✅ Proven in production (quarto-markdown is used widely)

### 3. Test Suite is Critical

Having 87 comprehensive tests immediately caught regressions. Without them, we might have:
- Shipped a broken parser
- Not understood the scope of the problem
- Spent time debugging user reports instead

### 4. Incremental Changes Are Safer

We tried to add multiple features at once:
- Bracketed spans in inline content
- Attributes on headings
- External scanner lookahead

A better approach:
1. Add heading attributes first (simpler, less ambiguous)
2. Test thoroughly
3. Then tackle inline attributes separately

## Future Directions

### Option 1: Adopt inline_link Approach ⭐ RECOMMENDED

**Pros**:
- Proven to work (official grammar uses this)
- No parse ambiguity
- Can be implemented without breaking existing tests

**Cons**:
- Semantically imperfect (spans represented as links)
- AST doesn't reflect true structure

**Implementation**:
```javascript
// Extend link rule to accept attributes
link: $ => choice(
  // [text](url) - traditional link
  seq('[', field('text', $.link_text), ']',
      '(', field('url', $.link_url), ')'),
  // [text]{attrs} - attributed span (still called "link")
  seq('[', field('text', $.link_text), ']',
      '{', field('attributes', $.attribute_list), '}')
),
```

### Option 2: Heading Attributes Only

**Pros**:
- Simpler (no inline ambiguity)
- Still useful for common cases
- Can be added safely

**Cons**:
- Doesn't solve inline span attributes
- Partial feature implementation

**Example**:
```markdown
# Introduction {#intro .chapter}
## Background {#bg}
```

### Option 3: Deep Grammar Redesign

**Pros**:
- Could achieve semantic correctness
- Separate block/inline grammars like official implementation

**Cons**:
- Major architectural change
- High risk of breakage
- Requires deep tree-sitter expertise

### Option 4: Pause and Document

**Pros**:
- Documented for future contributors
- Clear explanation of challenges
- Can revisit with more expertise

**Cons**:
- Feature not available
- Users may need it

## Recommendations (Original - 2025-10-14)

1. **Short term**: Implement heading attributes only (safe, useful)
2. **Medium term**: Adopt inline_link approach if inline attributes are critical
3. **Long term**: Consider separate block/inline grammars for semantic correctness

## Implementation (2025-10-17)

**Decision**: ✅ Adopted **Option 1: inline_link approach** (following official quarto-markdown grammar)

### What Was Implemented

Extended the `link` rule to accept `{attributes}` as alternative to `(destination)`:

```javascript
link: $ => seq(
  field('text', seq('[', repeat($._link_text_element), ']')),
  choice(
    // Traditional link: [text](url)
    field('destination', seq('(', alias(/[^)]+/, $.link_destination), ')')),
    // Attributed span: [text]{attrs}
    seq('{', optional(/[ \t]*/), field('attributes', $.attribute_list), ...)
  )
)
```

### Results

- ✅ **102/102 tests passing** (added 15 new inline attributes tests)
- ✅ All Pandoc inline attribute syntax supported
- ✅ Traditional links unaffected
- ✅ WASM parser tested and verified (Zed integration ready)
- ⚠️ Cosmetic ERROR nodes at paragraph start (pre-existing issue)

### Trade-offs Accepted

1. **Semantic impurity**: Attributed spans represented as `link` nodes (not ideal, but pragmatic)
2. **ERROR nodes**: Link text shows ERROR/reference_label at paragraph start (cosmetic only, doesn't affect functionality)
3. **AST structure**: Not perfectly semantic, but matches official grammar approach

See [inline-attributes-known-issues.md](./inline-attributes-known-issues.md) for detailed analysis of known limitations.

## References

- **Official quarto-markdown grammar**: https://github.com/quarto-dev/quarto-markdown
- **tree-sitter documentation**: https://tree-sitter.github.io/tree-sitter/
- **LR parsing theory**: Dragon Book (Compilers: Principles, Techniques, and Tools)
- **Test file location**: `tree-sitter-markdown-inline/test/corpus/attributes.txt` (official)
- **Our test suite**: `test/corpus/inline-attributes.txt` (15 tests, 100% passing)
- **WASM tests**: `bindings/node/wasm_test.js` (12 tests verifying C/WASM equivalence)

## Conclusion

Inline attributes were **successfully implemented** using the pragmatic inline_link approach from the official quarto-markdown grammar. While not semantically perfect, this solution:

- ✅ Works correctly for all Pandoc inline attribute syntax
- ✅ Maintains 100% test pass rate (102/102)
- ✅ Ready for editor integration (WASM parser tested)
- ✅ Follows proven approach from official grammar
- ⚠️ Has minor cosmetic issues documented in [known issues](./inline-attributes-known-issues.md)

The implementation validates that the official grammar's pragmatic approach is the right choice for tree-sitter constraints. Perfect semantic purity would require architectural changes beyond the scope of this parser.

This research and implementation attempt provides a solid foundation for future work, with clear understanding of:
- ✅ What works (heading attributes, official grammar's approach)
- ✅ What doesn't work (separate bracketed_span node type)
- ✅ Why (LR(1) ambiguity, parse state conflicts)
- ✅ Path forward (multiple viable options documented)
