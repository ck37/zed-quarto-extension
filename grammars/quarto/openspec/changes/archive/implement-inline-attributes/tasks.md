# Tasks: Implement Inline Attributes

## 1. Grammar Implementation

### 1.1 Add Bracketed Span Rule
- [ ] 1.1.1 Add `bracketed_span` to `_inline_element` choice (before `link`)
- [ ] 1.1.2 Define `bracketed_span` rule: `[` + content + `]{` + attributes + `}`
- [ ] 1.1.3 Define `_span_content_element` choice (inline elements valid in spans)
- [ ] 1.1.4 Add conflict declaration for `[bracketed_span, link]` if needed
- [ ] 1.1.5 Test basic parsing: `npx tree-sitter parse` with `[text]{#id}`

**Deliverable:** `bracketed_span` node successfully parses basic attributes

### 1.2 Extend Heading Attributes
- [ ] 1.2.1 Modify `atx_heading` to accept optional trailing `attribute_list`
- [ ] 1.2.2 Modify `setext_heading` to accept optional trailing `attribute_list`
- [ ] 1.2.3 Test heading parsing: `# Title {#id .class}`
- [ ] 1.2.4 Ensure attributes don't interfere with heading content parsing

**Deliverable:** Headings parse correctly with and without attributes

### 1.3 Regenerate Parser
- [ ] 1.3.1 Run `npx tree-sitter generate` to regenerate parser
- [ ] 1.3.2 Check for conflicts and resolve if necessary
- [ ] 1.3.3 Verify src/parser.c and src/node-types.json updated
- [ ] 1.3.4 Commit grammar changes: "feat: add bracketed_span grammar rules"

**Deliverable:** Parser successfully regenerated with new rules

## 2. Test Suite Development

### 2.1 Create Test File
- [ ] 2.1.1 Create `test/corpus/inline-attributes.txt`
- [ ] 2.1.2 Add test: Basic ID attribute `[text]{#myid}`
- [ ] 2.1.3 Add test: Basic class attribute `[text]{.myclass}`
- [ ] 2.1.4 Add test: Multiple classes `[text]{.class1 .class2}`
- [ ] 2.1.5 Add test: Key-value attribute `[text]{key="value"}`
- [ ] 2.1.6 Add test: Mixed attributes `[text]{#id .class key="value"}`
- [ ] 2.1.7 Add test: Empty attributes `[text]{}`

**Deliverable:** 6-7 basic attribute tests passing

### 2.2 Advanced Span Tests
- [ ] 2.2.1 Add test: Span with emphasis `[**bold** text]{.highlight}`
- [ ] 2.2.2 Add test: Span with code `[see `code`]{.example}`
- [ ] 2.2.3 Add test: Nested spans `[[inner]{.a}]{.b}`
- [ ] 2.2.4 Add test: Multiple spans in paragraph
- [ ] 2.2.5 Add test: Span at start/end of paragraph

**Deliverable:** 5 advanced span tests passing

### 2.3 Heading Attribute Tests
- [ ] 2.3.1 Add test: ATX heading with ID `# Title {#section}`
- [ ] 2.3.2 Add test: ATX heading with class `## Subtitle {.intro}`
- [ ] 2.3.3 Add test: Setext heading with attributes
- [ ] 2.3.4 Add test: Heading with mixed attributes
- [ ] 2.3.5 Verify heading content not affected by attributes

**Deliverable:** 4-5 heading attribute tests passing

### 2.4 Run Full Test Suite
- [ ] 2.4.1 Run `npx tree-sitter test` - all tests pass
- [ ] 2.4.2 Verify 12-15 new tests added (87 → 99-102 total)
- [ ] 2.4.3 Check for any test regressions
- [ ] 2.4.4 Commit tests: "test: add inline attributes test coverage"

**Deliverable:** 100% test pass rate maintained, ~100 total tests

## 3. Syntax Highlighting

### 3.1 Add Highlight Queries
- [ ] 3.1.1 Open `queries/highlights.scm`
- [ ] 3.1.2 Add pattern for `bracketed_span` → `@markup.span`
- [ ] 3.1.3 Add pattern for `attribute_id` → `@constant`
- [ ] 3.1.4 Add pattern for `attribute_class` → `@tag`
- [ ] 3.1.5 Add pattern for `attribute_key` → `@property`
- [ ] 3.1.6 Add pattern for `attribute_value` → `@string`
- [ ] 3.1.7 Test highlighting with `npx tree-sitter highlight`

**Deliverable:** Span attributes highlighted correctly in test files

### 3.2 Verify Highlighting
- [ ] 3.2.1 Create sample file with various span attributes
- [ ] 3.2.2 Run `npx tree-sitter highlight sample.qmd`
- [ ] 3.2.3 Verify all attribute types highlighted
- [ ] 3.2.4 Check heading attributes highlighted
- [ ] 3.2.5 Commit: "feat: add syntax highlighting for inline attributes"

**Deliverable:** All attribute types have correct syntax highlighting

## 4. Documentation Updates

### 4.1 Update Project Status
- [ ] 4.1.1 Update `openspec/project.md` - add inline-attributes to specs
- [ ] 4.1.2 Update test count in project.md (87 → ~100)
- [ ] 4.1.3 Update README.md test badge if needed
- [ ] 4.1.4 Add inline attributes to CLAUDE.md feature list

**Deliverable:** Documentation reflects new capability

### 4.2 Update Grammar Documentation
- [ ] 4.2.1 Add inline attributes section to grammar.js comments
- [ ] 4.2.2 Document `bracketed_span` rule usage
- [ ] 4.2.3 Document heading attribute syntax
- [ ] 4.2.4 Add examples in comments

**Deliverable:** Grammar well-documented with examples

## 5. Validation & Finalization

### 5.1 OpenSpec Validation
- [ ] 5.1.1 Run `openspec validate implement-inline-attributes --strict`
- [ ] 5.1.2 Fix any validation errors
- [ ] 5.1.3 Verify all requirements have scenarios
- [ ] 5.1.4 Verify proposal, tasks, and spec deltas align

**Deliverable:** OpenSpec validation passes

### 5.2 Integration Testing
- [ ] 5.2.1 Test with real Quarto documents
- [ ] 5.2.2 Verify no conflicts with existing features
- [ ] 5.2.3 Test edge cases (long spans, special chars)
- [ ] 5.2.4 Check parse performance (<100ms for typical docs)

**Deliverable:** Parser handles real-world usage

### 5.3 CI/CD Verification
- [ ] 5.3.1 Push changes to branch
- [ ] 5.3.2 Verify GitHub Actions pass (Ubuntu + macOS)
- [ ] 5.3.3 Check Node 18.x and 20.x compatibility
- [ ] 5.3.4 Verify no warnings or errors in CI logs

**Deliverable:** CI green across all platforms

### 5.4 Final Review
- [ ] 5.4.1 Review all changes for consistency
- [ ] 5.4.2 Ensure commit messages follow conventions
- [ ] 5.4.3 Update change status in tasks.md (all ✓)
- [ ] 5.4.4 Prepare for archival after merge

**Deliverable:** Change ready for archival

## Notes

**Implementation Order:**
1. Grammar rules (1.1-1.3)
2. Basic tests (2.1)
3. Advanced tests (2.2-2.3)
4. Highlighting (3.1-3.2)
5. Documentation (4.1-4.2)
6. Validation (5.1-5.4)

**Key Dependencies:**
- Grammar must be working before tests
- Tests must pass before highlighting
- All technical work before documentation updates

**Testing Strategy:**
- Test incrementally as you implement each rule
- Use `npx tree-sitter parse /tmp/test.qmd` for quick checks
- Run full suite after each major change

**Estimated Time:**
- Grammar: 2-3 hours
- Tests: 1-2 hours
- Highlighting: 30 minutes
- Documentation: 1 hour
- Validation: 30 minutes
- Total: 5-7 hours
