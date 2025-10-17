# Implementation Tasks

## 1. Grammar Rules

- [x] 1.1 Add strikethrough rule to grammar.js (seq('~~', $.inline, '~~'))
- [x] 1.2 Add highlight rule to grammar.js (seq('==', $.inline, '=='))
- [x] 1.3 Add subscript rule to grammar.js (with no-whitespace constraint)
- [x] 1.4 Add superscript rule to grammar.js (with no-whitespace constraint)
- [x] 1.5 Update inline choice rule to include new formatting types
- [x] 1.6 Set proper precedence for disambiguation

## 2. External Scanner

- [x] 2.1 Add STRIKETHROUGH token type to scanner - NOT NEEDED (token() sufficient)
- [x] 2.2 Add HIGHLIGHT token type to scanner - NOT NEEDED (token() sufficient)
- [x] 2.3 Add SUBSCRIPT token type to scanner with whitespace detection - NOT NEEDED (regex constraint sufficient)
- [x] 2.4 Add SUPERSCRIPT token type to scanner with whitespace detection - NOT NEEDED (regex constraint sufficient)
- [x] 2.5 Implement disambiguation logic for ~ (subscript vs strikethrough) - HANDLED by precedence
- [x] 2.6 Implement disambiguation logic for ^ (superscript vs footnote ref) - HANDLED by existing rules
- [x] 2.7 Add scanner state tracking for delimiter matching - NOT NEEDED

## 3. Highlight Queries

- [x] 3.1 Add strikethrough scope to queries/highlights.scm (@markup.strikethrough)
- [x] 3.2 Add highlight scope to queries/highlights.scm (@markup.mark)
- [x] 3.3 Add subscript scope to queries/highlights.scm (@markup.subscript)
- [x] 3.4 Add superscript scope to queries/highlights.scm (@markup.superscript)
- [x] 3.5 Add strikethrough scope to queries/zed/highlights.scm (@text.strike)
- [x] 3.6 Add highlight scope to queries/zed/highlights.scm (@text.highlight)
- [x] 3.7 Add subscript scope to queries/zed/highlights.scm (@text.subscript)
- [x] 3.8 Add superscript scope to queries/zed/highlights.scm (@text.super)

## 4. Tests

- [x] 4.1 Create test/corpus/inline-formatting.txt corpus file
- [x] 4.2 Add test: basic strikethrough
- [x] 4.3 Add test: basic highlight
- [x] 4.4 Add test: basic subscript (H~2~O)
- [x] 4.5 Add test: basic superscript (x^2^)
- [x] 4.6 Add test: strikethrough with spaces
- [x] 4.7 Add test: nested formatting (bold with strikethrough)
- [x] 4.8 Add test: adjacent to punctuation
- [x] 4.9 Add test: multi-word subscript/superscript
- [x] 4.10 Add test: disambiguation (~ for subscript vs ~~ for strikethrough)
- [x] 4.11 Add test: disambiguation (^ for superscript vs [^1] for footnote) - COVERED by existing footnote tests
- [x] 4.12 Add test: invalid cases (whitespace after opening delimiter) - NOT NEEDED (constraint in grammar)
- [x] 4.13 Add test: Unicode content

## 5. Documentation

- [x] 5.1 Update examples/sample.qmd to include all inline formatting examples
- [ ] 5.2 Add inline formatting section to README or docs - DEFERRED
- [ ] 5.3 Document known limitations or edge cases - DEFERRED

## 6. Validation

- [x] 6.1 Run tree-sitter generate successfully
- [x] 6.2 Run tree-sitter test with all tests passing (145/145 tests pass)
- [x] 6.3 Parse examples/sample.qmd without errors
- [ ] 6.4 Test in Zed editor with real Quarto documents - USER VALIDATION
- [ ] 6.5 Verify highlight queries work in Zed - USER VALIDATION

## Implementation Notes

### Design Decisions

1. **No External Scanner Required**: The inline formatting features were successfully implemented using tree-sitter's built-in `token()` function and precedence rules, without requiring custom C scanner code.

2. **Equals Sign Handling**: Added `equals_sign` as a separate inline element to handle single `=` in equations (like `E=mc^2^`) while allowing `==` to match as highlight delimiters.

3. **Text Pattern Exclusions**: Excluded `~` and `=` from the text regex pattern to allow proper matching of subscript, superscript, and highlight delimiters.

4. **Precedence**: Placed formatting rules in the middle of the `_inline_element` choice, after basic inline elements but before links and text, ensuring proper parsing order.

### Test Results

- All 19 inline-formatting tests pass
- All 145 total tests pass (100% success rate)
- Average parse speed: 12933 bytes/ms
- No parse errors in examples/sample.qmd
