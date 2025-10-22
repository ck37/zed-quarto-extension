# Table Cell Parsing - Resolved

**Status as of 2025-10-22**

## Summary

Issue [#11](https://github.com/ck37/tree-sitter-quarto/issues/11) in tree-sitter-quarto has been **RESOLVED**. Table data rows now expose individual cells as AST nodes, enabling proper syntax highlighting of cell contents.

## Current Status

- **Issue Status:** CLOSED ✅
- **Current Extension Commit:** `acaaaff1fe10780f496fb122e0de32bd8d7191d6`
- **Fix Status:** Implemented and merged

## The Problem (RESOLVED)

In the old grammar (before acaaaff):

```javascript
// Old implementation in grammar.js
pipe_table_row: ($) =>
  prec.dynamic(1, token(seq("|", repeat1(seq(/[^|\r\n]+/, "|")), /\r?\n/))),
```

The `token()` wrapper treated the entire row as an opaque unit, preventing individual cells from being exposed as AST nodes.

**Previous Impact:**
- Header rows exposed `table_cell` nodes ✅
- Data rows did NOT expose cell nodes ❌
- Could not highlight bold/italic/code within table cells
- Could not distinguish column types
- Cells could appear "cut off" in editors

## The Solution (IMPLEMENTED)

The fix restructured `pipe_table_row` to match the header structure:

```javascript
// New implementation in grammar.js (as of acaaaff)
pipe_table_row: ($) =>
  seq(
    token(prec(100, "|")),  // Atomic pipe with high precedence
    repeat1(
      seq(field("content", alias(/[^|\r\n]+/, $.table_cell)), token("|"))
    ),
    token(prec(100, /\r?\n/))  // Atomic newline
  ),
```

This approach:
- ✅ Exposes individual `table_cell` nodes
- ✅ Maintains row-level structure recognition
- ✅ Uses atomic tokens for delimiters to prevent ambiguity
- ✅ All tests pass

## Migration Completed

This was a **breaking change** for tree-sitter queries that reference table structures. The migration involved:

1. ✅ Updated `extension.toml` to commit `acaaaff1fe10780f496fb122e0de32bd8d7191d6`
2. ✅ Updated `build.rs` to match the same commit
3. ✅ Updated `languages/quarto/highlights.scm` to highlight pipe delimiters in rows
4. ✅ Fixed test `tests/table_currency_range.rs` to use `table_cell` instead of `pipe_table_cell`
5. ✅ All tests pass

### Changes Made

**highlights.scm:**
```diff
-; Note: pipe_table_row is a token with no internal structure,
-; so we can't highlight individual delimiters within rows
-(pipe_table_row) @none
+; Pipe table rows - individual cells are exposed as table_cell nodes
+; Delimiters are atomic tokens, cells contain raw text (no inline parsing yet)
+(pipe_table_row
+  "|" @punctuation.delimiter)
```

**Test update:**
Changed `child.kind() == "pipe_table_cell"` to `child.kind() == "table_cell"` to match the actual node type.

## Benefits

Now that cells are exposed:
- ✅ Can query individual cells in the AST
- ✅ Pipe delimiters in data rows are highlighted
- ✅ Consistent structure between header and data rows
- ✅ Foundation for future inline content parsing within cells
- ✅ Better editor experience with proper cell boundaries

## References

- Issue: https://github.com/ck37/tree-sitter-quarto/issues/11
- Created: 2025-10-22 13:27:30 UTC
- Comment about fix: 2025-10-22 13:49:54 UTC
- Test file demonstrating issue: `tests/table_currency_range.rs` (local extension)
