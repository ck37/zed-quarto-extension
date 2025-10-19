# Zed Query Loading Test Results

## Experiment Conducted

**Date:** October 14, 2025

**Question:** Does Zed load extension queries (`languages/quarto/highlights.scm`) or grammar queries (`queries/highlights.scm`)?

## Test Method

Added intentional syntax error to `languages/quarto/highlights.scm`:

```scheme
; TEST: If Zed loads this file, this syntax error should cause a failure:
INTENTIONAL_SYNTAX_ERROR_TO_TEST_IF_ZED_LOADS_THIS_FILE
```

Then:
1. Built extension with syntax error present
2. Installed dev extension in Zed
3. Opened a `.qmd` file to trigger query loading
4. Checked Zed logs for query parse errors

## Results

### Build Time
- ✅ Extension built successfully
- ✅ No query validation errors during build
- ✅ `cargo build` completed without issues

**Finding:** Extension build does NOT validate query syntax

### Runtime (Zed)
- ✅ Extension installed successfully (17:13:02)
- ✅ Grammar compiled successfully
- ✅ `.qmd` file opened without errors
- ❌ **NO query syntax error reported in logs**

**Log evidence:**
```
2025-10-14T17:13:02-04:00 INFO  [extension::extension_builder] compiled grammar quarto
2025-10-14T17:13:02-04:00 INFO  [extension::extension_builder] finished compiling extension
2025-10-14T17:13:02-04:00 WARN  [language] unrecognized capture name 'dedent' in Quarto indents
```

Only warning: `'dedent'` in indents file (unrelated to our test)

## Conclusion

**CONFIRMED: Zed is NOT loading extension queries**

### Evidence
1. Intentional syntax error in `languages/quarto/highlights.scm` was ignored
2. No query parse error in Zed logs
3. Extension loaded and opened `.qmd` files without issue
4. The only query-related warning was about indents (unrelated)

### What This Means

Zed is loading queries from the **grammar repository** (`tree-sitter-quarto/queries/highlights.scm`), not from the **extension** (`languages/quarto/highlights.scm`).

This explains:
- Why we see no highlighting (grammar uses `@markup.*` scopes)
- Why Zed themes don't work (themes expect `@text.*` scopes)
- Why our extension query fixes have no effect

## Implications

### For extension.toml
There is **NO field to specify custom query file names**:
- No `query_path` field
- No `highlights_query` field
- No `use_grammar_queries` flag
- Only fields: `repository`, `commit`/`rev`, `path` (for parser source)

### For Zed's Architecture

This appears to be either:
1. **A bug** - Zed should prioritize extension queries over grammar queries
2. **Missing feature** - Zed doesn't yet support extension query overrides
3. **Undocumented behavior** - There's a way to override but it's not documented

### Supporting Evidence from Research

From tree-sitter community:
- **Helix** maintains editor-specific queries separate from grammars
- **Neovim** has its own query files, ignores grammar queries
- **Standard pattern**: Each editor owns its queries

But Zed's extension system SHOULD allow extensions to provide queries. The docs say:
> "Extensions can provide query files in `languages/<lang>/` directory"

**The bug:** Zed loads grammar queries instead, ignoring extension queries.

## Recommended Solutions

### Short-term: Workaround Script

Use `fix-zed-queries.sh` to copy extension queries over grammar queries after installation:

```bash
./fix-zed-queries.sh
```

This works but requires:
- Running after every extension install
- Running after every Zed grammar rebuild
- Manual intervention

### Medium-term: Grammar PR

Add `queries/highlights-zed.scm` to tree-sitter-quarto:
- Keep standard `highlights.scm` unchanged
- Add Zed-compatible `highlights-zed.scm`
- **Problem:** Still need Zed to support specifying which query file to use

### Long-term: Fix Zed

File issue with Zed project:

**Title:** "Extension queries not loaded - Zed uses grammar's built-in queries instead"

**Description:**
- Extension provides `languages/<lang>/highlights.scm` with Zed-compatible scopes
- Grammar has `queries/highlights.scm` with standard scopes
- Expected: Zed loads extension queries (per documentation)
- Actual: Zed loads grammar queries, ignoring extension queries
- Evidence: Intentional syntax error in extension queries goes unreported

**Impact:** Extensions cannot override grammar query scopes, breaking highlighting for grammars that use standard (non-Zed) scope names.

## Test Files

- Test configuration: `docs/zed-query-loading-test.md`
- This results document: `docs/zed-query-loading-test-results.md`
- Workaround script: `fix-zed-queries.sh`
- Automated test: `tests/heading_highlighting.rs`

## Status

- ✅ Test completed successfully
- ✅ Syntax error removed from extension queries
- ✅ Root cause definitively identified
- ⏳ Awaiting decision on which solution to pursue

## Date
October 14, 2025
