# Inline Content Highlighting Investigation

**Date:** 2025-10-17
**Branch:** tree-sitter-quarto-migration
**Grammar:** tree-sitter-quarto @ 96638a5eaba58df4cf2ac270af844d19d8fff083

## Problem Statement

Users report that inline content (text inside emphasis, strong emphasis, and headings) is not being highlighted in Zed. Specifically:

1. **Emphasis/Strong**: Content inside `*italic*`, `**bold**`, `***bold italic***` is not highlighted
2. **Headings**: Content inside `# heading text` is not highlighted
3. **New Pandoc formatting**: Strikethrough, highlight, subscript, superscript may have similar issues

Only the markers (`*`, `**`, `#`) receive styling - the actual text content does not.

## Investigation

### Phase 1: Grammar Analysis

#### AST Structure (CONFIRMED CORRECT ✅)

Parsed `*italic text*` with tree-sitter:

```
(paragraph [13, 0] - [15, 0]
  content: (inline [13, 0] - [13, 28]
    (emphasis [13, 0] - [13, 28]
      (emphasis_delimiter [13, 0] - [13, 1])
      (text [13, 1] - [13, 27])          ← TEXT NODE EXISTS!
      (emphasis_delimiter [13, 27] - [13, 28]))))
```

Parsed `# Heading Text`:

```
(atx_heading [6, 0] - [8, 0]
  marker: (atx_heading_marker [6, 0] - [6, 2])
  content: (inline [6, 2] - [6, 32]
    (text [6, 2] - [6, 32])))            ← TEXT NODE EXISTS!
```

Parsed `**bold text**`:

```
(paragraph [32, 0] - [34, 0]
  content: (inline [32, 0] - [32, 28]
    (strong_emphasis [32, 0] - [32, 28]
      (strong_emphasis_delimiter [32, 0] - [32, 2])
      (text [32, 2] - [32, 26])          ← TEXT NODE EXISTS!
      (strong_emphasis_delimiter [32, 26] - [32, 28]))))
```

**Conclusion:** The grammar correctly parses the structure with child `(text)` nodes inside `(emphasis)`, `(strong_emphasis)`, and `(inline)` nodes. **The issue is NOT in the grammar.**

### Phase 2: Current Query Analysis

#### Current Queries (languages/quarto/highlights.scm)

```scheme
; Headings
(atx_heading
  (atx_heading_marker) @punctuation.special
  content: (inline) @text.title)

; Emphasis
(emphasis) @text.emphasis

; Strong
(strong_emphasis) @emphasis.strong

; Pandoc Extensions
(strikethrough) @text.strike
(highlight) @text.highlight
(subscript) @text.subscript
(superscript) @text.super

; Catch-all
(text) @text
```

#### Problem Hypothesis

Tree-sitter highlighting applies scopes on a **per-node basis**. When we apply `@text.emphasis` to the parent `(emphasis)` node, it highlights the node container but may not automatically propagate styling to descendant `(text)` nodes.

Potential issues:
1. **Zed's theme doesn't inherit styles** from parent nodes to child nodes
2. **The `(text) @text` catch-all query** (line 189) may override parent styles with lower priority
3. **Query ordering** may cause parent scopes to not apply to children

### Phase 3: Query Syntax Validation

**CRITICAL FINDING:** The grammar update claim was misleading!

When testing the queries with tree-sitter CLI, discovered that:
- `strikethrough`, `highlight`, `subscript`, `superscript` nodes **DO NOT EXIST** in the grammar
- These patterns parse as plain `(text)` nodes
- The queries referencing these nodes cause the entire highlighting system to fail silently in Zed

**Test Results:**
```
~~strikethrough~~ → (text [0, 0] - [0, 17])
==highlight== → (text [2, 0] - [2, 13])
H~2~O → (text [4, 0] - [4, 5])
x^2^ → (ERROR [6, 0] - [6, 4])
```

All Pandoc formatting extensions are unparsed. The grammar needs actual implementation of these features.

## Potential Solutions

###Solution 1: Explicit Child Captures

Add explicit queries that capture child text nodes:

```scheme
(emphasis
  (text) @text.emphasis)

(strong_emphasis
  (text) @emphasis.strong)

(atx_heading
  content: (inline
    (text) @text.title))
```

### Solution 2: Priority Rules

Use `(#set! "priority" N)` predicates to ensure parent styling takes precedence over catch-all:

```scheme
((emphasis) @text.emphasis
  (#set! "priority" 100))

((text) @text
  (#set! "priority" 50))
```

### Solution 3: Remove Catch-all

Remove the `(text) @text` catch-all query to prevent it from overriding parent styles.

**Risk:** May break other highlighting patterns that rely on it.

### Solution 4: Combined Approach

- Add explicit child captures for emphasis/strong/headings
- Keep catch-all with lower priority
- Test with various nesting scenarios

## Root Cause Discovered!

### The Catch-All Override Problem

Through deep analysis with `tests/highlight_capture_analysis.rs`, discovered the ACTUAL root cause:

**The `(text) @text` catch-all query was overriding all parent scopes!**

Example of what was happening:
```
▼ HighlightStart: @text.emphasis     ← Parent emphasis starts
  ▼ HighlightStart: @text.emphasis   ← Delimiter gets emphasis
    Source[0..1]: "*"
  ▲ HighlightEnd
  ▼ HighlightStart: @text             ← TEXT GETS @text, NOT @text.emphasis!
    Source[1..12]: "italic text"      ← Content uses catch-all, not parent!
  ▲ HighlightEnd
```

**Why this matters:** Tree-sitter applies the **innermost/last scope** to each node. When the catch-all `(text) @text` captures text nodes, it overrides any parent scope like `@text.emphasis`.

**Why headings "worked" initially:** The heading queries captured the `(inline)` parent node directly, so the parent scope applied before the catch-all could override it. But the content still had the wrong scope internally.

### The Fix

**Removed the catch-all `(text) @text` query entirely.**

After removal:
```
▼ HighlightStart: @text.emphasis     ← Parent emphasis starts
  ▼ HighlightStart: @text.emphasis   ← Delimiter gets emphasis
    Source[0..1]: "*"
  ▲ HighlightEnd
  ▼ HighlightStart: @text.emphasis   ← CONTENT NOW GETS @text.emphasis!
    Source[1..12]: "italic text"      ← ✓ Correct scope!
  ▲ HighlightEnd
```

Child text nodes now properly inherit from their parent formatting nodes.

## Solution Implemented

### Fix Applied (2025-10-17) - FINAL VERSION

Added explicit child captures in `languages/quarto/highlights.scm`:

```scheme
; Headings - capture parent and children
(atx_heading
  content: (inline) @text.title)
(atx_heading
  content: (inline (text) @text.title))
(atx_heading
  content: (inline (_) @text.title))

; Emphasis - capture parent and children
(emphasis) @text.emphasis
(emphasis (text) @text.emphasis)
(emphasis (_) @text.emphasis)

; Strong - capture parent and children
(strong_emphasis) @emphasis.strong
(strong_emphasis (text) @emphasis.strong)
(strong_emphasis (_) @emphasis.strong)
```

**Removed invalid queries** for Pandoc extensions that don't exist in grammar yet.

## Next Steps

1. ✅ Create comprehensive test fixture (`tests/fixtures/inline-highlighting.qmd`)
2. ✅ Analyze AST structure (CONFIRMED: Grammar parses emphasis/strong/headings correctly)
3. ✅ Validate queries with tree-sitter CLI (Found invalid node types)
4. ✅ Implement fix with explicit child captures
5. ✅ Remove invalid Pandoc extension queries
6. ⏳ **User testing in Zed** - Waiting for confirmation that highlighting works
7. ⏳ Add automated tests if fix works
8. ⏳ File issue about missing Pandoc extension support in tree-sitter-quarto grammar

## References

- OpenSpec proposal: `openspec/changes/fix-inline-highlighting/`
- Test fixture: `tests/fixtures/inline-highlighting.qmd`
- Highlight capture test: `tests/highlight_capture_test.rs`
- Current queries: `languages/quarto/highlights.scm`
- Related: `docs/bold-highlighting-investigation/` (old dual-grammar approach)
