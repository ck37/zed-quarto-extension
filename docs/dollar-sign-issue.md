# Dollar Sign Syntax Highlighting Issue

**Date**: 2025-10-21  
**Status**: Identified - Grammar Issue  
**Test**: `tests/dollar_sign_highlighting.rs`

## Problem

Dollar signs in regular text (like "$160" or "$25-30") are incorrectly parsed as LaTeX math delimiters by the tree-sitter-quarto grammar, causing syntax highlighting issues in Zed.

## Example

```markdown
The Keychron Q8 keyboard ($160) and wrist pads ($10) are ordered.
```

## Root Cause: Grammar Issue

The test results clearly show this is a **tree-sitter-quarto grammar bug**, not an issue with this extension's configuration.

### Evidence from Test Output

1. **Text with dollar signs produces ERROR nodes:**
   ```
   (document (ERROR (text) (math_delimiter) (math_content)))
   ```

2. **Dollar signs in parentheses are parsed as incomplete math:**
   - `($160)` becomes `(inline_math (math_delimiter) (math_content) (math_delimiter))`
   - The grammar treats `$1` as opening math, `6` as content, and `0` as closing delimiter
   - This creates unpaired math contexts and ERROR nodes

3. **Expected behavior:**
   - Dollar signs in regular prose should be treated as plain text
   - Only explicit math contexts should trigger math parsing
   - In Pandoc/Quarto, single dollar signs are only math when they:
     - Are not adjacent to spaces on the inside: `$x$` is math, `$ x$` is not
     - Don't have numbers immediately after: `$160` should be text

## Test Results

### Failed Tests (Grammar Issues)
- ❌ `dollar_signs_in_regular_text` - Text like "$160" creates ERROR nodes
- ❌ `dollar_signs_in_various_contexts` - All plain dollar amounts fail

### Passed Tests  
- ✅ `inline_math_with_dollar_signs` - Actual math like `$x^2 + y^2 = z^2$` works
- ✅ `display_math_with_double_dollar_signs` - Display math `$$E = mc^2$$` works  
- ✅ `mixed_dollar_signs_and_math` - Mixed context works

## Parse Tree Analysis

### Problem Case: `"Simple amount: $50"`
```
(document (ERROR (text) (math_delimiter) (math_content)))
```
The `$5` is treated as opening math delimiter + content, and `0` is orphaned, creating an ERROR node.

### Problem Case: `"The keyboard ($160) costs more"`
```
(document (ERROR 
  (text) 
  (inline_math (math_delimiter) (math_content) (math_delimiter)) 
  (text) 
  (inline_math (math_delimiter) (math_content) (math_delimiter)) 
  (text) 
  (math_delimiter) 
  (math_content)))
```
Multiple spurious `inline_math` nodes are created, with unpaired delimiters causing ERROR nodes.

### Working Case: `"Display math: $$E = mc^2$$"`
```
(document 
  (paragraph content: (inline (text))) 
  (display_math 
    open: (math_delimiter) 
    content: (math_content) 
    close: (math_delimiter)))
```
Double dollar signs correctly create a `display_math` node.

## Impact on Syntax Highlighting

When the grammar produces ERROR nodes:
- Zed's syntax highlighting breaks down at those locations
- Text after the error may not highlight correctly
- Users see visual artifacts or incorrect colors

## Solution

This needs to be fixed in the **tree-sitter-quarto grammar**, specifically in how it handles inline math detection. The grammar should follow Pandoc's rules for math delimiters:

1. Single `$` followed by a digit (like `$160`) should be text, not math
2. Math delimiters must not have spaces adjacent to their inside: `$x$` is math, `$ x$` is not
3. Consider requiring backslash escaping for literal dollar signs in ambiguous contexts

## References

- **Pandoc Manual**: https://pandoc.org/MANUAL.html#math
- **tree-sitter-quarto**: https://github.com/ck37/tree-sitter-quarto
- **Current commit**: `4012bc7d9930654c81f1ade1d2070e0b951aa689`

## Issue Tracking

**GitHub Issue**: https://github.com/ck37/tree-sitter-quarto/issues/9

## Next Steps

1. ✅ ~~File issue in tree-sitter-quarto repository~~ - Filed as issue #9
2. Implement fix in grammar's inline math scanner
3. Update extension to new grammar commit once fixed
4. Verify tests pass

## Workaround

Until the grammar is fixed, users can:
- Use LaTeX escaping: `\$160` 
- Use HTML entities: `&#36;160`
- Wait for grammar update (recommended)
