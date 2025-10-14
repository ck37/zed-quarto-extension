# Citation and Cross-Reference Highlighting Issue

## Status: RESOLVED

**Date**: 2025-10-13
**Resolution**: Citations now working with `@constant` scope
**Zed Version**: Custom build with PR #40063 (extension-to-extension grammar injection)
**Extension Version**: 0.1.0
**Grammar Commit**: `40ee81adf88b8d85eef939da6efcb6593dc4324a` (zed-compatible-scopes branch)

## Problem Statement

Citations (`@smith2024`, `[@doe2020]`) and cross-references (`@fig:plot`, `@tbl:data`) are not being highlighted in Quarto (`.qmd`) files, despite the inline grammar being successfully injected.

## Current Status

**Working:**
- ✅ Bold (`**text**`)
- ✅ Italic (`*text*`)
- ✅ Triple asterisks (`***text***`)
- ✅ Code blocks
- ✅ Headings
- ✅ Inline code

**Not Working:**
- ❌ Citations: `@smith2024`
- ❌ Citation groups: `[@doe2020 p. 4]`
- ❌ Cross-references: `@fig:plot`, `@tbl:data`
- ❌ (Unknown) Footnote references: `[^note]`

## Confirmed Facts

1. **Grammar injection IS working** - Confirmed by the fact that emphasis highlighting works (which is inline-grammar-only)
2. **Injection language name** - Successfully changed from `"pandoc_markdown_inline"` to `"Pandoc Markdown Inline"` to match registered language name
3. **Emphasis scopes work** - `@text.emphasis` and `@emphasis.strong` are recognized by the theme
4. **The grammar parses citations correctly** - Confirmed by upstream test suite (see `tree-sitter-pandoc-markdown-inline/test/corpus/foundation.txt`)

## Attempted Fixes

### Scope Names Tried for Citations/Cross-references

All attempts in `/Library/Application Support/Zed/extensions/installed/quarto/grammars/pandoc_markdown_inline/tree-sitter-pandoc-markdown-inline/queries/highlights.scm`:

| Scope | Result | Notes |
|-------|--------|-------|
| `@text.reference` | ❌ No highlighting | Original from grammar |
| `@tag` | ❌ No highlighting | Common for HTML-like tags |
| `@string` | ⚠️ Untested | Same scope that works for emphasis test |
| `@string.special.symbol` | ❌ No highlighting | Used for special identifiers |
| `@constant` | ⚠️ Not yet tried | Often used for special values |
| `@variable` | ⚠️ Not yet tried | Generic identifier |
| `@function` | ⚠️ Not yet tried | Function/method names |
| `@keyword` | ⚠️ Not yet tried | Language keywords |

**Note**: The system reminders show the file keeps reverting to `@text.reference`, which suggests either:
- Zed is re-loading from a cached/different location
- The file is being overwritten by something
- We're editing the wrong file

## Grammar Structure

Citations in the inline grammar are defined as:

```scm
; From tree-sitter-pandoc-markdown-inline/queries/highlights.scm
(citation_group) @<scope>
(citation) @<scope>
(cross_reference) @<scope>
```

The grammar nodes exist and parse correctly (verified by test corpus).

## Investigation Hypotheses

### Hypothesis 1: Wrong File Being Loaded
**Theory**: Zed may be loading highlights.scm from a different location than where we're editing.

**Test Steps**:
1. Check if there are multiple copies of the highlights.scm file:
   ```bash
   find "/Users/ck432/Library/Application Support/Zed/extensions" -name "highlights.scm" -path "*pandoc*inline*"
   ```
2. Check if Zed caches compiled queries somewhere else
3. Try using a unique, obviously wrong scope (like `@TESTING`) to see if changes are being picked up

**Expected**: If this is the issue, using `@TESTING` won't cause any errors, meaning our file isn't being loaded.

### Hypothesis 2: Grammar Nodes Don't Match
**Theory**: The grammar may not be producing the expected node types for citations.

**Test Steps**:
1. Create a test file with just a citation: `test-citation.txt` containing `@smith2024`
2. Parse it directly with the inline grammar:
   ```bash
   cd "/Users/ck432/Library/Application Support/Zed/extensions/installed/quarto/grammars/pandoc_markdown_inline/tree-sitter-pandoc-markdown-inline"
   echo "@smith2024" | npx tree-sitter-cli parse --quiet
   ```
3. Check the output node names match what's in highlights.scm

**Expected**: Should see `(citation)` node in parse tree.

### Hypothesis 3: Scope Name Not Supported by Theme
**Theory**: The Zed theme doesn't have definitions for any citation-related scopes.

**Test Steps**:
1. Use `@string` scope (known to work from emphasis test):
   ```scm
   (citation_group) @string
   (citation) @string
   (cross_reference) @string
   ```
2. Restart Zed and test
3. If this works, we know the theme is the limitation

**Expected**: If theme is the issue, using `@string` should show citations in string color.

### Hypothesis 4: Grammar Injection Not Capturing Citations
**Theory**: Citations might not be inside `(inline)` nodes, so they're not being injected.

**Test Steps**:
1. Parse a full qmd file with the block grammar to see structure:
   ```bash
   cd "/Users/ck432/Library/Application Support/Zed/extensions/installed/quarto/grammars/pandoc_markdown"
   cat << 'EOF' > /tmp/test.qmd
   Here is a citation @smith2024.
   EOF
   cd tree-sitter-pandoc-markdown
   cat /tmp/test.qmd | npx tree-sitter-cli parse --quiet
   ```
2. Check if `@smith2024` is inside an `(inline)` node

**Expected**: Should see structure like `(paragraph (inline ... (citation) ...))`

### Hypothesis 5: Query Syntax Error
**Theory**: The query might have a syntax error that's silently failing.

**Test Steps**:
1. Check Zed logs for query errors:
   ```bash
   tail -100 ~/Library/Logs/Zed/Zed.log | grep -i "query\|citation\|pandoc_markdown_inline"
   ```
2. Try a simpler query pattern:
   ```scm
   (citation) @string
   ```
   (no child node captures)

**Expected**: Should see error messages in logs if query is malformed.

### Hypothesis 6: Node Type Name Mismatch
**Theory**: The grammar might use different node names than we think (e.g., `citation_ref` instead of `citation`).

**Test Steps**:
1. List all node types in the grammar:
   ```bash
   cd "/Users/ck432/Library/Application Support/Zed/extensions/installed/quarto/grammars/pandoc_markdown_inline/tree-sitter-pandoc-markdown-inline"
   grep -r "citation\|cross_reference" src/
   ```
2. Check the grammar.js file:
   ```bash
   grep "citation\|cross_reference" grammar.js
   ```

**Expected**: Should see exact node type names defined in grammar.

### Hypothesis 7: Zed Doesn't Support Nested Injections
**Theory**: Citations inside injected inline grammar might not be highlighted (nested injection limitation).

**Test Steps**:
1. Check if other inline-only elements work (we already know emphasis works, but try others):
   - Inline math: `$E = mc^2$`
   - Inline footnotes: `^[inline note]`
   - Attribute spans: `[text]{.class}`
2. If these don't work either, it's a Zed limitation

**Expected**: If this is the issue, no inline-specific elements except emphasis will work.

## Next Steps

1. **Immediate**: Test Hypothesis 3 (use `@string` scope) - quickest validation
2. **Then**: Test Hypothesis 2 (verify grammar nodes) - confirms grammar is working
3. **Then**: Test Hypothesis 1 (check file locations) - ensures we're editing the right file
4. **If needed**: Test remaining hypotheses systematically

## Workaround Options

If Zed theme doesn't support appropriate scopes:

1. **Use existing supported scopes**: Map citations to `@string`, `@constant`, or `@variable`
2. **Request Zed theme support**: File issue to add `@string.special.symbol` or `@text.reference` support
3. **Custom theme**: User could create/modify a Zed theme with citation scopes
4. **Accept limitation**: Document that citations aren't highlighted in current Zed version

## Related Files

- Grammar highlights: `/Library/Application Support/Zed/extensions/installed/quarto/grammars/pandoc_markdown_inline/tree-sitter-pandoc-markdown-inline/queries/highlights.scm`
- Injection config: `/Library/Application Support/Zed/extensions/installed/quarto/languages/quarto/injections.scm`
- Grammar source: `https://github.com/ck37/tree-sitter-pandoc-markdown/tree/zed-compatible-scopes`
- Test corpus: `tree-sitter-pandoc-markdown-inline/test/corpus/foundation.txt` (lines 376-404)

## Resolution Summary

### Root Cause
The issue was that we were editing the **wrong highlights.scm file**. Zed uses the language override file at `languages/pandoc_markdown_inline/highlights.scm`, not the grammar's built-in file at `grammars/pandoc_markdown_inline/tree-sitter-pandoc-markdown-inline/queries/highlights.scm`.

### Discovery Process
1. Confirmed grammar supports citations (tested with `tree-sitter parse`)
2. Confirmed grammar injection works (emphasis highlighting worked)
3. Tested with `@string` scope - **this worked!**
4. Discovered we were editing the wrong file
5. Updated the correct file (language override)
6. Switched to `@constant` scope (semantically more appropriate)

### Final Solution
Changed citation scopes from `@text.reference` (unsupported by Zed themes) to `@constant` (widely supported):

```scm
; Citations - using @constant (semantically better than @string)
(citation_group) @constant
(citation) @constant

; Cross-references
(cross_reference) @constant

; Footnotes
(footnote_reference) @constant
(inline_footnote) @constant
```

### Key Learnings
1. **Language override files take precedence** - Always check `languages/*/highlights.scm` first
2. **Not all scopes are supported** - Zed themes don't support all nvim-treesitter scopes
3. **Test with known-working scopes** - Using `@string` as a test confirmed the queries worked
4. **Semantic compromise** - Sometimes need to use less-semantic but supported scopes

### Working Scopes in Zed
- ✅ `@constant` - Used for citations (constants/references)
- ✅ `@string` - Strings
- ✅ `@text.emphasis` - Italic
- ✅ `@emphasis.strong` - Bold
- ✅ `@text.literal` - Code spans
- ✅ `@tag` - HTML tags
- ❌ `@text.reference` - Not supported
- ❌ `@markup.bold` / `@markup.italic` - Not supported (Zed uses different convention)

## References

- Zed PR #40063: Extension-to-extension grammar injection
- Tree-sitter query syntax: https://tree-sitter.github.io/tree-sitter/syntax-highlighting#queries
- Nvim-treesitter scopes: https://github.com/nvim-treesitter/nvim-treesitter/blob/master/CONTRIBUTING.md#highlights
