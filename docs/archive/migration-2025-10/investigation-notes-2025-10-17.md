# Syntax Highlighting Investigation - October 17, 2025

## Problem Statement

Bold/italic syntax highlighting not working in .qmd files in Zed, despite:
- Extension loading successfully
- Grammar parsing correctly
- Queries compiling without errors
- Tests passing

## Evidence-Based Findings

### 1. How Zed's Highlighting Works (Source Code Analysis)

**File:** `crates/language/src/highlight_map.rs`

**Scope Matching Algorithm:**
```rust
for key_part in key.split('.') {
    if capture_parts.clone().any(|part| part == key_part) {
        len += 1;
    } else {
        return None;
    }
}
```

**Key Discovery:**
- Zed splits both theme keys and capture names on dots
- Checks if ALL theme key parts exist in capture name parts
- Order-independent matching
- Selects longest match

**Examples:**
- `@emphasis.markup` WILL match theme key `emphasis` ✅
- `@emphasis` WILL match theme key `emphasis` ✅
- Both work equally well

### 2. Query Loading (Source Code Analysis)

**File:** `crates/extension_host/src/extension_host.rs`

**Function:** `load_plugin_queries(&language_path)`

**Path Resolution:**
```
extension_dir/languages/<lang>/*.scm
```

**For our extension:**
```
~/Library/Application Support/Zed/extensions/installed/quarto/languages/quarto/highlights.scm
→ symlinks to project directory
```

**Confirmed:** Zed loads queries from extension directory, NOT from grammar repository.

### 3. Current Extension State

**Scope Names Used:**
- `@emphasis` (italic)
- `@emphasis.strong` (bold)
- `@link_text` (link text)
- `@link_uri` (link URL)
- `@punctuation.delimiter` (delimiters)

**Theme Scopes Available (One Dark):**
- `emphasis` ✅
- `emphasis.strong` ✅
- `link_text` ✅
- `link_uri` ✅
- `punctuation.delimiter` ✅

**All scope names match theme exactly.**

### 4. Query Patterns

**Current (after fixes):**
```scm
(emphasis
  (text) @emphasis)

(strong_emphasis
  (text) @emphasis.strong)
```

**Rationale:** Highlights the text child node, not the entire emphasis node.

### 5. Zed's Built-in Markdown Comparison

**Architecture:**
- Uses TWO grammars: `markdown` (block) + `markdown-inline` (inline)
- Injects inline grammar: `((inline) @injection.content (#set! injection.language "markdown-inline"))`
- Inline grammar uses `.markup` suffix: `@emphasis.markup`, `@link_text.markup`

**Our Architecture:**
- Uses ONE unified grammar: `tree-sitter-quarto`
- No injection needed (grammar handles both block and inline)
- Uses base scope names: `@emphasis`, `@link_text`

### 6. Testing & Validation

**What We've Verified:**
- ✅ Grammar parses bold/italic correctly (`tree-sitter parse`)
- ✅ Queries capture correctly (`tree-sitter query`)
- ✅ All 72 automated tests pass
- ✅ Extension loads without errors in Zed logs
- ✅ Scope names match theme definitions
- ✅ Matching algorithm would match our scopes

**What's NOT Working:**
- ❌ Actual highlighting in Zed editor

### 7. Zed Testing Infrastructure

**Finding:** Zed has NO automated tests for syntax highlighting.

**File Checked:** `crates/language/src/buffer_tests.rs`

**What Zed Tests:**
- Tree-sitter query execution (outlines, text objects)
- Parse tree correctness
- Language scope detection
- Bracket matching

**What Zed Does NOT Test:**
- Visual highlighting
- Color application
- Scope-to-theme mapping
- `HighlightStyle` rendering

**Implication:** Manual debugging required.

## Available Debug Tools

### Zed Debug Commands

1. **`debug: open syntax tree view`**
   - Shows tree-sitter parse tree
   - Verify grammar parsing

2. **`editor: copy highlight json`**
   - Dumps all highlight captures to clipboard
   - Shows what Zed is actually capturing

3. **Command Palette:** `Cmd+Shift+P` (Mac)

### Debug Steps to Try

1. Open .qmd file with `**bold**` and `*italic*`
2. Run `debug: open syntax tree view`
   - Verify `emphasis` and `strong_emphasis` nodes exist
3. Run `editor: copy highlight json`
   - Check if emphasis captures appear in JSON
4. Compare with .md file (which works)
   - Look for differences in captures

## Files Modified During Investigation

### Changes Made:
1. **`languages/quarto/highlights.scm`**
   - Fixed query pattern: highlight `(text)` child instead of whole node
   - Fixed scope names: removed `@text.` prefix from emphasis/links
   - Current: `@emphasis`, `@emphasis.strong`, `@link_text`, `@link_uri`

2. **`tests/bold_italic_highlighting_test.rs`**
   - Updated to match new scope names
   - All tests passing

3. **`docs/bold-italic-highlighting-debugging.md`**
   - Documented investigation findings
   - Added solution sections

4. **`docs/investigation-notes-2025-10-17.md`** (this file)
   - Comprehensive evidence-based findings

## Current Status: Unresolved

**Known Good:**
- Grammar parses correctly
- Queries compile successfully
- Scope names match theme
- Extension loads without errors
- Tests pass

**Unknown:**
- Why highlighting doesn't work in Zed runtime
- Whether queries are being executed
- Whether captures are being made
- Whether theme matching is working

## Next Steps (TODO)

### High Priority
- [ ] Test with Zed debug commands (`debug: open syntax tree view`, `editor: copy highlight json`)
- [ ] Add intentional errors to verify Zed error reporting (already added to highlights.scm)
- [ ] Reload extension in Zed and check logs for errors
- [ ] Compare highlight JSON between .qmd and .md files
- [ ] Check if captures are being made at all

### Medium Priority
- [ ] Search for Zed developer mode or verbose logging
- [ ] Look for environment variables to enable debug output
- [ ] Check if there's a way to inspect HighlightMap in Zed

### Low Priority
- [ ] File issue with Zed if this is a bug
- [ ] Test with different themes
- [ ] Try minimal reproduction case

## Intentional Errors Added (For Testing)

Added to end of `languages/quarto/highlights.scm`:
1. Invalid node type: `(invalid_node_that_does_not_exist) @test.error1`
2. Missing @ symbol: `(text) test.error2`
3. Unclosed parenthesis: Missing `)`
4. Invalid capture name: `@test/error/4`

**Purpose:** See which errors Zed reports in logs.

**To Clean Up:**
```bash
# Remove last 14 lines
head -n -14 languages/quarto/highlights.scm > languages/quarto/highlights.scm.tmp
mv languages/quarto/highlights.scm.tmp languages/quarto/highlights.scm
```

## Key References

### Zed Source Code
- `crates/language/src/highlight_map.rs` - Scope matching algorithm
- `crates/extension_host/src/extension_host.rs` - Query loading
- `crates/language/src/buffer.rs` - Highlight application
- `crates/languages/src/markdown-inline/highlights.scm` - Working example

### Issues & Discussions
- [#20525](https://github.com/zed-industries/zed/issues/20525) - Debug command for viewing highlights

### Documentation
- [Language Extensions](https://zed.dev/docs/extensions/languages)
- [Syntax-Aware Editing](https://zed.dev/blog/syntax-aware-editing)

## Summary

We've done extensive evidence-based investigation and confirmed:
1. Our queries are correct
2. Our scope names match the theme
3. The grammar parses correctly
4. The extension loads successfully

**The problem must be occurring at runtime in Zed, and can only be debugged using Zed's debug tools or by examining logs during actual extension execution.**
