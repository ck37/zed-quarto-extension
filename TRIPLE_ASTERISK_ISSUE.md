# Triple asterisks (***text***) not supported for bold+italic

## Problem

Triple asterisks for combined bold and italic (`***text***`) produce an ERROR node instead of being parsed as nested emphasis.

```markdown
***bold and italic***
```

### Current behavior

The grammar produces:
```
ERROR [0..21]
  ** [0..2]
  * [2..3]
  text [3..18] "bold and italic"
  ** [18..20]
  * [20..21]
```

### Expected behavior

Should parse as nested emphasis (either emphasis containing strong_emphasis, or strong_emphasis containing emphasis):

```
inline
  emphasis [0..21]
    * [0..1]
    strong_emphasis [1..20]
      ** [1..3]
      text [3..18] "bold and italic"
      ** [18..20]
    * [20..21]
```

## Workarounds that already work

Mixed delimiters work correctly:
- `**_bold and italic_**` ✅ (strong containing emphasis)
- `*__bold and italic__*` ✅ (emphasis containing strong)

## Context

This is valid Pandoc/CommonMark syntax. Triple underscores (`___text___`) have the same issue.

## Technical analysis

The issue appears to be related to tree-sitter's tokenization of `**` as an atomic token for `strong_emphasis` before considering the `emphasis` rule. Attempts to fix this in grammar.js using precedence rules and dynamic precedence have not succeeded:

```javascript
emphasis: $ => choice(
  // Attempted fix with high precedence
  prec.dynamic(10, prec(3, seq('*', $.strong_emphasis, '*'))),
  prec.dynamic(10, prec(3, seq('_', $.strong_emphasis, '_'))),
  // Standard rules
  prec.left(1, seq('*', repeat1($._inline_no_star), '*')),
  prec.left(1, seq('_', repeat1($._inline_no_underscore), '_'))
),
```

This might require changes to the scanner.c to handle delimiter runs differently, similar to how the CommonMark spec handles emphasis delimiter runs.

## Test case

```rust
use tree_sitter::{Language, Parser};

#[test]
fn triple_asterisk_should_parse_as_nested_emphasis() {
    let source = "***bold and italic***";

    let mut parser = Parser::new();
    parser.set_language(&inline_language()).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    let root = tree.root_node();

    // Should NOT have ERROR nodes
    assert!(!has_error_node(&root), "Triple asterisks should parse without ERROR nodes");

    // Should have nested structure: emphasis > strong_emphasis > text
    // OR: strong_emphasis > emphasis > text
}
```

## Impact

Medium priority - the workaround (`**_text_**`) works well and is valid Pandoc syntax, but triple asterisks are commonly used and expected to work.

## References

- CommonMark spec on emphasis: https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis
- Pandoc manual on emphasis: https://pandoc.org/MANUAL.html#emphasis
