# Proposal: Implement Inline Attributes

> **✅ COMPLETED:** 2025-10-17
> **Implementation:** Successfully implemented using link-based approach from official quarto-markdown grammar
> **Test Coverage:** 15 passing tests in `test/corpus/inline-attributes.txt`
> **Status:** Archived - All features working, known cosmetic issue documented in `docs/inline-attributes-known-issues.md`

## Why

Pandoc Markdown supports inline attributes (also called "bracketed spans") using the syntax `[text]{#id .class key=value}`. This feature allows authors to add IDs, CSS classes, and custom attributes to arbitrary spans of text within a document.

**Current Status:**
- Grammar has `attribute_list` rules (lines 444-465 in grammar.js)
- Rules used for executable cells and enhanced divs
- **NOT** implemented for inline spans - `[text]{#id}` produces parser ERRORs
- Missing `bracketed_span` node type in inline elements

**Use Cases:**
- Style specific text: `[important]{.highlight}`
- Add anchors: `[section text]{#anchor-name}`
- Custom attributes: `[data]{data-value="123"}`
- Semantic markup: `[term]{.definition}`
- Cross-references targets

**Gap:**
Quarto documents frequently use inline attributes for styling and semantic markup, but the parser currently cannot parse them, leading to poor editor experience (no syntax highlighting, folding, or validation for this common pattern).

## What Changes

### 1. Add Bracketed Span Node
Create new `bracketed_span` inline node type:
- Pattern: `[` + inline_content + `]` + `{` + attributes + `}`
- Content can include other inline elements (emphasis, code, etc.)
- Attributes use existing `attribute_list` rule

### 2. Integrate with Inline Elements
Add `bracketed_span` to `_inline_element` choice:
- Must come before `link` to avoid conflict with `[text](url)` syntax
- Link check happens after span check fails (no trailing `{attrs}`)

### 3. Handle Heading Attributes
Extend `atx_heading` and `setext_heading` to support trailing attributes:
- Pattern: `# Heading {#id .class}`
- Attributes are optional after heading content

### 4. Add Syntax Highlighting
Update `queries/highlights.scm`:
- `bracketed_span` as `@markup.span`
- Attribute IDs as `@constant`
- Attribute classes as `@tag`
- Attribute keys/values as `@property`/`@string`

### 5. Add Test Coverage
Create comprehensive test suite in `test/corpus/inline-attributes.txt`:
- Basic spans with ID, class, key-value attributes
- Mixed attributes
- Empty attribute lists
- Spans with inline formatting (bold, code, etc.)
- Heading attributes
- Edge cases (nested brackets, escaped chars)

## Impact

**Benefits:**
- ✅ Full Pandoc Markdown compatibility for inline spans
- ✅ Enable syntax highlighting for span attributes
- ✅ Support common Quarto styling patterns
- ✅ Foundation for attribute validation in language server

**Affected Components:**
- `grammar.js`: Add `bracketed_span` rule, extend headings
- `queries/highlights.scm`: Add highlighting patterns
- `test/corpus/inline-attributes.txt`: New test file
- Specs: New `inline-attributes` capability spec

**Breaking Changes:** None
- Purely additive change
- No modification to existing node types
- Backward compatible

**Scope:**
- ~50-80 lines grammar changes
- ~20-30 lines highlight queries
- ~12-15 test cases
- Estimated 4-6 hours implementation time

## Success Criteria

✅ `[text]{#id}` parses as `bracketed_span` with `attribute_id`
✅ `[text]{.class}` parses with `attribute_class`
✅ `[text]{key="value"}` parses with `key_value_attribute`
✅ `# Heading {#id}` parses heading with attributes
✅ All test cases pass (12-15 new tests)
✅ No regressions in existing 87 tests
✅ Syntax highlighting works for span attributes
✅ `openspec validate --strict` passes

## Non-Goals

- Not implementing link attributes `[text](url){attrs}` (different Pandoc feature)
- Not implementing image attributes `![alt](url){attrs}` (separate feature)
- Not implementing div attributes (already supported via enhanced divs)
- Not implementing validation logic (belongs in language server)
- Not implementing multi-line spans (Pandoc limitation)
