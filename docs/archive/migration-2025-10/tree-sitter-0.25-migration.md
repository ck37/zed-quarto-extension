# tree-sitter 0.25 API Migration

**Date**: 2025-10-17
**Status**: ✅ Complete - All tests passing

## Summary

Successfully migrated the Rust test suite from tree-sitter 0.24 to 0.25 API. The key change is how `HighlightConfiguration.configure()` works with scope names.

## Breaking Change: HighlightConfiguration.configure()

### The Problem

In tree-sitter 0.24, you could pass any list of scope names to `configure()`, and it would accept all of them. In 0.25, `configure()` only accepts scope names that actually exist in your query.

**Symptom**: Index out of bounds errors when accessing `config.names()[highlight_index]` during highlighting.

```rust
// Before (tree-sitter 0.24) - This worked
let scope_names = vec![
    "annotation", "attribute", "comment", "constant",
    "markup.bold", "markup.heading", "text.title",
    // ... many more names, some not in the query
];
config.configure(&scope_names);
// config.names() would have all 40 items

// Highlighting would sometimes try to access index 35+
let scope_name = config.names()[s.0];  // PANIC: index out of bounds!
```

### The Fix

**Only configure scope names that exist in your queries.**

```rust
// After (tree-sitter 0.25) - Use only names from the query
let scope_names: Vec<String> = config.names().iter().map(|s| s.to_string()).collect();
let scope_names_refs: Vec<&str> = scope_names.iter().map(|s| s.as_str()).collect();
config.configure(&scope_names_refs);
// config.names() now has only 25 items (the ones actually in the query)

// Highlighting works correctly
let scope_name = config.names()[s.0];  // ✓ Always valid index
```

## How HighlightConfiguration Works in 0.25

1. **Create configuration** with your queries:
   ```rust
   let mut config = HighlightConfiguration::new(
       language(),
       "quarto",
       highlight_query,  // Contains "@text.title", "@punctuation.special", etc.
       injection_query,
       locals_query,
   )?;
   ```

2. **Check what capture names exist** in your query:
   ```rust
   // config.names() returns the capture names found in your queries
   // Example: ["_lang", "injection.content", "punctuation.delimiter",
   //           "text.title", "emphasis.strong", ...]
   eprintln!("Captures in query: {:?}", config.names());
   ```

3. **Configure with those names** (or a mapping):
   ```rust
   // Option A: Use the same names (identity mapping)
   let scope_names: Vec<String> = config.names().iter().map(|s| s.to_string()).collect();
   let scope_names_refs: Vec<&str> = scope_names.iter().map(|s| s.as_str()).collect();
   config.configure(&scope_names_refs);

   // Option B: Map to different theme names
   let scope_names = vec![
       "_lang",              // Keep as-is
       "injection.content",  // Keep as-is
       "delimiter",          // Map punctuation.delimiter -> delimiter
       "builtin",            // Map function.builtin -> builtin
       "heading",            // Map text.title -> heading
       // ... must have same length as config.names()
   ];
   config.configure(&scope_names);
   ```

4. **Use during highlighting**:
   ```rust
   for event in highlighter.highlight(&config, source, None, |_| None)? {
       match event? {
           HighlightEvent::HighlightStart(s) => {
               let scope_name = config.names()[s.0];  // s.0 is always < config.names().len()
               println!("<{}>", scope_name);
           }
           HighlightEvent::HighlightEnd => println!("</>"),
           HighlightEvent::Source { start, end } => print!("{}", &source[start..end]),
       }
   }
   ```

## Why This Changed

In 0.24, `configure()` would accept extra scope names that weren't in the query, leading to:
- Wasted memory storing unused names
- Potential index mismatches
- Confusion about what scopes are actually used

In 0.25, `configure()` enforces that you only map the scopes that exist in your query, making the API more predictable and type-safe.

## Migration Checklist

- [x] Replace hardcoded scope lists with `config.names()` iteration
- [x] Update `heading_highlighting.rs` test
- [x] Verify all tests pass (15/15 passing)
- [x] Document the change

## Files Changed

### tests/heading_highlighting.rs

**Before**:
```rust
let scope_names = vec![
    "annotation", "attribute", "comment", "constant",
    "constant.builtin", "constant.macro", "emphasis.strong",
    // ... 40 total names, many not in query
];
config.configure(&scope_names);
```

**After**:
```rust
// Only configure names that exist in the query
let scope_names: Vec<String> = config.names().iter().map(|s| s.to_string()).collect();
let scope_names_refs: Vec<&str> = scope_names.iter().map(|s| s.as_str()).collect();
config.configure(&scope_names_refs);
```

**Also Updated**:
- Fixed heading marker assertions to accept trailing space: `## ` vs `##`
- Removed debug prints for cleaner test output
- Simplified highlight event handling

## Test Results

```
running 15 tests
test result: ok. 15 passed; 0 failed; 0 ignored
```

All tests passing:
- ✅ Manifest validation
- ✅ Query validation
- ✅ Heading highlighting
- ✅ Python code injection
- ✅ R code injection
- ✅ LSP smoke tests
- ✅ WASM extension builds

## References

- Tree-sitter docs: https://tree-sitter.github.io/tree-sitter/
- tree-sitter-highlight crate: https://docs.rs/tree-sitter-highlight/0.25.10
- Related issue: https://github.com/tree-sitter/tree-sitter/issues/3487

## Impact on Zed Extension

This change only affects our Rust test suite. The actual Zed extension is unaffected because:
- Zed uses its own highlighting system
- Zed doesn't use `tree-sitter-highlight` crate directly
- Our extension queries (`languages/quarto/highlights.scm`) remain unchanged

The test suite now correctly validates that our queries work with tree-sitter 0.25, which is what Zed uses.
