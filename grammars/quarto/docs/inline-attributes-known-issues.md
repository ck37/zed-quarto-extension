# Inline Attributes - Known Issues

## Link Text ERROR Nodes (Cosmetic Issue)

### Summary

When inline attributes appear at the **start of a paragraph**, the parser creates cosmetic ERROR nodes in the link text. This does **not** affect functionality - attributes parse correctly in all cases.

### What Happens

**Standalone at paragraph start:**
```markdown
[text]{.class}
```

Parse tree:
```
(link
  (ERROR              ← Cosmetic issue
    (reference_label))
  attributes: (attribute_list
    class: (attribute_class)))  ← ✅ Works correctly
```

**In context (surrounded by text):**
```markdown
Here is [text]{.class} in a sentence.
```

Parse tree:
```
(link
  text: (link_text)   ← ✅ Clean, no ERROR
  attributes: (attribute_list
    class: (attribute_class)))  ← ✅ Works correctly
```

### Why This Happens

1. **Parser ambiguity**: When the parser encounters `[text]` at paragraph start, it considers multiple patterns:
   - Regular link: `[text](...)`
   - Reference-style link: `[text][ref]` or `[text]`
   - Footnote definition: `[^1]:`
   - Attributed span: `[text]{attrs}` (our new feature)

2. **Speculative parsing**: The parser speculatively tries to match `reference_label` (from footnote definitions).

3. **Error recovery**: When `{.class}` appears instead of expected reference syntax, the parser creates an ERROR node around the failed `reference_label` attempt but continues parsing successfully.

4. **Context resolves ambiguity**: When surrounded by text, the parser has enough context to correctly identify `link_text` from the start - no ERROR nodes.

### Impact Assessment

**Functionality:** ✅ **No impact**
- Attributes parse correctly in all cases
- Traditional links still work
- All 102 tests pass

**Editor Integration:** ✅ **No impact**
- Syntax highlighting works (query patterns can match either path)
- The `attributes` field is always correct
- Editor features (autocomplete, jump-to-definition) unaffected

**Cosmetic Only:** ⚠️ **Minor**
- ERROR appears in parse tree but doesn't block functionality
- Some syntax highlighters may show incorrect colors at paragraph start
- Error disappears when text added before the span

### Pre-existing Issue

This is **not** introduced by inline attributes implementation. The ambiguity exists in the base grammar for reference-style links.

Verified by:
1. Testing with `git stash` before inline attributes implementation
2. Comparing parse trees with official quarto-markdown grammar (same behavior)
3. The WASM and C parsers produce identical results (same grammar.js source)

### Workarounds

For users experiencing highlighting issues:

**Option 1: Add leading text**
```markdown
<!-- Instead of: -->
[text]{.class}

<!-- Use: -->
The [text]{.class} appears here.
```

**Option 2: Use heading attributes (no ERROR)**
```markdown
# Heading {#id .class}  ← Always works cleanly
```

**Option 3: Ignore it**
- The ERROR is cosmetic only
- Attributes work correctly regardless
- Most users won't notice

### Technical Details

**Grammar location:** `grammar.js:538-551`

The `link` rule accepts a choice:
```javascript
link: $ => seq(
  field('text', seq('[', repeat($._link_text_element), ']')),
  choice(
    field('destination', seq('(', alias(/[^)]+/, $.link_destination), ')')),
    seq('{', optional(/[ \t]*/), field('attributes', $.attribute_list), ...)
  )
)
```

The `_link_text_element` allows multiple content types but can't disambiguate `[text]` patterns without lookahead.

**Reference label definition:** `grammar.js` (footnote definitions)
```javascript
field('label', seq('[', alias(/[^\]]+/, $.reference_label), ']:'))
```

### Future Resolution

This could be resolved by:

1. **Deep grammar redesign** - Separate block/inline parsers (like official quarto-markdown)
2. **External scanner** - C code for context-aware lookahead
3. **Wait for official grammars** - Migrate to quarto-markdown in 2026+ when production-ready

Current recommendation: **Accept this limitation** - the impact is minimal and a fix would require significant architectural changes.

### Testing

See test file: `test/corpus/inline-attributes.txt`

Tests verify:
- ✅ Attributes parse correctly with ERROR nodes present
- ✅ Attributes parse correctly without ERROR nodes (in context)
- ✅ Traditional links unaffected
- ✅ All 15 inline attribute test cases pass

WASM testing: `bindings/node/wasm_test.js`
- ✅ 12 WASM tests verify C/WASM parser equivalence
- ✅ Confirms this behavior matches what Zed editor will see

## Other Known Limitations

None currently. Inline attributes implementation is feature-complete.

### Unsupported Syntax

All Pandoc inline attribute syntax is supported:
- ✅ `.class` - Single class
- ✅ `#id` - ID attribute
- ✅ `#id .class1 .class2` - ID with multiple classes
- ✅ `key="value"` - Key-value attributes
- ✅ `key='value'` - Single-quoted values
- ✅ Mixed: `#id .class key="value"`
- ✅ Nested formatting: `[*emphasized*]{.class}`
- ✅ Code spans: `` [`code`]{.highlight} ``
- ✅ In executable cells: `` ```{python #fig-id .class} ``

## Related Documentation

- [Implementation challenges](./inline-attributes-implementation-challenges.md) - Design decisions and alternatives considered
- [WASM test results](../WASM_PARSER_TEST_RESULTS.md) - Detailed WASM parser behavior
- [Test corpus](../test/corpus/inline-attributes.txt) - All 15 test cases with expected AST
