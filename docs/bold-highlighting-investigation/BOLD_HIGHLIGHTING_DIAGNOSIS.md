# Bold Highlighting Diagnosis

## Problem
Bold text (`**text**`) is not highlighting correctly in Zed editor for `.qmd` files.

## Root Cause
The pandoc-markdown grammar has a **two-grammar architecture**:
1. **Block grammar** (`tree-sitter-pandoc-markdown`) - parses document structure
2. **Inline grammar** (`tree-sitter-pandoc-markdown-inline`) - parses inline content (bold, italic, links, etc.)

The block grammar creates `(inline)` nodes, which must be injected with the inline grammar to parse emphasis/bold/italic.

### What We Found

1. **Missing Inline Injection** (FIXED)
   - The extension's `injections.scm` was missing: `((inline) @injection.content (#set! injection.language "pandoc_markdown_inline"))`
   - Added this critical line to inject inline grammar into `(inline)` nodes

2. **Grammar Compilation** (FIXED)
   - The inline grammar wasn't being compiled by `build.rs`
   - Updated `build.rs` to compile both grammars together into one library

3. **Scope Index Mismatch** (CURRENT ISSUE)
   - Tree-sitter-highlight requires injected grammars to share a unified scope index space
   - Block grammar: 22 scopes (indices 0-21)
   - Inline grammar: 16 scopes (indices 0-15)
   - Highlighter emitting scope index 25 → out of bounds for both!

   **Why This Happens:**
   - Tree-sitter-highlight auto-adds `injection.content` and `injection.language` scopes
   - Different scope orderings in block vs inline queries
   - No coordination between the two grammar configurations

## Current Test Status

```bash
cargo test highlights_cover_quarto_constructs
```

**Error:**
```
ERROR: Scope index 25 out of bounds (max 22)
Block config has 22 scopes
Inline config has 16 scopes
```

**Block Grammar Scopes (22):**
```
[0] injection.content    [11] text.strike
[1] injection.language   [12] text.highlight
[2] text.title           [13] text.subscript
[3] punctuation.special  [14] text.super
[4] text.literal         [15] text.underline
[5] punctuation.delimiter [16] text.uri
[6] comment              [17] tag
[7] string               [18] constant.macro
[8] text.reference       [19] type
[9] text.emphasis        [20] text
[10] text.strong         [21] property
```

**Inline Grammar Scopes (16):**
```
[0] text.emphasis        [8] text.highlight
[1] text.strong          [9] text.subscript
[2] text.literal         [10] text.super
[3] property             [11] text.underline
[4] text.reference       [12] text
[5] text.uri             [13] string
[6] comment              [14] punctuation.special
[7] text.strike          [15] tag
```

## Options to Fix

### Option 1: Use Only Block Grammar (Simple but Limited)
Remove inline grammar injection entirely. The block grammar can parse `strong_emphasis` nodes, but won't handle complex nested cases.

**Pros:**
- Simple, no injection complexity
- Works for basic bold/italic

**Cons:**
- May not handle all Pandoc inline features
- Block grammar might not have full inline coverage

### Option 2: Fix Scope Coordination (Proper Fix)
Ensure both grammars use identical scope name lists in the same order.

**Approach:**
1. Create a shared scope names constant
2. Configure both grammars with the same list
3. The inline grammar will only USE the scopes it needs, but they'll map to the same indices

**Implementation:**
```rust
const SHARED_SCOPES: &[&str] = &[
    "injection.content",
    "injection.language",
    "text.emphasis",
    "text.strong",
    // ... complete unified list
];

block_config.configure(SHARED_SCOPES);
inline_config.configure(SHARED_SCOPES);
```

### Option 3: Check Zed's Handling (May Work Already)
Zed might handle multi-grammar injection differently than our test harness. The extension configuration in `extension.toml` declares both grammars and the injection query.

**Test:** Install the current extension in Zed and check if bold highlighting works there, even though tests fail.

## Recommendation

Try **Option 3 first** - install in Zed and test. If that doesn't work, implement **Option 2** for proper multi-grammar support.

The fact that VS Code's Quarto extension works suggests this is a solvable problem with proper configuration.

## Next Steps

1. Use `./install-dev.sh` to prepare extension
2. Install in Zed: Cmd+Shift+P → "zed: install dev extension"
3. Open a `.qmd` file and test `**bold**` highlighting
4. If it doesn't work in Zed either, implement Option 2 (scope coordination fix)
