# Proposal: Add Comprehensive Test Coverage

## Why

The official Quarto markdown parser (quarto-dev/quarto-markdown) has extensive test coverage for Pandoc Markdown features that our grammar already supports but we don't test. Analysis of their test corpus reveals critical gaps in our test suite:

**Currently missing tests for:**
- Footnotes (inline and reference-style)
- Inline attributes (`{#id}`, `{.class}`, `{key=value}`)
- Pipe tables (basic, aligned, escaped pipes)
- Citation variations (author suppression, bracketed citations)
- Shortcode edge cases (nested, escaped, quoted arguments)
- Code block attributes beyond executable cells

**Grammar capabilities confirmed:**
- `footnote_definition` rule exists in grammar.js
- `attribute_list` rule supports id, class, and key-value attributes
- `pipe_table*` rules support full pipe table syntax
- All features are parseable, just untested

**Additional cleanup opportunity:**
- Remove mermaid and dot language injections (diagram specs, not executable code)

## What Changes

### 1. Add New Test Suites

Create four new test corpus files covering untested Pandoc features:

1. **`test/corpus/footnotes.txt`**
   - Inline footnotes: `^[note]`
   - Footnote references: `[^1]`
   - Nested footnotes
   - Footnote definitions

2. **`test/corpus/inline-attributes.txt`**
   - ID attributes: `[text]{#id}`
   - Class attributes: `[text]{.class1 .class2}`
   - Key-value attributes: `[text]{key="value"}`
   - Mixed attributes: `[text]{#id .class key="value"}`
   - Escaped quotes in values

3. **`test/corpus/pipe-tables.txt`**
   - Basic pipe tables with headers
   - Column alignment (left, center, right)
   - Escaped pipes in cells: `\|`
   - Empty and whitespace-only cells
   - Tables followed by other blocks

4. **`test/corpus/test-refinements.txt`**
   - Citation variations (suppress author: `-@cite`, bracketed: `@{url}`)
   - Shortcode edge cases (nested, escaped, special quotes)
   - Code block attributes for non-executable blocks
   - Fenced div edge cases

### 2. Clean Up Language Injections

Remove non-executable language injections:
- Delete mermaid injection rules (lines 100-107 in queries/injections.scm)
- Delete dot/graphviz injection rules (lines 109-116 in queries/injections.scm)

Rationale: These are diagram specification languages, not executable computation code. Quarto is primarily used for data science (Python/R/Julia/SQL), not diagram rendering.

## Impact

### Benefits
- **Comprehensive coverage** - Align test suite with official Quarto parser standards
- **Regression prevention** - Catch breaking changes to existing Pandoc features
- **Documentation** - Test corpus serves as usage examples
- **Confidence** - Verify all grammar rules are working correctly
- **Focused injections** - Remove non-core language injections

### Scope
- **No grammar changes** - All features already supported, only adding tests
- **No breaking changes** - Pure additive (tests + cleanup)
- **Low risk** - Test-only changes don't affect parser behavior
- **Quick win** - Can implement incrementally by test file

### Metrics
- **Before:** 58 tests across 6 test files
- **After:** ~90-110 tests across 10 test files (estimated +32 to +52 tests)
- **Coverage:** Tests all documented grammar rules instead of subset
- **CI impact:** Minimal (<10s additional test time)

## Implementation Phases

### Phase 1: High Priority (Core Pandoc Features)
1. Footnotes - Critical Markdown feature
2. Inline attributes - Used extensively in Pandoc/Quarto
3. Pipe tables - Common table format

### Phase 2: Refinements (Edge Cases & Cleanup)
4. Test refinements - Citation/shortcode edge cases
5. Language injection cleanup - Remove mermaid/dot

### Rollout Strategy
- Each test file is independent, can be added incrementally
- Each new test file goes through standard CI (58→70→82→90+ tests)
- Language injection cleanup after Phase 1 complete
- All tests must pass before merging

## Success Criteria

✅ 90-110 total tests passing (100% success rate)
✅ All grammar rules have at least one test case
✅ CI passes on Ubuntu + macOS, Node 18.x + 20.x
✅ Test execution time remains under 1 second average
✅ Language injections focused on executable code languages only

## Non-Goals

- Not adding new grammar features
- Not changing parser behavior
- Not modifying existing test assertions
- Not testing features we don't support (generic fenced divs remain a known limitation)
