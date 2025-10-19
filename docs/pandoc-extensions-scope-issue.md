# Pandoc Extensions Scope Issue

## Problem

Strikethrough, highlight, subscript, and superscript text don't highlight in Zed, even though:
- Grammar parses them correctly
- Queries capture them correctly
- Tests pass

## Root Cause

**Zed's theme system doesn't support these scopes:**

- `@text.strike` (strikethrough: `~~text~~`)
- `@text.highlight` (highlight/mark: `==text==`)
- `@text.subscript` (subscript: `~text~`)
- `@text.super` (superscript: `^text^`)

### Evidence

From `docs/zed-syntax-scopes.md`, the complete list of Zed core scopes includes:
- `text.literal` ✅ (for code spans)
- `text` ✅ (plain text)

But **NOT**:
- `text.strike` ❌
- `text.highlight` ❌
- `text.subscript` ❌
- `text.super` ❌

### Why Tests Pass

The scope validation test (`tests/zed_scope_validation.rs`) allows these scopes because of this regex pattern:

```rust
r"^(text|emphasis|function|variable|constant|string|punctuation)\..*$"
```

This allows **any** `text.*` sub-scope, even though Zed's theme doesn't actually support them all.

## Current Behavior

In highlights.scm (lines 105-111):

```scm
(strikethrough) @text.strike
(highlight) @text.highlight
(subscript) @text.subscript
(superscript) @text.super
```

These patterns:
1. ✅ Parse correctly (grammar supports the node types)
2. ✅ Capture correctly (queries compile and execute)
3. ❌ Don't highlight in Zed (theme doesn't support the scopes)

The scopes likely **fall back to `text`**, which means they render as plain text with no special styling.

## Possible Solutions

### Option 1: Accept No Highlighting (Current)

**Pros:**
- Queries are correct and match grammar
- No errors or warnings
- Works if/when Zed adds support for these scopes

**Cons:**
- No visual distinction for these Pandoc features
- Users won't see strikethrough, highlight, subscript, superscript styling

### Option 2: Map to Closest Supported Scopes

Map to scopes that Zed does support:

```scm
(strikethrough) @comment       ; or @string.special
(highlight) @string.special    ; or @emphasis
(subscript) @variable          ; or @string
(superscript) @variable        ; or @string
```

**Pros:**
- Some visual distinction
- Better than plain text

**Cons:**
- Semantically incorrect (strikethrough isn't really a comment)
- May look wrong with themes (e.g., strikethrough colored like comments)
- Confusing for users who expect correct highlighting

### Option 3: Highlight Content Only (Like Bold/Italic Fix)

Similar to how we fixed bold/italic, highlight just the content nodes:

```scm
; Delimiters
(strikethrough_delimiter) @punctuation.delimiter
(highlight_delimiter) @punctuation.delimiter
(subscript_delimiter) @punctuation.delimiter
(superscript_delimiter) @punctuation.delimiter

; Content (if it has text child nodes)
(strikethrough
  (text) @emphasis)  ; or @string.special

(highlight
  (text) @emphasis.strong)  ; or @string.special
```

**Pros:**
- Consistent with bold/italic approach
- Avoids whole-node highlighting
- Delimiters get styled

**Cons:**
- Still uses unsupported scopes for content
- May not have the semantic meaning we want

### Option 4: Document Limitation

Keep current queries but add documentation that these features won't highlight in Zed.

**Pros:**
- Queries are "correct" for the grammar
- Future-proof if Zed adds support
- Clear expectations for users

**Cons:**
- Features don't work in Zed

## Investigation Needed

Test in Zed to confirm:
1. Do these scopes render as plain text?
2. Do they fall back to `text` scope?
3. Are there any console warnings?
4. How does Zed's built-in markdown handle these (if at all)?

## Recommendation

**Option 1** (current approach) with **Option 4** (documentation):

1. Keep the current queries (they're technically correct)
2. Add a note in README.md about unsupported Pandoc features
3. File a feature request with Zed to add support for these scopes
4. Consider Option 2 or 3 only if user feedback indicates it's needed

## Testing

To test in Zed:

```markdown
This is ~~strikethrough~~ text.

This is ==highlighted== text.

Chemical formula: H~2~O

Math: x^2^ + y^2^ = z^2^
```

Expected: All render as plain text
Desired: Visual styling for each type

## Related Issues

- Bold/italic highlighting was fixed by removing catch-all `(text) @text` pattern
- These Pandoc extensions have the same pattern (whole-node highlighting)
- But the scope names are the real blocker, not the query patterns

## Files

- `languages/quarto/highlights.scm` lines 105-111
- `tests/pandoc_extensions.rs` - Tests that verify parsing works
- `docs/zed-syntax-scopes.md` - Complete list of Zed-supported scopes
