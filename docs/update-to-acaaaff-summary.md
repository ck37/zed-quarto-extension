# Update to tree-sitter-quarto acaaaff

**Date:** 2025-10-22

## Summary

Successfully updated the Zed Quarto extension to use tree-sitter-quarto commit `acaaaff1fe10780f496fb122e0de32bd8d7191d6`, which implements [issue #11](https://github.com/ck37/tree-sitter-quarto/issues/11) - exposing individual cells in pipe table rows.

## Changes Made

### 1. Grammar Version Update

**extension.toml:**
```diff
-rev = "91d8de3f3eef8131b7b8d97e172d14a2466272b7"
+rev = "acaaaff1fe10780f496fb122e0de32bd8d7191d6"
```

**build.rs:**
```diff
-const QUARTO_COMMIT: &str = "91d8de3f3eef8131b7b8d97e172d14a2466272b7";
+const QUARTO_COMMIT: &str = "acaaaff1fe10780f496fb122e0de32bd8d7191d6";
```

### 2. Highlights Query Update

**languages/quarto/highlights.scm:**
```diff
-; Note: pipe_table_row is a token with no internal structure,
-; so we can't highlight individual delimiters within rows
-(pipe_table_row) @none
+; Pipe table rows - individual cells are exposed as table_cell nodes
+; Delimiters are atomic tokens, cells contain raw text (no inline parsing yet)
+(pipe_table_row
+  "|" @punctuation.delimiter)
```

### 3. Test Fix

**tests/table_currency_range.rs:**
```diff
-if child.kind() == "pipe_table_cell" {
+if child.kind() == "table_cell" {
```

## What Changed in the Grammar

### Before (91d8de3)
```javascript
pipe_table_row: ($) =>
  prec.dynamic(1, token(seq("|", repeat1(seq(/[^|\r\n]+/, "|")), /\r?\n/))),
```

The entire row was an opaque token with no internal structure.

### After (acaaaff)
```javascript
pipe_table_row: ($) =>
  seq(
    token(prec(100, "|")),
    repeat1(
      seq(field("content", alias(/[^|\r\n]+/, $.table_cell)), token("|"))
    ),
    token(prec(100, /\r?\n/))
  ),
```

Now individual cells are exposed as `table_cell` nodes with the `content` field.

## AST Structure Change

### Before
```
(pipe_table_row)  # Single opaque token
```

### After
```
(pipe_table_row
  content: (table_cell)
  content: (table_cell)
  content: (table_cell))
```

## Benefits

1. **Cell-level highlighting:** Can now apply syntax highlighting to individual cells
2. **Consistent structure:** Data rows now match header row structure
3. **Query capabilities:** Can write tree-sitter queries targeting specific cells
4. **Foundation for future work:** Enables future inline content parsing within cells (bold, italic, code spans, etc.)
5. **Better editor experience:** Proper cell boundaries improve navigation and selection

## Test Results

All 73 tests pass:
- ✅ Fixed `tests/table_currency_range.rs` to use correct node type
- ✅ Grammar compiles successfully
- ✅ Highlighting queries are valid
- ✅ All existing functionality preserved

## Breaking Changes

This is a **breaking change** for custom queries that reference `pipe_table_row`. However:
- The extension's queries have been updated
- Tests have been updated
- No user-visible breaking changes (only improvements)

## Documentation

Updated:
- `docs/table-parsing-limitation.md` - Changed from "limitation" to "resolved"
- `docs/update-to-acaaaff-summary.md` - This document

## Next Steps

1. Build and install the extension in Zed for testing:
   ```bash
   cargo build --release --target wasm32-wasip2
   cp target/wasm32-wasip2/release/quarto_zed.wasm extension.wasm
   # Then in Zed: Cmd+Shift+P -> "zed: install dev extension"
   ```

2. Test table rendering in Zed with various table structures

3. Consider future enhancements:
   - Inline content parsing within cells (requires grammar work)
   - Additional table cell highlighting patterns
   - Column-specific styling

## References

- Issue: https://github.com/ck37/tree-sitter-quarto/issues/11
- Grammar commit: https://github.com/ck37/tree-sitter-quarto/commit/acaaaff1fe10780f496fb122e0de32bd8d7191d6
- CHANGELOG: https://github.com/ck37/tree-sitter-quarto/blob/main/CHANGELOG.md
