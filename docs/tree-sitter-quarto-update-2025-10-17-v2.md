# tree-sitter-quarto Update: October 17, 2025 (v2)

**Date**: 2025-10-17
**Commits**: bc21d45 → 54e3586 (8 commits)
**Tests**: 102 → 122 (+20 new tests)

## Summary

Updated tree-sitter-quarto from commit bc21d45 to 54e3586, adding 20 new tests and two major features: pipe table parsing and dual query file system.

## New Features Since Last Update

### 1. Pipe Table Parsing - **MAJOR**

**Commits**: feat: implement pipe table parsing

Full support for Pandoc pipe tables with external scanner detection.

**Syntax Support:**
```markdown
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
```

**Grammar Changes:**
- Added `pipe_table_header`, `pipe_table_delimiter`, `pipe_table_row` nodes
- External scanner token for table detection
- 8 new pipe table tests

**Impact**: Enables proper highlighting and structure for table content.

### 2. Dual Query File System - **MAJOR**

**Commits**: refactor: add dual query files for modern and legacy scope naming

Grammar now ships with TWO sets of highlight queries:

1. **`queries/highlights.scm`** - Modern nvim-treesitter scopes (@markup.*)
   - Compatible with Neovim, Helix, and other editors
   - Uses standardized scope naming conventions

2. **`queries/zed/highlights.scm`** - Legacy Zed scopes (@text.*, @emphasis.strong)
   - Specifically for Zed editor compatibility
   - Matches Zed's current theming system

**Extension Strategy:**
- Zed does NOT automatically load `queries/zed/` from grammars yet
- Issue [#1659](https://github.com/zed-industries/extensions/issues/1659) is still open
- **We must continue using extension query override** in `languages/quarto/highlights.scm`
- Updated our override to match grammar's `queries/zed/highlights.scm` (commit 54e3586)

**Key Difference**:
The only change from our previous version is:
```diff
-(pipe_table_row
-  "|" @punctuation.delimiter)
+(pipe_table_row) @none
+; Note: pipe_table_row is a token with no internal structure,
+; so we can't highlight individual delimiters within rows
```

### 3. Additional Test Coverage

**Commit**: test: add edge case test coverage

- 12 new tests for citations, shortcodes, code blocks, and enhanced divs
- Comprehensive coverage of edge cases
- Total tests: 102 → 114 → 122

## Grammar Capabilities (Full List)

✅ **Fully Implemented:**
- Executable code cells (`{python}`, `{r}`, `{julia}`)
- Chunk options (`#| key: value`)
- Cross-references (`@fig-plot`, `@tbl-data`)
- Inline code cells (`` `{python} expr` ``)
- Shortcodes (`{{< video url >}}`)
- Enhanced divs (callouts, tabsets, conditional content)
- Footnotes with full structured parsing
- Inline attributes (`[text]{.class}`)
- **NEW**: Pipe tables (`| header | header |`)
- Language injection (20+ languages)
- Full Pandoc Markdown support

## Extension Query Strategy

### Current Situation

**Zed's Query Loading (as of 2025-10-17):**
- Extensions provide queries in `languages/{language}/` directory
- Grammars ship with queries in `queries/` directory
- **Zed loads grammar's `queries/` by default**
- **Extension queries do NOT override grammar queries** (this is the bug)

**Grammar's Query Structure (NEW):**
```
tree-sitter-quarto/
├── queries/
│   ├── highlights.scm        # Modern @markup.* scopes (Neovim, Helix)
│   ├── injections.scm         # Language injections
│   ├── folds.scm              # Code folding
│   ├── indents.scm            # Indentation
│   ├── locals.scm             # Variable scoping
│   └── zed/
│       └── highlights.scm     # Legacy @text.* scopes (Zed-specific)
```

### Why We Keep Extension Override

1. **Zed doesn't auto-load `queries/zed/` subdirectory** - Feature not implemented yet
2. **Extension queries don't override** - Documented bug in Zed
3. **Our build.rs workaround** - Copies extension queries over grammar's for testing

### When Extension Override Becomes Unnecessary

When ONE of these happens:
1. Zed implements automatic loading of `queries/zed/` subdirectory ([#1659](https://github.com/zed-industries/extensions/issues/1659))
2. Zed allows extension queries to override grammar queries
3. Zed adopts modern @markup.* scope names in themes

## Query File Comparison

**Extension vs Grammar (54e3586):**
- Files are now **identical** in content
- Our `languages/quarto/highlights.scm` = Grammar's `queries/zed/highlights.scm`
- `injections.scm` unchanged (both versions identical)

## Testing Status

### Dependency Versions
- tree-sitter: 0.25.10
- web-tree-sitter: 0.25.10
- tree-sitter-quarto commit: **54e3586** (was bc21d45)

### Test Results

**Rust Tests**: ✅ **15/15 PASSING**
- Manifest validation
- Query validation
- Heading highlighting
- Code injection (Python, R)
- LSP smoke tests
- WASM extension builds

**WASM Integration Tests**: ✅ **12/12 PASSING**
- Grammar loading
- Document parsing (basic, callouts, advanced)
- Query loading and scope validation
- Highlighting correctness
- All Quarto features (code cells, cross-refs, inline code, footnotes, attributes)

**Total**: ✅ **27/27 tests passing**

## Known Limitations

From grammar README (unchanged):
- Citation lists `[@cite1; @cite2]` not supported
- ID attributes `{.class #id}` in inline spans have parsing issues
- Multi-line chunk option values not supported

## Migration Path

### Current Branch Status

Branch: `tree-sitter-quarto-migration`
Status: **Production-ready, waiting on Zed**

**Blockers:**
1. Zed doesn't load `queries/zed/` from grammars → [#1659](https://github.com/zed-industries/extensions/issues/1659)
2. Extension queries don't override grammar queries → Documented issue

**Workarounds in Place:**
1. Extension provides override in `languages/quarto/highlights.scm`
2. Build script copies extension queries for test validation
3. All features work correctly in real Zed usage

### When to Merge to Main

✅ Ready to merge when:
- [ ] Zed implements `queries/zed/` subdirectory loading, OR
- [ ] Zed allows extension query overrides, OR
- [ ] We're satisfied with current workaround and want to ship

⚠️ Alternative: Merge now with documentation about query override requirement.

## Next Steps

1. ✅ ~~Update to latest grammar commit~~ Done - 54e3586
2. ✅ ~~Update extension queries~~ Done - matches grammar's zed/ version
3. ✅ ~~Test with WASM and Rust tests~~ Done - all 27 tests passing
4. **Wait for Zed query loading fix** OR **Merge with current workaround**
5. **Document** pipe table support for users
6. **Consider filing upstream issues** for grammar limitations:
   - Citation list support
   - ID attribute support

## References

- Grammar repo: https://github.com/ck37/tree-sitter-quarto
- Grammar commit: 54e3586bc0238f5a7d41ca4ebb4916504263b1a9
- Zed query loading issue: https://github.com/zed-industries/extensions/issues/1659
- Previous update doc: docs/tree-sitter-quarto-update-2025-10-17.md
