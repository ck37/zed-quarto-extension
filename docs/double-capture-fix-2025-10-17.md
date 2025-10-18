# Double-Capture Bug Fix - October 17, 2025

## Problem Discovered

While running automated tests for bold/italic highlighting, discovered that text nodes inside emphasis/strong were being captured **twice**:

```
@emphasis.strong           "bold" (text)
@text                      "bold" (text)  ← DUPLICATE!
```

This double-capture occurred because:
1. Specific pattern captured it: `(emphasis (text) @emphasis)`
2. Catch-all pattern also captured it: `(text) @text`

## Root Cause

The highlights.scm file had a catch-all pattern at line 279:

```scm
(text) @text
```

This pattern matched **all** text nodes, including those already captured by more specific patterns like:
- `(emphasis (text) @emphasis)`
- `(strong_emphasis (text) @emphasis.strong)`
- `(link_text) @link_text`

Tree-sitter allows multiple captures on the same node, but this can confuse editors like Zed when determining which highlighting to apply.

## Solution

**Removed the catch-all `(text) @text` pattern** from highlights.scm (line 279).

### Rationale:

1. **Specific patterns are sufficient**: Text nodes inside emphasis, strong, links, code spans, etc. are already captured by their specific patterns
2. **Plain text doesn't need highlighting**: Unformatted text can remain unstyled
3. **Eliminates ambiguity**: Each node now has exactly one highlight capture, making the highlighting behavior deterministic
4. **Common pattern in other grammars**: Many tree-sitter grammars don't have catch-all text patterns

### After Fix:

```
@punctuation.delimiter     "**" (strong_emphasis_delimiter)
@emphasis.strong           "bold" (text)      ← Only one capture!
@punctuation.delimiter     "**" (strong_emphasis_delimiter)
@punctuation.delimiter     "*" (emphasis_delimiter)
@emphasis                  "italic" (text)    ← Only one capture!
@punctuation.delimiter     "*" (emphasis_delimiter)
```

Plain text (like "Test", "and", etc.) is no longer captured, which is expected and correct.

## Testing

### Before Fix:
- 4 out of 5 bold_italic_highlighting tests **failed** due to incorrect scope name expectations (test issue, not query issue)
- Tests showed double-captures

### After Fixes:
1. Updated test expectations to match current scope names (removed `.markup` suffix)
2. Removed catch-all `(text) @text` pattern
3. **All 72 tests pass** ✅

### Test Command:
```bash
cargo test --workspace --all-features
```

### Result:
```
72 tests passed
1 test ignored (emphasis_variations - requires Zed injection system)
0 tests failed
```

## Impact on Zed Highlighting

### Hypothesis:
The double-capture issue **may have been preventing Zed from applying highlights correctly**. When a node has multiple highlight scopes, Zed might:
1. Skip the node entirely (conflicting instructions)
2. Apply the wrong scope (e.g., generic `@text` instead of specific `@emphasis`)
3. Apply neither scope

### Next Steps:
1. Reinstall the extension with the fix:
   ```bash
   ./install-dev.sh
   ```

2. Test in Zed editor with a .qmd file containing:
   ```markdown
   This is **bold** and *italic* text with a [link](https://example.com).
   ```

3. Verify that bold/italic highlighting now works correctly

## Files Modified

1. **languages/quarto/highlights.scm**:
   - Removed `(text) @text` catch-all pattern (line 279)
   - Added comment explaining the removal

2. **tests/bold_italic_highlighting_test.rs**:
   - Updated scope name expectations:
     - `emphasis.strong.markup` → `emphasis.strong`
     - `emphasis.markup` → `emphasis`
     - `link_text.markup` → `link_text`
     - `link_uri.markup` → `link_uri`
   - These changes align tests with current highlights.scm scope names

## Related Issues

- Scope naming was updated in previous commits to use Zed-compatible scopes (removed `.markup` suffix)
- Tests were not updated at that time, leading to false failures
- The double-capture issue was hidden by the test failures

## Investigation Timeline

1. **Started**: Resume from investigation-notes-2025-10-17.md
2. **Discovered**: Extensive automated test suite exists for highlighting
3. **Found**: Tests were failing due to outdated scope name expectations
4. **Revealed**: Diagnostic output showed double-captures on text nodes
5. **Root Cause**: Catch-all `(text) @text` pattern conflicting with specific patterns
6. **Fixed**: Removed catch-all pattern
7. **Validated**: All 72 tests pass

## Lessons Learned

1. **Always run existing tests first**: The automated tests immediately revealed the double-capture issue
2. **Keep tests in sync with queries**: When scope names change, update tests immediately
3. **Diagnostic test output is invaluable**: The `diagnostic_print_all_captures` test made the issue obvious
4. **Catch-all patterns can cause problems**: Be cautious with broad patterns that overlap with specific ones
5. **Trust the test suite**: The tests caught an issue that manual inspection might have missed
