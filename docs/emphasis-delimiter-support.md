# Emphasis Delimiter Support

**Feature**: Separate highlighting for emphasis markers (`*`, `**`, `_`, `__`)
**Added in**: tree-sitter-quarto@28111dc
**Status**: ✅ Fully supported (no action needed)

## What This Enables

Themes can now style emphasis markers differently from their content. For example:

```markdown
*italic text*     ← `*` markers can be dimmed/grayed
**bold text**     ← `**` markers can be dimmed/grayed
```

This is a common UX pattern in modern markdown editors (VSCode, Obsidian, Typora) where the syntax markers are de-emphasized to focus on the content.

## How It Works

### New Delimiter Captures

The grammar now captures emphasis delimiters separately:

```scheme
; Delimiters (new in 28111dc)
(emphasis_delimiter) @punctuation.delimiter
(strong_emphasis_delimiter) @punctuation.delimiter

; Content (existing)
(emphasis) @text.emphasis
(strong_emphasis) @emphasis.strong
```

### Scope Mapping

| Element | Node Type | Scope | What Themes Can Style |
|---------|-----------|-------|----------------------|
| `*text*` delimiters | `emphasis_delimiter` | `@punctuation.delimiter` | The `*` markers |
| `*text*` content | `emphasis` | `@text.emphasis` | The word "text" |
| `**text**` delimiters | `strong_emphasis_delimiter` | `@punctuation.delimiter` | The `**` markers |
| `**text**` content | `strong_emphasis` | `@emphasis.strong` | The word "text" |

## Current State

### ✅ Upstream Grammar (28111dc)
The tree-sitter-quarto grammar has these captures in `queries/highlights.scm`.

### ✅ Extension Highlights
Our `languages/quarto/highlights.scm` has these captures (lines 92-93).

### ✅ Build System
The `build.rs` copies our highlights.scm to the test grammar, so tests use the delimiter captures.

### ⚠️ Zed Theme Support
**To be tested**: Whether Zed themes actually use `@punctuation.delimiter` to style emphasis markers.

Most Zed themes may not have specific styling for this yet, but the infrastructure is in place for theme authors to use it.

## Testing

To verify this feature works in Zed:

1. **Install the extension** with the latest grammar (28111dc)
2. **Create a test file** with emphasis:
   ```markdown
   *This is italic text*
   **This is bold text**
   ```
3. **Inspect with Zed's syntax inspector** (if available) to see if delimiters are captured
4. **Try different themes** to see if any style delimiters differently

## Theme Authors

If you're creating or modifying a Zed theme, you can now style emphasis markers:

```json
{
  "punctuation.delimiter": {
    "color": "#888888",  // Dim gray for markers
    "opacity": 0.6       // Or make them semi-transparent
  },
  "text.emphasis": {
    "color": "#ff6b6b",  // Bright color for italic content
    "style": "italic"
  },
  "emphasis.strong": {
    "color": "#4ecdc4",  // Bright color for bold content
    "weight": "bold"
  }
}
```

## Backward Compatibility

This is a **purely additive change** with no breaking changes:
- Old themes that don't style `@punctuation.delimiter` will still work
- The content captures (`@text.emphasis`, `@emphasis.strong`) remain unchanged
- Themes can choose to style delimiters or ignore them

## Related

- **Grammar commit**: https://github.com/ck37/tree-sitter-quarto/commit/28111dc
- **Extension version**: 0.2.0+ with grammar@28111dc
- **Similar features**: Code span delimiters also captured as `@punctuation.delimiter`

## No Action Required

✅ **The extension already supports this feature.** Theme authors can take advantage of it when they update their themes. No changes needed to the extension code.
