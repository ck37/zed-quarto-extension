# Upstream Grammar Warnings to Fix

## Issue Summary

When compiling the tree-sitter-pandoc-markdown grammar (commit f9d6861), the C compiler emits warnings about unused functions and parameters in `scanner.c`.

## Warnings

```
grammars/pandoc_markdown/tree-sitter-pandoc-markdown/src/scanner.c:1179:47:
  warning: unused parameter 's' [-Wunused-parameter]

grammars/pandoc_markdown/tree-sitter-pandoc-markdown/src/scanner.c:1179:59:
  warning: unused parameter 'lexer' [-Wunused-parameter]

grammars/pandoc_markdown/tree-sitter-pandoc-markdown/src/scanner.c:1179:12:
  warning: unused function 'is_line_block_quick_check' [-Wunused-function]

grammars/pandoc_markdown/tree-sitter-pandoc-markdown/src/scanner.c:1192:13:
  warning: unused function 'parse_line_block' [-Wunused-function]
```

## Affected Code

**File**: `tree-sitter-pandoc-markdown/src/scanner.c`

**Lines 1179-1187**: `is_line_block_quick_check()` function
```c
static int is_line_block_quick_check(Scanner *s, TSLexer *lexer) {
    // This is a non-advancing check
    // We're already at '|', check what comes after without advancing

    // For now, just return -1 to indicate "unknown, need full parse"
    // The full disambiguation happens in the parse functions
    return -1;
}
```

**Lines 1192+**: `parse_line_block()` function
```c
static bool parse_line_block(Scanner *s, TSLexer *lexer,
                              const bool *valid_symbols) {
    (void)(valid_symbols);
    // ... implementation ...
}
```

## Root Cause

These functions appear to be stubbed out or disabled functionality for line block parsing. Based on the commit message "fix: disable external scanner to restore test suite" (f9d6861), line block support may have been intentionally disabled.

## Fix Options

### Option 1: Comment Out Unused Functions (Recommended)

If line blocks are intentionally disabled, wrap the functions in comments:

```c
/*
// Disabled pending line block implementation
static int is_line_block_quick_check(Scanner *s, TSLexer *lexer) {
    return -1;
}

static bool parse_line_block(Scanner *s, TSLexer *lexer,
                              const bool *valid_symbols) {
    // ... implementation ...
}
*/
```

### Option 2: Use Compiler Attributes

Mark functions as potentially unused:

```c
__attribute__((unused))
static int is_line_block_quick_check(Scanner *s, TSLexer *lexer) {
    return -1;
}

__attribute__((unused))
static bool parse_line_block(Scanner *s, TSLexer *lexer,
                              const bool *valid_symbols) {
    (void)(valid_symbols);
    // ... implementation ...
}
```

### Option 3: Add `(void)` Casts for Unused Parameters

If keeping the functions but they don't use parameters:

```c
static int is_line_block_quick_check(Scanner *s, TSLexer *lexer) {
    (void)s;      // Mark as intentionally unused
    (void)lexer;  // Mark as intentionally unused
    return -1;
}
```

### Option 4: Remove Functions Entirely

If line blocks won't be supported in the near term, delete the functions completely.

## Repository Information

- **Repository**: https://github.com/ck37/tree-sitter-pandoc-markdown
- **Branch**: feat/phase-1-pandoc-grammar
- **Commit**: f9d6861 (docs: update README with Phase 1 completion status)
- **File**: tree-sitter-pandoc-markdown/src/scanner.c
- **Lines**: 1179-1187, 1192+

## Impact

**Severity**: Low - These are compiler warnings, not errors. The grammar functions correctly.

**Build Impact**: Warnings appear during:
- Native test builds (`cargo test`)
- Release builds (`cargo build --release`)
- Do NOT appear in WASM builds (grammar pre-compiled)

## Current Workaround

The Zed extension suppresses these warnings in `build.rs`:

```rust
cc::Build::new()
    .include(&src_dir)
    .file(src_dir.join("parser.c"))
    .file(src_dir.join("scanner.c"))
    .flag_if_supported("-Wno-unused-parameter")
    .flag_if_supported("-Wno-unused-function")
    .compile("tree-sitter-pandoc-markdown");
```

This works but is a downstream workaround. Fixing upstream would benefit all users of the grammar.

## Recommendation

**For tree-sitter-pandoc-markdown repository:**

1. Review if line block support is planned for Phase 2
2. If yes (keeping functions): Add `__attribute__((unused))` or `(void)` casts
3. If no (removing line blocks): Comment out or delete the functions
4. If uncertain: Add `#ifdef ENABLE_LINE_BLOCKS` wrapper for future activation

**Example commit message:**
```
fix: suppress unused function warnings in scanner.c

Comment out is_line_block_quick_check() and parse_line_block()
functions that were disabled in f9d6861. These will be re-enabled
when line block support is implemented in Phase 2.

Fixes compiler warnings:
- [-Wunused-function] for both functions
- [-Wunused-parameter] for s and lexer parameters
```

## Testing

After fixing, verify warnings are gone:

```bash
cd tree-sitter-pandoc-markdown
npm install
npm run build

# Or with the Rust bindings
cd bindings/rust
cargo build
```

Should compile without warnings.
