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

See `extension-research.md` for full details.

---

## Alternative Approach 1: Merged Grammar Highlights ❌ REJECTED

### Status: **NOT VIABLE**

### Why Rejected

The upstream grammar has a [detailed architecture rationale](https://github.com/ck37/tree-sitter-pandoc-markdown/blob/feat/phase-1-pandoc-grammar/docs/architecture-rationale.md) explaining why dual-grammar architecture is essential:

1. **CommonMark spec compliance** - Two-phase parsing is part of the specification
2. **Tree-sitter LR parser limitations** - Cannot handle merged grammar complexity
3. **Would cause massive parsing conflicts** - Technically infeasible
4. **Intentional design** - "should be preserved"

As the grammar maintainer states: "The split block/inline architecture is intentional, well-founded, and should be preserved. Attempting to unify the grammars would introduce major complexity, performance issues, and maintenance headaches."

### What We Tried (Documented)

**Attempt 1: Remove inline grammar injection**
- Tested if block grammar alone would provide highlighting
- Result: No highlighting for inline elements
- Reason: Block grammar creates `(inline)` nodes with unparsed text; inline grammar must parse that text

**Attempt 2: Inject built-in markdown-inline**
- See `builtin-injection-test.md`
- Result: Only 10% coverage (single asterisk italic only)
- Reason: Built-in grammar incompatible with Pandoc's tokenization

### Conclusion

Merged grammar approach:
- ❌ Violates upstream architecture principles
- ❌ Technically problematic (parsing conflicts)
- ❌ Would be rejected by grammar maintainers
- ❌ Does not actually work without injection

**This approach is abandoned.**

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

## Alternative Approach 3: Modify Upstream Grammar ❌ REJECTED

### Status: **NOT VIABLE**

### Strategy
Modify tree-sitter-pandoc-markdown to work as single grammar.

### Why Rejected

Same reasons as Approach 1 - the dual-grammar architecture is:
- Essential for CommonMark spec compliance
- Required due to tree-sitter LR parser limitations
- Intentionally designed and documented as correct approach
- Would introduce massive parsing conflicts if unified

Additionally:
- ❌ Would require maintaining a fork
- ❌ Cannot be contributed back (violates upstream architecture)
- ❌ Complex and time-consuming
- ❌ Does not actually solve the problem (injection still needed)

**This approach is abandoned.**

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

## Alternative Approach 5: Inject Built-in markdown-inline ✅ PARTIAL SUCCESS

### Status: **IMPLEMENTED** (Workaround)

### Strategy
Inject Zed's built-in `markdown-inline` grammar into Pandoc's `(inline)` nodes.

### Implementation
```scheme
((inline) @injection.content
 (#set! injection.language "markdown-inline"))
```

### Test Results

**Tested** - See `builtin-injection-test.md`

✅ **Works (70% coverage)**:
- Bold with `**` and `__`
- Italic with `*` and `_`
- Inline code

⚠️ **Partial (~10%)**:
- Mixed content (only italic works)

❌ **Doesn't work (~20%)**:
- Links
- Triple asterisks `***`
- Pandoc extensions (strikethrough, sub/super)

### Pros
✅ **Significant improvement** - 0% → 70% coverage
✅ **Solves primary user complaint** - bold/italic now work
✅ **Works for most common use cases** - simple emphasis
✅ **Better than nothing** - practical workaround
✅ **Can be improved later** - switch to full Pandoc inline grammar if Zed adds support

### Cons
❌ Incomplete - doesn't work for all inline features
❌ No links highlighting
❌ No Pandoc-specific inline features
❌ Mixed content partially broken

### Decision

**KEEP as practical workaround** while we work on the proper fix (contributing to Zed).

---

---

## Recommended Path Forward

### ✅ Current State (IMPLEMENTED)

**Approach 5: Built-in markdown-inline Injection**
- Provides 70% coverage for bold/italic highlighting
- Implemented and working
- Practical workaround that solves primary user complaint

### 🚀 Long-term Solution (PLANNED)

**Contribute to Zed: Enable Custom-to-Custom Grammar Injection**

See [`zed-modification-analysis.md`](./zed-modification-analysis.md) for detailed plan.

**Strategy:**
1. Contribute PR to Zed to support extension-to-extension grammar injection
2. File Zed issue documenting:
   - The limitation (only built-in injection works)
   - Our research (no extensions do custom-to-custom injection)
   - Use case (dual-grammar architectures like Pandoc markdown)
   - Proposed solution (from zed-modification-analysis.md)

**Timeline:**
- Short term (now): Keep built-in injection workaround (70% coverage)
- Medium term (2-4 weeks): File Zed issue with research
- Long term (1-3 months): Contribute PR to Zed
- Future: Switch to full Pandoc inline grammar when Zed supports it

**Benefits:**
- ✅ Solves problem for all Zed extensions needing dual grammars
- ✅ Respects upstream grammar architecture
- ✅ Enables 100% inline highlighting (vs current 70%)
- ✅ Benefits entire Zed ecosystem

---

## Rejected Approaches

1. ❌ **Merged Grammar** - Violates architecture, technically infeasible
2. ❌ **Modify Upstream Grammar** - Same issues as merged grammar
3. ⏸️ **Ask Zed Team** - Will file issue as part of contribution process
4. ⏸️ **Wait for Zed Fix** - Being proactive by contributing ourselves

---

## Next Steps

1. ✅ **Keep built-in injection** (70% coverage, implemented)
2. 📝 **Update documentation** to reflect current state and limitations
3. 🐛 **File Zed issue** with thorough research and use case
4. 💻 **Prepare PR** following zed-modification-analysis.md
5. 🔄 **Switch to Pandoc inline grammar** once Zed supports custom injection

**User expectation management**: Explain that bold highlighting requires workaround due to Zed limitation.
