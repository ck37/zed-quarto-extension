# Hyphenated Headings Highlighting Issue

**Status:** Known limitation with tree-sitter-pandoc-markdown dual grammar approach
**Tracking:** Part of [#3](https://github.com/ck37/zed-quarto-extension/issues/3)
**Workaround:** Use tree-sitter-quarto grammar (recommended)

## Problem

Headings containing hyphens (e.g., `## Non-Longitudinal Clustering`) do not receive heading color styling, while non-hyphenated headings work correctly.

**Affected:**
```markdown
## Longitudinal vs Non-Longitudinal Clustering  ❌ Not colored
### Test-With-Hyphen                            ❌ Not colored
```

**Working:**
```markdown
## Introduction                                 ✅ Colored correctly
### Longitudinal Clustering                     ✅ Colored correctly
```

## Root Cause

The issue stems from the **dual grammar architecture** (separate block and inline grammars with injection):

1. Block grammar (`pandoc_markdown`) parses document structure and captures `(atx_heading) @title`
2. Inline grammar (`pandoc_markdown_inline`) injects into `(inline)` nodes to parse bold/italic/links
3. When the inline grammar parses heading text containing hyphens, it creates `(text)` nodes
4. These text nodes have no scope in the inline grammar, causing them to lose the parent heading color
5. The inline grammar cannot "see" that it's inside a heading (injection loses parent context)

### Why Some Headings Work

Headings without special characters that trigger inline parsing (like hyphens adjacent to word boundaries) may be parsed as a single text node that doesn't get split, allowing the parent heading scope to apply.

## Attempted Solutions

### 1. Capture `(inline)` child in block grammar ❌
```scheme
(atx_heading (inline) @text.title)  ; Blocks injection from working
```
**Result:** Prevents inline grammar injection entirely, breaking bold/italic in headings.

### 2. Add text capture in inline grammar ❌
```scheme
(text) @text  ; Overrides parent scope
```
**Result:** All text gets generic `@text` scope, overriding heading colors everywhere.

### 3. Exclude headings from injection ❌
```scheme
((inline) @injection.content
  (#not-has-ancestor? @injection.content atx_heading))
```
**Result:** Non-hyphenated headings work, but bold/italic in headings doesn't work.

### 4. Use Zed's `@title` scope ❌
```scheme
(atx_heading) @title  ; Instead of @text.title
```
**Result:** Doesn't solve the underlying injection + hyphen parsing issue.

### 5. Priority adjustments ❌
```scheme
((atx_heading) @title (#set! "priority" 90))
```
**Result:** Priority doesn't affect how child text nodes inherit parent scopes.

## Why This Is Hard to Fix

Tree-sitter **injections are isolated** - the injected grammar (pandoc_markdown_inline) has no knowledge of its parent context (that it's inside a heading). The inline grammar can't conditionally apply different scopes based on parent nodes.

Solutions would require:
- External scanner to track context across grammars (complex)
- Modifying tree-sitter itself to pass parent context to injections (not feasible)
- Using a single unified grammar instead of dual grammar (this is what tree-sitter-quarto does)

## Recommendation: Switch to tree-sitter-quarto

**tree-sitter-quarto** uses a **unified grammar** that handles both block and inline parsing in one grammar, avoiding the injection context loss issue.

**Benefits:**
- ✅ No dual grammar injection issues
- ✅ Quarto-specific features (chunk options, cross-references, callouts)
- ✅ Maintained specifically for Quarto use cases
- ✅ Alpha complete with 58/58 tests passing

**Migration path:** See [#3](https://github.com/ck37/zed-quarto-extension/issues/3) for tree-sitter-quarto integration plan.

## Comparison with Zed's Markdown

**Question:** How does Zed's built-in markdown handle this?

**Answer:** Zed's markdown also uses dual grammars with injection, but:
- Zed's `markdown` and `markdown-inline` grammars are simpler (no Pandoc extensions)
- The inline grammar doesn't capture plain text nodes at all
- This allows parent heading scopes to "show through"

However, testing reveals Zed's markdown likely has the same issue with certain character combinations that trigger inline parsing. The hyphen issue may be specific to how tree-sitter-pandoc-markdown's inline grammar tokenizes hyphens in certain contexts.

## Workaround (Current)

**For users:** Avoid hyphens in heading text, or accept that some headings won't be specially colored.

**For developers:** Switch to tree-sitter-quarto grammar (see issue #3).

## References

- Original investigation: PR analysis in `docs/bold-highlighting-investigation/`
- Zed markdown implementation: `/tmp/zed/crates/languages/src/markdown/`
- tree-sitter injection limitations: [tree-sitter docs](https://tree-sitter.github.io/tree-sitter/syntax-highlighting#language-injection)

---

**Last Updated:** 2025-10-14
**Status:** Documented, recommend migration to tree-sitter-quarto
