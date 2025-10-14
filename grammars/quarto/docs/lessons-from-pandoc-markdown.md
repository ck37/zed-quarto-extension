# Lessons from tree-sitter-pandoc-markdown

This document captures key insights from the [tree-sitter-pandoc-markdown](https://github.com/ck37/tree-sitter-pandoc-markdown) project that inform tree-sitter-quarto's design.

## Core Project Rationale

### Why Fork/Extend?

tree-sitter-pandoc-markdown exists because:
1. **Standard markdown parsers are insufficient** - CommonMark doesn't include Pandoc extensions
2. **Editor integration requires semantic nodes** - Need distinct node types for citations, divs, spans, etc.
3. **LR(1) limitations require careful design** - Some features (like definition lists) are impossible to implement

### Parallel for tree-sitter-quarto

tree-sitter-quarto extends this rationale:
1. **Pandoc markdown parsers lack Quarto features** - Chunk options, cross-references, executable cells
2. **Editors need Quarto-specific semantics** - Distinguish `@fig-plot` from `@citation`, parse `#| key: value`
3. **Pre-execution parsing is different** - Parse raw `.qmd` files before knitr/jupyter runs

## Key Architectural Decisions

### 1. Split Grammar (Block vs Inline)

**Decision:** Separate block-level and inline-level grammars

**Rationale from pandoc-markdown:**
- Reduces parsing complexity
- Allows independent evolution of block and inline features
- Matches how Pandoc internally processes Markdown

**Application to tree-sitter-quarto:**
- `tree-sitter-quarto-block/` - Executable cells, chunk options, enhanced divs
- `tree-sitter-quarto-inline/` - Cross-references, inline code cells, citations

### 2. External Scanner Strategy

**Decision:** Use external scanner (C code) for complex/ambiguous syntax

**When needed (from pandoc-markdown experience):**
- Lookahead beyond LR(1) capability
- Context-sensitive parsing
- Disambiguation that requires examining future lines

**Likely needs for tree-sitter-quarto:**
- **Chunk option detection:** Distinguish `#| key: value` from `# comment` at cell start
- **Cell boundary detection:** Determine when ` ``` ` ends a cell vs starts nested code
- **Multi-line chunk values:** Handle `|` continuation in chunk options

### 3. Phased Implementation

**Approach from pandoc-markdown:**
```
Phase 1: Core grammar (basic Markdown)
Phase 2: External scanner features (complex elements)
Phase 3: Future enhancements
```

**Adapted for tree-sitter-quarto:**
```
Phase 1: Foundation
  - Executable code cells (basic)
  - Simple chunk options (single-line)
  - Cross-reference distinction
  - Inline code cells

Phase 2: External Scanner Features
  - Multi-line chunk options
  - Cell boundary disambiguation
  - Nested code blocks in cells

Phase 3: Advanced Features
  - Chunk option validation
  - Language-specific cell features
  - Cross-reference validation
```

## Known Limitations (LR(1) Constraints)

### From pandoc-markdown

**Impossible to implement:**
- **Definition lists** - "Definition terms are structurally identical to paragraphs until the next line is examined"
- Cannot look ahead to determine if a paragraph becomes a definition term

### Potential Issues for tree-sitter-quarto

**Watch out for:**
1. **Chunk options vs comments:**
   ```python
   ```{python}
   # This is a comment
   #| This is a chunk option
   ```
   - Both start with `#`, need to distinguish at parse time
   - **Solution:** External scanner to check if at cell start

2. **Cross-references vs citations:**
   ```markdown
   @fig-plot    → cross-reference (has type prefix)
   @smith2020   → citation (no type prefix)
   ```
   - **Solution:** Pattern matching can distinguish these (no external scanner needed)

3. **Inline code cells vs code spans:**
   ```markdown
   `{python} expr`  → inline code cell
   `code`           → code span
   ```
   - **Solution:** Check for `{lang}` pattern immediately after opening backtick

## Testing Strategy

### From pandoc-markdown

**Approach:**
1. Start with minimal test cases
2. Use `tree-sitter parse --debug` for debugging
3. Add edge cases incrementally
4. Test feature interactions
5. Validate with real-world documents
6. Maintain pass rate on existing tests (80/80)
7. Performance testing
8. Fuzzing for edge cases

### Applied to tree-sitter-quarto

**Testing phases:**
1. **Unit tests** (test/corpus/)
   - One test per feature
   - Edge cases for each feature
   - Feature interaction tests

2. **Integration tests**
   - Parse `examples/sample.qmd`
   - Parse real Quarto documents from quarto-web
   - Ensure no parse errors

3. **Performance tests**
   - Parse 1000-line document in <100ms
   - Incremental parsing works correctly

4. **Editor validation**
   - Test in Neovim with nvim-treesitter
   - Test in Zed editor
   - Test in Helix

## Implementation Order

### From pandoc-markdown experience

**Principle:** "Start simple, add complexity incrementally"

**Order considerations:**
1. Core features first (most common syntax)
2. Complex features later (require external scanner)
3. Validation features last (require complete grammar)

### Recommended order for tree-sitter-quarto

**Week 1-2: Core Grammar**
1. Basic executable cells (no chunk options)
   ```python
   ```{python}
   code here
   ```
   ```
2. Language specifier parsing (`{python}`, `{r}`, etc.)
3. Simple cross-references (`@fig-plot` vs `@citation`)

**Week 3-4: Chunk Options**
1. Single-line chunk options
   ```python
   #| label: fig-plot
   #| echo: false
   ```
2. External scanner for `#|` detection
3. Key-value parsing

**Week 5-6: Advanced Features**
1. Multi-line chunk option values
2. Inline code cells
3. Enhanced divs (callouts, tabsets)

## External Scanner Design Principles

### From pandoc-markdown

**When to use external scanner:**
- Syntax requires arbitrary lookahead
- Context-sensitive parsing (same token means different things)
- Disambiguation requires examining multiple lines

**When NOT to use external scanner:**
- Simple pattern matching works
- Grammar rules can express it
- Lexical analysis is sufficient

### For tree-sitter-quarto

**Definitely need external scanner:**
- Chunk option detection (`#|` at cell start vs `#` comment)
- Cell boundary detection (nested ` ``` ` blocks)
- Multi-line chunk value continuation (`|`)

**Probably DON'T need external scanner:**
- Cross-reference patterns (`@fig-id` vs `@id`)
- Language specifiers (`{python}`, `{r}`)
- Basic callout types (`.callout-note`)

## Grammar Extension Pattern

### From pandoc-markdown

**Pattern:** Extend tree-sitter-markdown, don't fork

**Approach:**
```javascript
const markdown = require('tree-sitter-markdown');

module.exports = grammar(markdown, {
  name: 'pandoc_markdown',
  rules: {
    // Extend existing rules
    _inline: $ => choice(
      ...markdown.rules._inline,
      $.citation,
      $.cross_reference
    )
  }
});
```

### For tree-sitter-quarto

**Pattern:** Extend tree-sitter-pandoc-markdown, don't fork

**Approach:**
```javascript
// Copy base grammar from tree-sitter-pandoc-markdown
const pandoc = require('./pandoc-markdown-base');

module.exports = grammar(pandoc, {
  name: 'quarto',
  externals: $ => [
    ...pandoc.externals,
    $.chunk_option_marker,  // #| at cell start
    $.cell_boundary         // ``` with context
  ],
  rules: {
    // Extend block rules
    _block: $ => choice(
      ...pandoc.rules._block,
      $.executable_code_cell
    ),

    // Extend inline rules
    _inline: $ => choice(
      ...pandoc.rules._inline,
      $.inline_code_cell
    )
  }
});
```

## Success Criteria

### From pandoc-markdown

- Working external scanner
- No test regressions (80/80 tests pass)
- New corpus tests for new features
- Updated documentation
- Maintained parsing performance

### For tree-sitter-quarto

**Phase 1 Success:**
- [ ] Parse executable cells with language specifiers
- [ ] Parse single-line chunk options
- [ ] Distinguish cross-references from citations
- [ ] All tests passing (20+ test cases)
- [ ] Basic syntax highlighting works

**Phase 2 Success:**
- [ ] Multi-line chunk options
- [ ] Inline code cells
- [ ] Callouts and tabsets
- [ ] 50+ test cases passing
- [ ] Advanced highlighting queries

**Phase 3 Success:**
- [ ] Parse quarto-web without errors
- [ ] Performance: <100ms for typical documents
- [ ] Editor integration in 3+ editors
- [ ] Documentation complete

## Key Takeaways

1. **Be realistic about LR(1) limitations** - Some syntax is impossible, design around it
2. **Use external scanner strategically** - Only when grammar rules can't express it
3. **Test incrementally** - Start simple, add complexity gradually
4. **Validate with real documents** - Theory meets practice in actual files
5. **Performance matters** - Editors need fast, incremental parsing
6. **Document design decisions** - Future maintainers need to understand why
7. **Extend, don't fork** - Build on existing work when possible

## Questions to Answer Before Implementation

Based on pandoc-markdown experience:

1. **External scanner scope:**
   - What exactly needs the external scanner?
   - Can we minimize external scanner usage?

2. **Grammar complexity:**
   - Can chunk options be parsed without external scanner?
   - How do we handle nested ` ``` ` blocks?

3. **Feature prioritization:**
   - What features are essential for MVP?
   - What can wait for later phases?

4. **Testing strategy:**
   - What corpus tests do we need?
   - How do we test in actual editors?

5. **Performance targets:**
   - What's acceptable parse time?
   - How do we measure incremental parsing performance?

## Resources

- [tree-sitter-pandoc-markdown plan](https://github.com/ck37/tree-sitter-pandoc-markdown/blob/zed-compatible-scopes/docs/plan.md)
- [Tree-sitter documentation](https://tree-sitter.github.io/tree-sitter/)
- [LR parsing limitations](https://en.wikipedia.org/wiki/LR_parser)
