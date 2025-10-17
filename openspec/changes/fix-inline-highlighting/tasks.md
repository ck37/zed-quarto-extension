# Tasks: Fix Inline Content Highlighting

## Phase 1: Investigation & Diagnosis

### 1. Create comprehensive test fixture
- [ ] Create `tests/fixtures/inline-highlighting.qmd` with all inline formatting patterns
- [ ] Include: emphasis, strong, combined, headings, strikethrough, highlight, subscript, superscript
- [ ] Include edge cases: nested elements, multiple words, special characters
- [ ] Add comments documenting expected highlighting behavior

**Validation:** File exists and can be opened in Zed

### 2. Test current highlighting behavior manually
- [ ] Open test fixture in Zed with extension installed
- [ ] Document which patterns work vs. which don't
- [ ] Take screenshots or detailed notes on observed behavior
- [ ] Check if markers are highlighted separately from content

**Validation:** Investigation document created in `docs/`

### 3. Analyze AST structure for test cases
- [ ] Use `tree-sitter parse` on test fixture to examine AST
- [ ] Verify that `(text)` nodes exist inside emphasis/strong/heading nodes
- [ ] Document the node hierarchy for each formatting pattern
- [ ] Confirm grammar structure matches query expectations

**Validation:** AST analysis shows correct node structure

### 4. Test highlight queries with tree-sitter-highlight
- [ ] Create test program using tree-sitter-highlight API
- [ ] Run `highlights.scm` queries against parsed test fixture
- [ ] Document which captures trigger for content vs. markers
- [ ] Identify if child text nodes receive proper captures

**Validation:** Test output shows which nodes are/aren't captured

## Phase 2: Fix Implementation

### 5. Update emphasis/strong queries
- [ ] Modify `languages/quarto/highlights.scm` emphasis queries
- [ ] Add explicit child captures if needed: `(emphasis (text) @text.emphasis)`
- [ ] Test with both `*` and `_` delimiter variants
- [ ] Add similar queries for `strong_emphasis`

**Validation:** `cargo test highlights_query_is_valid_syntax` passes

### 6. Update heading content queries
- [ ] Review current `content: (inline) @text.title` query
- [ ] Add explicit text captures if needed
- [ ] Test with ATX headings (levels 1-6)
- [ ] Test with setext headings

**Validation:** `cargo test highlights_query_is_valid_syntax` passes

### 7. Verify Pandoc extension queries
- [ ] Confirm strikethrough, highlight, subscript, superscript queries capture content
- [ ] Add child text captures if needed
- [ ] Test with simple and complex examples

**Validation:** Query syntax validation passes

### 8. Add priority rules if needed
- [ ] Determine if query ordering conflicts exist
- [ ] Add `(#set! "priority" N)` predicates to resolve conflicts
- [ ] Ensure parent styles don't get overridden by catch-all `(text) @text` query
- [ ] Test priority values incrementally

**Validation:** Highlighting works correctly in Zed

## Phase 3: Testing & Validation

### 9. Create automated highlight coverage test
- [ ] Add test in `tests/inline_highlighting.rs`
- [ ] Parse test fixture with tree-sitter-quarto
- [ ] Run highlight queries and collect captures
- [ ] Assert that content nodes receive expected scopes
- [ ] Assert that markers receive expected scopes

**Validation:** `cargo test inline_highlighting` passes

### 10. Test in Zed with clean install
- [ ] Run `./install-dev.sh` to rebuild and clean extension
- [ ] Restart Zed completely
- [ ] Reinstall dev extension
- [ ] Open test fixture and verify all patterns highlight correctly
- [ ] Test with multiple themes to ensure scope compatibility

**Validation:** All inline formatting displays correctly in Zed

### 11. Update documentation
- [ ] Document query patterns used for inline highlighting
- [ ] Update CLAUDE.md if needed
- [ ] Add notes to `docs/syntax-highlighting-architecture.md`
- [ ] Document any Zed-specific quirks discovered

**Validation:** Documentation is accurate and complete

### 12. Run full test suite
- [ ] `cargo test --workspace --all-features`
- [ ] Verify no regressions in other tests
- [ ] Check that WASM build still succeeds
- [ ] Validate query syntax with tree-sitter CLI

**Validation:** All tests pass

## Dependencies
- Task 2 depends on Task 1 (need test fixture)
- Task 3 depends on Task 1 (parse test fixture)
- Task 4 depends on Task 3 (need AST understanding)
- Tasks 5-7 can be done in parallel after Task 4
- Task 8 depends on Tasks 5-7 (priority rules after base queries)
- Task 9 depends on Tasks 5-8 (test after implementation)
- Task 10 depends on Task 9 (manual test after automated test)
- Tasks 11-12 can be done in parallel after Task 10

## Estimated Effort
- Phase 1 (Investigation): 2-3 hours
- Phase 2 (Implementation): 2-4 hours
- Phase 3 (Testing): 1-2 hours
- **Total:** 5-9 hours

## Success Metrics
- All inline formatting patterns display correctly in Zed
- Automated tests prevent regression
- Documentation helps future contributors understand inline highlighting
- No breaking changes to existing working patterns
