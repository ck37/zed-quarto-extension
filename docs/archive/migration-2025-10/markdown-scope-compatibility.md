# Markdown Scope Compatibility Analysis

**Date**: October 18, 2025
**Question**: Why do emphasis markers look distinct in .md files but not in .qmd files?
**Answer**: We need to use the same scope as tree-sitter-markdown-inline

## Discovery

Tree-sitter-markdown-inline uses a **more specific scope** for emphasis delimiters:

```scheme
(emphasis_delimiter) @punctuation.delimiter.emphasis
```

Not just `@punctuation.delimiter`, but the qualified version `@punctuation.delimiter.emphasis`.

## Scope Hierarchy

Zed themes use a **hierarchical fallback** system:

1. Try `@punctuation.delimiter.emphasis` (most specific)
2. If not defined, fall back to `@punctuation.delimiter`
3. If not defined, fall back to `@punctuation`
4. If not defined, use default text color

## One Dark Theme Scopes

One Dark doesn't define `@punctuation.delimiter.emphasis`, so it falls back to:
- `@punctuation.delimiter` → `#b2b9c6ff` (light gray)

This is **slightly lighter** than:
- `primary` text → `#acb2beff` (default text)

## Why Markdown .md Files Look Different

The subtle difference comes from using the proper hierarchical scope:

```scheme
# Our old approach (test)
(emphasis_delimiter) @comment  # Gray #5d636f - very distinct

# Our new approach (matching markdown)
(emphasis_delimiter) @punctuation.delimiter.emphasis  # Light gray #b2b9c6ff - subtle

# Markdown inline grammar uses
(emphasis_delimiter) @punctuation.delimiter.emphasis  # Same!
```

## Updated Implementation

Changed from:
```scheme
(emphasis_delimiter) @comment
(strong_emphasis_delimiter) @comment
```

To match tree-sitter-markdown-inline:
```scheme
(emphasis_delimiter) @punctuation.delimiter.emphasis
(strong_emphasis_delimiter) @punctuation.delimiter.emphasis
```

## Visual Comparison (One Dark)

| Approach | Delimiter Color | Content Color | Visibility |
|----------|----------------|---------------|------------|
| `@comment` | `#5d636f` (dim gray) | `#74ade8` (blue) | ✅ Very distinct |
| `@punctuation.delimiter.emphasis` | `#b2b9c6` (light gray) | `#74ade8` (blue) | ⚠️ Subtle |
| `@punctuation.delimiter` | `#b2b9c6` (light gray) | `#74ade8` (blue) | ⚠️ Subtle |

## The Trade-off

### Using `@punctuation.delimiter.emphasis` (Current)

**Pros:**
- ✅ Matches standard Markdown behavior
- ✅ Semantically correct
- ✅ Compatible with future theme improvements
- ✅ Hierarchical scope allows theme customization

**Cons:**
- ⚠️ Subtle visual difference in One Dark (light gray vs default)
- ⚠️ Markers don't "pop" like with `@comment`

### Using `@comment` (Previous Test)

**Pros:**
- ✅ Very visible difference (dim gray vs blue/orange content)
- ✅ Modern UX pattern (VSCode, Obsidian dim markers)

**Cons:**
- ❌ Semantically incorrect
- ❌ Doesn't match Markdown convention
- ❌ Could break with themes that style comments oddly

## Recommendation

**Use `@punctuation.delimiter.emphasis`** to match tree-sitter-markdown-inline.

This provides:
1. **Consistency** with Zed's Markdown highlighting
2. **Semantic correctness** (it IS a punctuation delimiter for emphasis)
3. **Theme flexibility** (themes can target this specific scope if they want)
4. **Subtle distinction** (light gray vs default text, just like regular Markdown)

## For Theme Authors

To make emphasis markers more distinct, add to your theme:

```json
{
  "syntax": {
    "punctuation.delimiter.emphasis": {
      "color": "#666666",
      "opacity": 0.6
    }
  }
}
```

This would make markers dimmer while keeping them semantically correct.

## Testing

To verify the change:
1. Restart Zed
2. Reinstall extension
3. Open a `.qmd` file with `*italic*` and `**bold**`
4. Compare with a `.md` file with same content
5. Markers should look similar in both files now

## Conclusion

The answer to "why do markers look distinct in .md files?" is:
- They use `@punctuation.delimiter.emphasis`
- Which falls back to `@punctuation.delimiter` (`#b2b9c6`)
- Which is **slightly** lighter than default text (`#acb2be`)
- The difference is subtle but consistent

We've now matched that behavior in Quarto.
