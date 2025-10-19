# Language Grammar Requirements for Code Block Highlighting

**Issue**: Code blocks may not have syntax highlighting even with correct injection queries
**Cause**: Zed needs the target language's grammar installed
**Solution**: Install language support for each language you use

## How Language Injection Works

When the Quarto extension encounters a code block like:

```markdown
```python
import numpy as np
```
```

The extension's injection queries tell Zed:
1. "This is Python code" (`#set! injection.language "python"`)
2. "Highlight this content" (`(code_line) @injection.content`)

But Zed can only highlight Python **if it has a Python grammar installed**.

## Installing Language Support

### Via Zed UI

1. Open a file with the language extension (e.g., `test.r`, `query.sql`)
2. Zed will prompt to install language support
3. Click "Install"

**OR**

1. Open Command Palette (`Cmd+Shift+P`)
2. Type "zed: install language server"
3. Select the language

### Commonly Used Languages in Quarto

| Language | Extension to Open | Notes |
|----------|------------------|-------|
| **Python** | `.py` | Usually pre-installed |
| **R** | `.r` | May need manual install |
| **SQL** | `.sql` | May need manual install |
| **JavaScript** | `.js` | Usually pre-installed |
| **TypeScript** | `.ts` | Usually pre-installed |
| **Julia** | `.jl` | May need manual install |
| **Bash** | `.sh` | Usually pre-installed |
| **JSON** | `.json` | Usually pre-installed |
| **YAML** | `.yaml` | Usually pre-installed |
| **HTML** | `.html` | Usually pre-installed |
| **CSS** | `.css` | Usually pre-installed |

## Verification

After installing a language, create a test `.qmd` file:

```markdown
# Test

```python
print("Python works!")
```

```r
print("R works!")
```

```sql
SELECT * FROM users;
```
```

All code blocks should now have syntax highlighting.

## Troubleshooting

### Code block has no highlighting

**Check:**
1. Is the language grammar installed in Zed?
2. Is the language name correct? (e.g., `python` not `Python`)
3. Does opening a `.py` file show highlighting?

**Solution:**
- Install language support for that language
- Restart Zed after installing

### Some languages work, others don't

**This is expected!** Each language needs its own grammar installed. The Quarto extension provides the **injection queries** (which code to highlight), but Zed provides the **grammars** (how to highlight it).

### Quarto executable cells vs standard Markdown

Both styles use the same injection mechanism:
- ` ```{python}` → Uses `executable_code_cell` injection
- ` ```python` → Uses `fenced_code_block` injection

Both require the same language grammar to be installed.

## Supported Languages

The Quarto extension has injection queries for:

### Quarto Executable Cells
- Python (`{python}`, `{python3}`)
- R (`{r}`)
- Julia (`{julia}`)
- SQL (`{sql}`)
- Bash/Shell (`{bash}`, `{sh}`, `{shell}`)
- JavaScript (`{javascript}`, `{js}`)
- TypeScript (`{typescript}`, `{ts}`)
- Observable JS (`{ojs}`)

### Standard Markdown Code Blocks
- Python (`python`)
- R (`r`)
- Julia (`julia`)
- JavaScript (`javascript`, `js`)
- TypeScript (`typescript`, `ts`)
- Bash (`bash`, `sh`)
- SQL (`sql`)
- JSON (`json`)
- YAML (`yaml`)
- TOML (`toml`)
- HTML (`html`)
- CSS (`css`)
- Markdown (`markdown`)

### Inline Code Cells
- Python (`` `{python} expr` ``)
- R (`` `{r} expr` ``)
- Julia (`` `{julia} expr` ``)

## Adding Support for New Languages

If you need a language not listed above, you can add it to your local copy:

1. Edit `languages/quarto/injections.scm`
2. Add a pattern like:
   ```scheme
   ((fenced_code_block
     info: (info_string) @_lang
     (#eq? @_lang "your-language")
     (code_line) @injection.content)
    (#set! injection.language "your-language")
    (#set! injection.combined))
   ```
3. Make sure Zed has that language's grammar installed
4. Rebuild and reinstall the extension

## Related

- Issue #7: https://github.com/ck37/tree-sitter-quarto/issues/7 (injection fix)
- `docs/update-to-4012bc7-2025-10-18.md` (injection fix details)
- `languages/quarto/injections.scm` (full injection query file)

## Summary

✅ **Extension provides**: Injection queries (what to highlight, where)
✅ **Zed provides**: Language grammars (how to highlight)
⚠️ **You need**: Language support installed for each language you use

The extension can't highlight a language that Zed doesn't have a grammar for, just like how Zed can't highlight an `.r` file without R language support installed.
