# The Catch-All Query Override Issue

**Date:** 2025-10-17
**Issue:** Inline formatting content (text inside `*italic*`, `**bold**`, `# headings`) was not being highlighted
**Root Cause:** Catch-all `(text) @text` query overriding parent scopes
**Solution:** Remove catch-all query entirely

## Problem Description

When highlighting inline formatted text, the content inside emphasis/strong/headings appeared unstyled, even though:
- The grammar parsed the structure correctly
- Queries captured both parent and child nodes
- Tests showed captures were working

## Root Cause Analysis

### Tree-sitter Scope Precedence

Tree-sitter applies **the innermost/last scope** to each node. When multiple captures match the same text range, the last one wins.

### What Was Happening

With the catch-all query `(text) @text` present:

```scheme
(emphasis) @text.emphasis          # Captures parent emphasis node
(emphasis (text) @text.emphasis)   # Captures child text node with emphasis scope
(text) @text                        # CATCH-ALL captures ALL text nodes with generic scope
```

When highlighting `*italic text*`:
1. Parent `(emphasis)` node → captured as `@text.emphasis` ✓
2. Child `(text "italic text")` → captured FIRST as `@text.emphasis` ✓
3. Child `(text "italic text")` → captured AGAIN as `@text` ✗ **OVERRIDES PARENT!**

### The Evidence

Created `tests/highlight_capture_analysis.rs` which revealed:

```
▼ HighlightStart: @text.emphasis     ← Parent opens
  ▼ HighlightStart: @text.emphasis   ← Delimiter
    Source[0..1]: "*"
  ▲ HighlightEnd
  ▼ HighlightStart: @text             ← WRONG! Should be @text.emphasis
    Source[1..12]: "italic text"
  ▲ HighlightEnd
▲ HighlightEnd
```

The analysis showed: **'italic text' is NOT inside any emphasis/strong highlight scope!**

## The Solution

### Remove the Catch-All

```scheme
; Text
; ----
; NOTE: Catch-all (text) @text removed because it overrides parent scopes
; Child text nodes inherit styling from their parent (emphasis, strong, heading, etc.)
```

By removing `(text) @text`, child text nodes only get captured by their explicit parent queries:
- `(emphasis (text) @text.emphasis)`
- `(strong_emphasis (text) @emphasis.strong)`
- `(atx_heading content: (inline (text) @text.title))`

### Result After Fix

```
▼ HighlightStart: @text.emphasis     ← Parent opens
  ▼ HighlightStart: @text.emphasis   ← Delimiter
    Source[0..1]: "*"
  ▲ HighlightEnd
  ▼ HighlightStart: @text.emphasis   ← ✓ CORRECT! Content gets parent scope
    Source[1..12]: "italic text"
  ▲ HighlightEnd
▲ HighlightEnd
```

Analysis confirms: **✓ Found 'italic text' inside a highlight scope!**

## Why This Is Subtle

### Headings Appeared to Work

Headings seemed to work initially because the `(inline)` parent was captured, which created a scope that applied to its range. But internally, the text nodes still had `@text` scope, not `@text.title`.

The fix makes it consistent: text nodes get their parent's scope explicitly.

### The Explicit Child Captures Are Still Needed

We need BOTH:
1. Remove catch-all to prevent override
2. Keep explicit child captures to ensure parent scope applies

```scheme
(emphasis) @text.emphasis               # Parent node
(emphasis (text) @text.emphasis)        # Explicit child capture
(emphasis (_) @text.emphasis)           # Any other children (code spans, etc.)
```

## Lessons Learned

### Query Order Matters

In tree-sitter queries, later captures can override earlier ones for the same text range.

### Test with tree-sitter-highlight

The `HighlightEvent` API shows exactly what scopes are applied:
- `HighlightStart` - when a scope begins
- `Source` - the actual text being highlighted
- `HighlightEnd` - when a scope ends

This reveals issues that aren't obvious from just looking at queries.

### Catch-Alls Are Dangerous

Catch-all queries like `(text) @text` seem harmless but can override more specific parent captures. Use sparingly and test thoroughly.

## Related Files

- **Fix**: `languages/quarto/highlights.scm:283` - Catch-all removed
- **Analysis Test**: `tests/highlight_capture_analysis.rs` - Deep capture analysis
- **Investigation**: `docs/inline-highlighting-investigation.md` - Full investigation history
- **Validation Test**: `tests/query_node_validation.rs` - Prevents invalid queries

## Testing

To verify the fix works:

```rust
cargo test analyze_emphasis_captures -- --nocapture
cargo test analyze_heading_captures -- --nocapture
```

Should show:
- `✓ Found 'italic text' inside a highlight scope!`
- `✓ Found 'Heading Text' inside @text.title scope!`
