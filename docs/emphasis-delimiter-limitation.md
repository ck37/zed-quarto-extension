# Emphasis Delimiter Styling Limitation

**Issue**: Bold/italic markers (`*`, `**`) don't appear visually different from content
**Cause**: Zed themes don't have distinct styling for `@punctuation.delimiter`
**Status**: Grammar is correct, theme support needed

## What We Implemented

The grammar correctly captures emphasis delimiters separately:

```scheme
; Delimiters captured separately
(emphasis_delimiter) @punctuation.delimiter        # The * or _
(strong_emphasis_delimiter) @punctuation.delimiter # The ** or __

; Content captured separately
(emphasis) @text.emphasis                          # The italic text
(strong_emphasis) @emphasis.strong                 # The bold text
```

## The Problem

When you write `*italic text*` or `**bold text**`, the grammar produces:

```
(emphasis
  (emphasis_delimiter)    # <-- @punctuation.delimiter
  (text)                  # <-- @text.emphasis
  (emphasis_delimiter))   # <-- @punctuation.delimiter
```

The delimiters ARE being captured with `@punctuation.delimiter`, but **Zed themes don't style that scope differently** from regular text.

## Why Themes Don't Support It Yet

Most Zed themes were created before this feature existed. They have styling for:
- `@text.emphasis` (italic content)
- `@emphasis.strong` (bold content)
- `@punctuation.bracket` (brackets like `[`, `]`)
- `@punctuation.special` (special chars like `#`, `@`)

But they don't have distinct styling for:
- `@punctuation.delimiter` (emphasis markers, code span backticks, etc.)

So the delimiters just inherit the default text color or a generic punctuation color that looks the same.

## Verification

You can verify the grammar is working by checking other delimiters:

**Code spans**: `` `code` ``
- The backticks are captured as `(code_span_delimiter) @punctuation.delimiter`
- They probably also look the same as the content

**Code fences**: ` ``` `
- The fence markers are captured as `(code_fence_delimiter) @punctuation.delimiter`
- They might look slightly different depending on theme

If backticks and fence markers also don't look different from content, that confirms themes don't style `@punctuation.delimiter`.

## Workarounds

### Option 1: Custom Theme (Advanced)

Create or modify a Zed theme to style `@punctuation.delimiter`:

```json
{
  "syntax": {
    "punctuation.delimiter": {
      "color": "#666666",
      "opacity": 0.5
    },
    "text.emphasis": {
      "color": "#ff6b6b",
      "font_style": "italic"
    },
    "emphasis.strong": {
      "color": "#4ecdc4",
      "font_weight": "bold"
    }
  }
}
```

This would make delimiters gray and semi-transparent while keeping content colorful.

### Option 2: Use Different Scope (Experimental)

We could try using `@comment` for delimiters to make them dimmer:

```scheme
(emphasis_delimiter) @comment
(strong_emphasis_delimiter) @comment
```

This might make them appear grayed out in most themes, but could have unintended side effects.

### Option 3: Wait for Theme Updates

As more Zed themes adopt modern tree-sitter conventions, they may add styling for `@punctuation.delimiter`.

## Comparison with Other Editors

### VSCode
Has explicit theme support for dimming markdown syntax:
- `punctuation.definition.bold.markdown`
- `punctuation.definition.italic.markdown`

### Obsidian
Automatically dims markdown syntax markers.

### Neovim
Many themes style `@markup.italic` (content) differently from `@punctuation.delimiter`.

### Zed
**Currently lacks theme support** for delimiter styling in most themes.

## Future

This feature is **ready to use** as soon as Zed themes add styling for `@punctuation.delimiter`. The grammar infrastructure is in place:

1. ✅ Grammar captures delimiters
2. ✅ Extension uses correct scope names
3. ⏳ Themes need to add styling rules

## Testing

To test if a theme supports delimiter styling:

1. Open a `.qmd` file with emphasis: `*italic*` and `**bold**`
2. Check if the `*` markers look different from "italic" and "bold"
3. Try different themes to see if any style delimiters

If all themes show markers the same as content, that confirms the limitation.

## Recommendation

**For now**: Accept that markers look the same as content. This is how standard Markdown editors work too (until recently).

**For future**: When creating or updating Zed themes, add `@punctuation.delimiter` styling to enable this modern UX pattern.

## Related

- Issue: tree-sitter-quarto@28111dc added delimiter captures
- `docs/emphasis-delimiter-support.md`: Feature documentation
- `languages/quarto/highlights.scm:92-93`: Delimiter capture rules
