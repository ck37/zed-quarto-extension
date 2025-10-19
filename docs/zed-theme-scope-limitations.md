# Zed Theme Scope Limitations for Markdown-Style Formatting

## Issue Summary

While our Quarto extension correctly captures bold, italic, and delimiter syntax using tree-sitter queries, **Zed themes may not provide distinct visual styling for these scopes**.

## What We've Confirmed

### ✅ Extension is Working Correctly

1. **Queries capture correctly**: Our tests (`tests/bold_italic_highlighting_test.rs`) confirm that:
   - `@emphasis.strong` captures bold text content
   - `@text.emphasis` captures italic text content
   - `@punctuation.delimiter` captures `**`, `*`, `---` markers
   - `@text.reference` and `@text.uri` capture link text and URLs

2. **Scopes are Zed-compatible**: The `zed_scope_validation` test confirms all our scopes are in Zed's official list

3. **No loading errors**: Zed loads the extension without warnings or errors

### ❌ Theme Styling May Be Subtle or Missing

**Observed behavior**:
- YAML front matter content is highlighted (language injection works)
- YAML markers (`---`) are NOT visibly highlighted
- Bold/italic markers (`**`, `*`) are NOT visibly highlighted
- Bold/italic content MAY not be visibly different from plain text

**Root cause**: Zed themes don't always provide distinct colors for these scopes:
- `@punctuation.delimiter` - often styled the same as plain text
- `@emphasis.strong` - may not be styled differently than `@text`
- `@text.emphasis` - may not be styled differently than `@text`

## Zed's Built-in Markdown Behavior

Research into Zed's own markdown extension reveals:

1. Zed's markdown uses scopes like:
   - `@title.markup` (not `@text.title`)
   - `@punctuation.list_marker.markup` (not `@punctuation.special`)
   - `@link_text.markup` / `@link_uri.markup`

2. Zed's markdown extension **uses `.markup` suffixes** on many scopes

3. Zed's built-in markdown highlights.scm (in their source code) focuses on structural elements (headings, lists, tables, code blocks) rather than inline formatting

## Why This Happens

### Tree-Sitter vs Theme Styling

1. **Tree-sitter queries** define WHAT to capture (structural)
2. **Theme files** define HOW to display (visual)

There's a disconnect: even if queries correctly capture `@emphasis.strong`, the theme must explicitly style that scope for it to look different.

### Theme Coverage Varies

Different Zed themes have different scope coverage:
- Some themes style many scopes distinctly
- Other themes use the same color for many related scopes
- `@punctuation.*` scopes are often not given distinct colors

## Potential Solutions

### Option 1: Use `.markup` Suffixed Scopes (Like Zed's Markdown)

Change scopes to match Zed's built-in markdown pattern:

```scm
; Current (standard tree-sitter)
(strong_emphasis) @emphasis.strong
(emphasis) @text.emphasis
(link_text) @text.reference

; Potential alternative (Zed markdown style)
(strong_emphasis) @emphasis.strong.markup
(emphasis) @text.emphasis.markup
(link_text) @link_text.markup
```

**Pros**: Might get better theme support
**Cons**: Non-standard, deviates from tree-sitter conventions

### Option 2: Document as Known Limitation

Accept that theme styling varies and document it for users.

**Pros**: Honest, acknowledges the ecosystem reality
**Cons**: Users may be disappointed

### Option 3: Create Custom Theme Overrides

Provide theme override examples in documentation showing how users can add styling:

```json
{
  "experimental.theme_overrides": {
    "syntax": {
      "emphasis.strong": {
        "font_weight": 700
      },
      "text.emphasis": {
        "font_style": "italic"
      },
      "punctuation.delimiter": {
        "color": "#666666"
      }
    }
  }
}
```

**Pros**: Gives users control
**Cons**: Requires manual configuration

## Recommendation

**Combine Options 2 and 3**:

1. Document this as a known limitation in README.md
2. Provide theme override examples for users who want distinct styling
3. Note that the extension is working correctly - it's purely a theme styling issue
4. Consider opening an issue with Zed to request better default theme support for these scopes

## Testing This Theory

To confirm this is a theme issue, users can:

1. Open a `.qmd` file in Zed
2. Place cursor on bold text like `**test**`
3. Check the status bar (should show the scope name)
4. If it shows `emphasis.strong` but looks plain, it's a theme styling issue
5. Try different Zed themes to see if any style it distinctly

## References

- Zed syntax highlighting docs: https://zed.dev/docs/extensions/languages#syntax-highlighting
- Zed themes docs: https://zed.dev/docs/themes
- Zed's markdown extension source: https://github.com/zed-industries/zed/tree/main/crates/languages/src/markdown
