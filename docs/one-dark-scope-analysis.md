# One Dark Theme Scope Analysis

**Date**: October 18, 2025
**Purpose**: Determine the best scope for emphasis delimiters in Zed

## One Dark Color Palette (Dark Variant)

| Scope | Hex Color | Visual | Usage |
|-------|-----------|--------|-------|
| `primary` | `#acb2beff` | Light gray | Default text |
| `punctuation` | `#acb2beff` | Light gray | General punctuation |
| `punctuation.delimiter` | `#b2b9c6ff` | Lighter gray | Delimiters (`,`, `;`, `:`) |
| `punctuation.bracket` | - | - | Brackets `[]`, `()` |
| `punctuation.special` | - | - | Special chars `#`, `@` |
| `emphasis` | `#74ade8ff` | Blue | Italic content |
| `emphasis.strong` | `#bf956aff` | Orange/tan | Bold content |
| `text.literal` | `#a1c181ff` | Green | Code spans/blocks |
| `comment` | TBD | Gray/dim | Comments |

## Current Implementation

### What We're Using Now

```scheme
; Test (changed today)
(emphasis_delimiter) @comment
(strong_emphasis_delimiter) @comment

; Content
(emphasis) @emphasis             # Blue #74ade8ff
(strong_emphasis) @emphasis.strong  # Orange #bf956aff
```

### What We Should Use

```scheme
; Delimiters (proper scope)
(emphasis_delimiter) @punctuation.delimiter        # Light gray #b2b9c6ff
(strong_emphasis_delimiter) @punctuation.delimiter # Light gray #b2b9c6ff

; Content
(emphasis) @emphasis                    # Blue #74ade8ff
(strong_emphasis) @emphasis.strong      # Orange #bf956aff
```

## Color Comparison

For `*italic text*` in One Dark:

| Element | Current (@comment) | Correct (@punctuation.delimiter) | Ideal (Different Color) |
|---------|-------------------|----------------------------------|------------------------|
| `*` markers | Comment color (TBD) | Light gray `#b2b9c6ff` | Dimmer/different |
| "italic text" | Blue `#74ade8ff` | Blue `#74ade8ff` | Same |

## The Problem

The issue is that `punctuation.delimiter` (`#b2b9c6ff`) is **very similar** to the primary text color (`#acb2beff`). The difference is minimal:

- Primary: `#acb2be` (RGB: 172, 178, 190)
- Delimiter: `#b2b9c6` (RGB: 178, 185, 198)

This is only ~3-6 points difference per channel - barely visible to the human eye!

## Why Delimiters Don't Stand Out

One Dark theme intentionally makes `punctuation.delimiter` similar to normal text because:
1. In code, you want delimiters (commas, semicolons) to be subtle
2. They're structural, not semantic
3. The focus should be on keywords, strings, functions

But for **Markdown emphasis**, we actually WANT delimiters to be dimmer/grayer so the content stands out.

## Other Scopes We Could Try

| Scope | One Dark Color | Pros | Cons |
|-------|----------------|------|------|
| `@punctuation.delimiter` | Light gray (subtle) | Correct semantic scope | Too similar to text |
| `@comment` | Dim gray | Very visible difference | Semantically wrong |
| `@punctuation.special` | Unknown | Might be dimmer | Not really "special" |
| Custom color | N/A | Perfect control | Requires theme modification |

## Recommendation

### Option 1: Use @comment (Pragmatic)

**Change**: Keep `@comment` for delimiters
**Result**: Markers will be dimmed/grayed (comment color)
**Trade-off**: Semantically incorrect but visually effective

```scheme
(emphasis_delimiter) @comment
(strong_emphasis_delimiter) @comment
```

**Pros:**
- ✅ Visible difference from content
- ✅ Works with existing themes
- ✅ Matches user expectations from other editors

**Cons:**
- ❌ Not semantically correct
- ❌ Might break with some themes
- ❌ Could affect other features

### Option 2: Use @punctuation.delimiter (Correct)

**Change**: Use proper punctuation scope
**Result**: Markers slightly lighter than default text (barely visible)
**Trade-off**: Correct but not very effective

```scheme
(emphasis_delimiter) @punctuation.delimiter
(strong_emphasis_delimiter) @punctuation.delimiter
```

**Pros:**
- ✅ Semantically correct
- ✅ Future-proof
- ✅ Works with theme conventions

**Cons:**
- ❌ Minimal visual difference
- ❌ Doesn't achieve the UX goal
- ❌ Users won't notice the difference

### Option 3: Create Custom Scope (Advanced)

**Change**: Define a new scope like `@punctuation.markup.delimiter`
**Result**: Depends on theme support
**Trade-off**: Requires theme updates

Not practical for this extension.

## Testing Needed

Check what color `@comment` actually produces in One Dark:
1. Apply current change (using `@comment`)
2. Open `.qmd` with `*italic*` and `**bold**`
3. Compare marker color to content color
4. If dimmed/grayed → keep `@comment`
5. If not different → revert to `@punctuation.delimiter`

## Decision Framework

**If `@comment` makes markers visibly dimmer:**
→ Keep `@comment` (pragmatic choice for better UX)

**If `@comment` looks weird or broken:**
→ Use `@punctuation.delimiter` (correct but subtle)

**If neither works well:**
→ Document as theme limitation, wait for Zed theme improvements

## Next Steps

1. Get comment color from One Dark
2. Test current implementation with `@comment`
3. Make decision based on visual results
4. Document final choice
