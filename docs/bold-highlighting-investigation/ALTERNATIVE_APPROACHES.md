# Alternative Approaches for Bold Highlighting

## Problem Summary

After 7 failed test attempts and extensive research, we've determined:

❌ **Grammar-to-grammar injection doesn't work in Zed extensions**
- Tried multiple configurations
- Researched all popular extensions
- Found Issue #484: injection problems with extension grammars
- Zed's built-in markdown uses `"markdown-inline"` (bundled with Zed)
- Extensions can only inject to **built-in** languages, not custom grammars

## Research Evidence

1. **No extension found** doing custom-to-custom grammar injection
2. **Vue/Svelte** - inject built-in languages only (JS, CSS, HTML)
3. **PHP** - injects `phpdoc` but it might be built-in or special-cased
4. **Issue #484** - Open bug about injection limitations
5. **Zed's markdown** - Uses bundled `markdown-inline`, not extension-defined

See `EXTENSION_RESEARCH.md` and `BOLD_HIGHLIGHTING_TEST_LOG.md` for full details.

---

## Alternative Approach 1: Merged Grammar Highlights (Recommended)

### Strategy
Merge both grammars' highlight queries into the block grammar's highlights.scm. Don't use injection.

### Implementation

**Step 1**: Remove inline grammar entirely
```bash
# In extension.toml - remove this section:
[grammars.pandoc_markdown_inline]
repository = "https://github.com/ck37/tree-sitter-pandoc-markdown"
commit = "f9d68613baef187daffcfbd4af09b5eab1005d38"
path = "tree-sitter-pandoc-markdown-inline"
```

**Step 2**: Remove injection
```bash
# Remove from languages/quarto/injections.scm:
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))
```

**Step 3**: Delete inline language directory
```bash
rm -rf languages/pandoc_markdown_inline/
```

**Step 4**: Verify block grammar has inline highlights
The upstream block grammar already includes them (we verified this):
```scheme
(emphasis) @text.emphasis
(strong_emphasis) @text.strong
(code_span) @text.literal
# etc.
```

### Pros
✅ Simple - just remove things
✅ No injection needed
✅ Upstream grammar already has inline highlights in block highlights.scm
✅ Should work immediately

### Cons
❌ Less precise than dual-grammar parsing
❌ May miss some edge cases in complex Pandoc syntax
❌ Not following upstream's intended dual-grammar architecture

### Testing
```bash
./install-dev.sh
# Restart Zed
# Install extension
# Test bold in test-bold.qmd
```

---

## Alternative Approach 2: Ask Zed Team

### Strategy
File an issue asking how to properly inject custom grammars.

### Issue Title
"How to inject custom grammar into another custom grammar in extensions?"

### Issue Content
```markdown
## Problem
I'm developing a Zed extension for Quarto (Pandoc Markdown) that uses a dual-grammar architecture:
- Block grammar: parses document structure
- Inline grammar: parses inline elements (bold, italic, links)

The inline grammar needs to be injected into `(inline)` nodes created by the block grammar.

## What I've Tried
1. Defined both grammars in `extension.toml`
2. Created `languages/pandoc_markdown_inline/` with `config.toml`
3. Added injection in `languages/quarto/injections.scm`:
   ```scheme
   ((inline) @injection.content
    (#set! injection.language "pandoc_markdown_inline"))
   ```

## Result
- Extension installs successfully
- Both grammars compile
- But injection doesn't work - no bold highlighting

## Research
- Reviewed all popular extensions - none do custom-to-custom grammar injection
- Vue/Svelte only inject built-in languages (JS, CSS)
- Found Issue #484 about injection limitations
- Zed's markdown uses `"markdown-inline"` (bundled with Zed, not extension-defined)

## Questions
1. Can extensions inject custom grammars into other custom grammars?
2. Is this a built-in-only feature?
3. If it should work, what am I doing wrong?

## Repository
https://github.com/ck37/zed-quarto-extension
```

### Pros
✅ Get authoritative answer from Zed team
✅ May uncover hidden feature or proper syntax
✅ Could benefit other extension developers

### Cons
❌ Takes time to get response
❌ May confirm it's not supported
❌ Users waiting for fix

---

## Alternative Approach 3: Modify Upstream Grammar

### Strategy
Modify tree-sitter-pandoc-markdown to work as single grammar.

### Implementation
Fork the grammar and combine both grammars into one that doesn't require injection.

### Pros
✅ Proper long-term solution
✅ Could contribute back upstream

### Cons
❌ Complex - requires tree-sitter grammar expertise
❌ Maintains fork of upstream grammar
❌ Time-consuming

---

## Alternative Approach 4: Wait for Zed Fix

### Strategy
Monitor Issue #484 and wait for Zed to fix injection support.

### Pros
✅ Proper fix from Zed team
✅ Will work as intended eventually

### Cons
❌ Unknown timeline
❌ May never be fixed
❌ Users have no workaround

---

## Alternative Approach 5: Use Built-in Markdown as Base

### Strategy
Extend Zed's built-in markdown instead of creating new extension.

### Implementation
Since markdown-inline works in built-in markdown, use markdown as base and add Quarto-specific features on top.

### Pros
✅ Leverage working injection
✅ Inherit all markdown features

### Cons
❌ Can't replace markdown grammar with pandoc-markdown
❌ Limited customization
❌ May conflict with built-in markdown
❌ Not clear if this is even possible

---

## Recommendation: Approach 1 (Merged Highlights)

**Rationale**:
1. Quick fix - can be implemented immediately
2. Upstream block grammar already has inline highlights
3. Will provide bold/italic highlighting (the main user need)
4. Can revisit if Zed adds proper injection support later

**Implementation**:
```bash
# 1. Remove inline grammar from extension.toml
# 2. Remove injection from injections.scm
# 3. Delete languages/pandoc_markdown_inline/
# 4. Test
```

**If this doesn't work**:
- The block grammar might not parse inline nodes at all
- In that case, we need Approach 2 (ask Zed team) or Approach 3 (modify upstream grammar)

---

## Next Steps

1. Try Approach 1 first (simplest)
2. If that fails, file issue with Zed (Approach 2)
3. Meanwhile, research if modifying upstream grammar is feasible (Approach 3)

**User expectation management**: Explain that bold highlighting requires workaround due to Zed limitation.
