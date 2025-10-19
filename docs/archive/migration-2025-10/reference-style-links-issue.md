# Reference-Style Links Issue

## Problem

Reference-style links are not highlighting correctly in Zed. These are links that use reference labels instead of inline URLs:

```markdown
This is a [reference link][ref1].

[ref1]: https://example.com
```

## Current Status

### What Works ✅

1. **Inline links**: `[link text](https://example.com)` - Fixed in scope validation work
2. **Link reference definitions**: `[ref]: https://example.com` - Highlighting works

### What Doesn't Work ❌

**Reference usage in text**: `[link text][ref]` - Not highlighting

## Root Cause

This is a **grammar parsing limitation**, not a highlighting query issue.

### Evidence

When parsing reference-style links:

```bash
$ tree-sitter parse test-ref-simple.qmd
(document
  (paragraph
    content: (inline
      (text))
    (ERROR                                    # <-- Parse error!
      (link_text)
      (link_text)
      (ERROR)))
  (link_reference_definition                  # <-- This part works
    label: (reference_label)
    destination: (link_destination)))
```

The grammar:
- ✅ **Successfully parses** `link_reference_definition` (the `[ref]: URL` part)
- ❌ **Fails to parse** the reference usage `[text][ref]` (produces ERROR nodes)

## Why This Happens

Reference-style links require complex parsing:
1. Parse `[link text]` as potential link
2. Look ahead for `[ref]` or implicit reference
3. Later resolve `[ref]` against `link_reference_definition`

This requires either:
- More sophisticated grammar rules for inline content
- Two-pass parsing (not supported by tree-sitter)

## Impact

**Severity**: Low to Medium
- Reference-style links are less common in Quarto documents
- Users typically use inline links: `[text](URL)`
- Link definitions still work (the `[ref]: URL` part highlights)

**Workaround**: Use inline links instead of reference-style links.

## Potential Solutions

### Solution 1: Fix in Grammar (Recommended)

This requires upstream work in `tree-sitter-quarto`:
1. Add grammar rules to recognize `[text][ref]` pattern
2. Create appropriate AST nodes (e.g., `link_reference`)
3. Add test cases for reference-style links

**Effort**: Medium (grammar work)
**Impact**: Complete fix

### Solution 2: Partial Highlighting (Workaround)

We could add queries to highlight the pieces that *do* parse:
- The `link_text` nodes that appear in ERROR
- The `link_reference_definition` nodes

```scheme
; Highlight link text even in error contexts (may have false positives)
(ERROR (link_text) @link_text.markup)
```

**Effort**: Low (query work)
**Impact**: Partial fix, may cause false positives

### Solution 3: Document Limitation (Current)

Document that reference-style links aren't supported and recommend inline links.

**Effort**: Minimal
**Impact**: None (but sets expectations)

## Recommendation

1. **Short term**: Document the limitation (this file)
2. **Medium term**: Open issue in tree-sitter-quarto grammar repo
3. **Long term**: Contribute grammar fix upstream

## Related

- Inline links fixed: `languages/quarto/highlights.scm` lines 135-147
- Link definitions work: `languages/quarto/highlights.scm` lines 259-264
- Grammar source: https://github.com/ck37/tree-sitter-quarto

## Test Cases

To verify any future fix:

```markdown
# Test 1: Basic reference
Here is [a link][1].

[1]: https://example.com

# Test 2: Named reference
Here is [link text][ref-name].

[ref-name]: https://example.com "Optional Title"

# Test 3: Implicit reference
Here is [Example].

[Example]: https://example.com

# Test 4: Multiple references
First [link][1] and second [link][2].

[1]: https://first.com
[2]: https://second.com
```

All four should highlight:
- `[a link]`, `[link text]`, etc. with `@link_text.markup`
- `[1]`, `[ref-name]`, etc. with appropriate scope
- URL in definitions with `@link_uri.markup`

## Status

- **Reported**: 2025-10-17
- **Investigated**: 2025-10-17
- **Cause**: Grammar parsing limitation
- **Upstream issue**: Not yet filed
- **Workaround**: Use inline links

## See Also

- `docs/scope-validation-summary.md` - Scope validation work
- `docs/zed-syntax-scopes.md` - Complete scope reference
- `tests/zed_scope_validation.rs` - Scope validation tests
