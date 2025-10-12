# Built-in markdown-inline Injection Test

## Test Date
2025-10-12

## Hypothesis
Instead of injecting the custom `pandoc_markdown_inline` grammar (which doesn't work due to Zed limitation), try injecting Zed's built-in `markdown-inline` grammar. Extensions CAN inject built-in languages, so this should work.

## Implementation
Added to `languages/quarto/injections.scm`:
```scheme
; Try injecting Zed's built-in markdown-inline grammar for emphasis, links, etc.
((inline) @injection.content
 (#set! injection.language "markdown-inline"))
```

This uses the exact same pattern as Zed's built-in markdown.

## Test Results

Tested with `test-builtin-inline.qmd`:

### ✅ What Worked
- **Single asterisk italic** (`*italic*`) - WORKS

### ❌ What Failed
- **Bold with double asterisks** (`**bold**`) - NO HIGHLIGHTING
- **Bold with double underscores** (`__bold__`) - NO HIGHLIGHTING
- **Triple asterisks** (`***bold italic***`) - NO HIGHLIGHTING
- **Links** (`[text](url)`) - NO HIGHLIGHTING
- **Bold in mixed content** - NO HIGHLIGHTING
- All other inline features - NO HIGHLIGHTING

### Coverage
- ✅ Working: ~10% (only single asterisk italic)
- ❌ Broken: ~90% (bold, links, everything else)

## Root Cause

The built-in `markdown-inline` grammar IS being injected (proven by italic working), but it's incompatible with Pandoc's `(inline)` node content.

**Likely reasons:**
1. Pandoc's block grammar tokenizes content differently than tree-sitter-markdown
2. The built-in markdown-inline expects specific delimiters/structure
3. Pandoc includes additional syntax that confuses the standard markdown parser
4. Different escaping or character handling between grammars

## Conclusion

**FAILED**: Built-in injection is not viable.

While extensions CAN inject built-in languages, the built-in `markdown-inline` grammar is too incompatible with Pandoc's block grammar to provide useful highlighting.

## Recommendation

Revert this change and pursue **Alternative Approach #1** from ALTERNATIVE_APPROACHES.md:
- Use merged grammar highlights (single grammar approach)
- No injection needed
- Block grammar already includes inline node highlights

## Related Issues

- Original investigation: [README.md](./README.md)
- Zed limitation: [Issue #484](https://github.com/zed-industries/zed/issues/484)
