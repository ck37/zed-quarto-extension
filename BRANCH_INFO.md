# Branch: use-pandoc-inline-grammar

This branch uses the full `pandoc_markdown_inline` grammar for inline element highlighting, providing 100% coverage for bold, italic, links, and all Pandoc inline features.

## Requirements

This branch **requires** the Zed fix for extension-to-extension grammar injection to be merged and released.

**Zed PR**: https://github.com/zed-industries/zed/pull/[PR_NUMBER]

**Status**:
- ✅ Fix implemented in Zed branch `fix/extension-grammar-injection`
- ⏳ Testing pending
- ⏳ PR to Zed pending
- ⏳ Zed release pending

## When to Use This Branch

**Use this branch when:**
- The Zed fix has been merged and released
- You're using a Zed build that includes the fix
- You want 100% inline highlighting coverage (not the 70% workaround)

**Do NOT use this branch if:**
- Using official Zed releases without the fix → highlighting won't work
- The Zed PR hasn't been merged yet → stay on `main` branch with workaround

## What's Different from Main

**Main branch (workaround):**
```scheme
((inline) @injection.content
 (#set! injection.language "markdown-inline"))  # Built-in Zed grammar
```
- ✅ Works with all Zed versions
- ⚠️ Only 70% coverage (basic bold/italic)
- ❌ No links, mixed content, or Pandoc features

**This branch (full solution):**
```scheme
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))  # Extension grammar
```
- ✅ 100% coverage (all inline features)
- ✅ Links, mixed content, Pandoc extensions
- ⚠️ Requires Zed fix to be released

## Installation

1. **Verify Zed has the fix:**
   ```bash
   # Check Zed version includes the registry version increment fix
   zed --version
   # Should be version X.Y.Z or later (after fix merges)
   ```

2. **Clone and checkout this branch:**
   ```bash
   git clone https://github.com/ck37/zed-quarto-extension.git
   cd zed-quarto-extension
   git checkout use-pandoc-inline-grammar
   ```

3. **Install in Zed:**
   - Open Zed
   - Command palette: `zed: install dev extension`
   - Select the repository directory

## Testing

Open a `.qmd` file with:
```markdown
**bold text**
*italic text*
***bold italic***
[link text](url)
~~strikethrough~~
H~2~O (subscript)
x^2^ (superscript)
```

All should be highlighted correctly if the fix is working.

## Merging Back to Main

Once the Zed fix is widely released (in stable Zed):

1. Merge this branch to `main`
2. Update README to remove "Known Limitations" about bold/italic
3. Archive investigation documentation (keep for reference)
4. Celebrate 100% coverage! 🎉

## Related Documentation

- [zed-fix-implemented.md](docs/bold-highlighting-investigation/zed-fix-implemented.md) - Details of the Zed fix
- [verification-findings.md](docs/bold-highlighting-investigation/verification-findings.md) - Root cause analysis
- [README.md](README.md) - Main extension documentation (workaround branch)
