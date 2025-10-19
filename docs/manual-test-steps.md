# Manual Testing Steps for Bold/Italic Highlighting

## Current Status

Extension has been cleaned up and is ready for testing. The intentional errors have been removed from `languages/quarto/highlights.scm`.

## Prerequisites

1. Reinstall the extension:
   ```bash
   ./install-dev.sh
   ```

2. Restart Zed or reload extensions (`Cmd+Shift+P` → `zed: reload extensions`)

## Test Files

Create two test files with identical content:

**test.qmd:**
```markdown
# Test Document

This is **bold** and this is *italic*.

Here is a [link](https://example.com).

- List item with **bold**
- List item with *italic*
```

**test.md:**
```markdown
# Test Document

This is **bold** and this is *italic*.

Here is a [link](https://example.com).

- List item with **bold**
- List item with *italic*
```

## Debug Steps

### Step 1: Visual Inspection

1. Open `test.qmd` in Zed
2. Check if bold and italic are highlighted
3. Open `test.md` in Zed
4. Compare highlighting between the two files

**Expected:** Both should have identical highlighting
**Actual:** Document what you see

### Step 2: Parse Tree Inspection

1. Open `test.qmd`
2. `Cmd+Shift+P` → `debug: open syntax tree view`
3. Look for these nodes:
   - `strong_emphasis` containing `text` for `**bold**`
   - `emphasis` containing `text` for `*italic*`
   - `link` with `link_text` and `link_destination`

**Screenshot or paste the tree structure here**

### Step 3: Highlight Captures (Quarto)

1. With `test.qmd` open
2. `Cmd+Shift+P` → `editor: copy highlight json`
3. Paste output into `/tmp/qmd-highlights.json`
4. Save the content

### Step 4: Highlight Captures (Markdown)

1. With `test.md` open
2. `Cmd+Shift+P` → `editor: copy highlight json`
3. Paste output into `/tmp/md-highlights.json`
4. Save the content

### Step 5: Compare Captures

```bash
# Compare the two files
diff /tmp/qmd-highlights.json /tmp/md-highlights.json
```

**Look for:**
- Are there `@emphasis` or `@emphasis.strong` captures in qmd file?
- How do they differ from the md file?
- Are the scope names identical or different?

### Step 6: Check Zed Logs

```bash
# View recent Zed logs
tail -100 ~/Library/Logs/Zed/Zed.log

# Search for extension errors
grep -i "quarto\|error\|warning" ~/Library/Logs/Zed/Zed.log | tail -50
```

**Look for:**
- Query compilation errors
- Grammar loading errors
- Extension initialization errors

## Results Template

### Visual Test Results
- [ ] Bold highlighting works in .qmd: YES / NO
- [ ] Italic highlighting works in .qmd: YES / NO
- [ ] Link highlighting works in .qmd: YES / NO
- [ ] Highlighting identical to .md: YES / NO

### Parse Tree Results
```
Paste tree structure here
```

### Highlight JSON Results

**QMD Captures:**
```json
Paste relevant captures here
```

**MD Captures:**
```json
Paste relevant captures here
```

### Log Errors
```
Paste any errors here
```

## Analysis Questions

After gathering data:

1. **Does the grammar parse correctly?**
   - YES/NO based on syntax tree view

2. **Are captures being made?**
   - YES/NO based on highlight JSON

3. **Do scope names match theme?**
   - Compare captured scopes with theme definitions

4. **Are there any errors in logs?**
   - YES/NO and paste errors if any

5. **How does .qmd differ from .md?**
   - Describe differences in captures or tree structure

## Next Steps Based on Results

### If captures are missing:
- Query patterns may not be matching
- Grammar may not be loaded
- Extension may not be active

### If captures exist but no highlighting:
- Scope names may not match theme
- Theme may not support these scopes
- HighlightMap may not be matching

### If errors in logs:
- Fix errors first
- Reload extension
- Retest

## Files Modified

- Cleaned up: `languages/quarto/highlights.scm` (removed intentional errors)
- This document: `docs/manual-test-steps.md`
