# Code Block Syntax Highlighting Fix

**Date**: October 18, 2025
**Issue**: Code blocks were not getting syntax highlighting in Zed
**Status**: Fixed

## Problem

Fenced code blocks (standard Markdown style like ` ```python`) were not getting syntax highlighting because the injection queries were missing the `@injection.content` capture.

## Root Cause

The injection queries in `languages/quarto/injections.scm` had patterns like:

```scheme
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "python"))
 (#set! injection.language "python"))
```

This sets the injection language but doesn't tell tree-sitter **what content** to inject the language highlighting into. Without the `@injection.content` capture, no highlighting occurs.

## Solution

Added `(code_line) @injection.content` to all `fenced_code_block` injection patterns:

```scheme
((fenced_code_block
  info: (info_string) @_lang
  (#eq? @_lang "python")
  (code_line) @injection.content)
 (#set! injection.language "python"))
```

This captures the code content lines and enables syntax highlighting.

## Languages Fixed

All fenced code block injections were updated:
- Python
- R
- Julia
- JavaScript/TypeScript
- Bash/Shell
- SQL
- JSON
- YAML
- TOML
- HTML
- CSS
- Markdown

## Testing Instructions

1. **Restart Zed completely** (to clear grammar cache)
2. **Install dev extension**:
   - Cmd+Shift+P → "zed: install dev extension"
   - Select this directory
3. **Test with a .qmd file** containing code blocks:

```markdown
# Test File

Standard Markdown style:

\`\`\`python
import numpy as np
print("Hello")
\`\`\`

Quarto executable style (these should already work):

\`\`\`{python}
x = 42
print(x)
\`\`\`
```

4. **Expected result**: Both code block styles should have Python syntax highlighting

## Upstream Impact

The same bug exists in the upstream `tree-sitter-quarto` grammar at commit `c2c28fd`. After confirming this fix works in Zed, we should:

1. Submit a PR to fix `grammars/quarto/queries/injections.scm`
2. Update the extension to use the fixed commit
3. Document in issue #6: https://github.com/ck37/tree-sitter-quarto/issues/6

## Related Files

- `languages/quarto/injections.scm`: Extension's injection queries (now fixed)
- `grammars/quarto/queries/injections.scm`: Upstream grammar queries (needs same fix)
