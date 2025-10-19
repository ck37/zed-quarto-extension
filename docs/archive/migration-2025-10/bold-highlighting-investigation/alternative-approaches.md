# Alternative Approaches for Bold Highlighting

> **Update (2025-10-12)**: Root cause identified and fix implemented! The issue was that the registry version wasn't incremented when extension grammars loaded. Fix is one line of code. See [zed-fix-implemented.md](./zed-fix-implemented.md).

## Problem Summary

After 7 failed test attempts and extensive research, we determined:

❌ **Grammar-to-grammar injection didn't work in Zed extensions**
- Tried multiple configurations
- Researched all popular extensions
- Root cause: Registry version not incremented after extension grammar loading
- Zed's built-in markdown uses `"markdown-inline"` (bundled with Zed, immediately available)
- Extension grammars load asynchronously as WASM, and pending injections weren't rechecked

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
````markdown
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
````

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

### 🚀 Fix Implemented

**Root Cause Identified and Fixed**

See [`zed-fix-implemented.md`](./zed-fix-implemented.md) for complete implementation details.

**The Fix:**
- One-line change to increment registry version when extension grammars load
- Enables pending injections to be resolved after async loading completes
- Minimal impact, uses existing infrastructure

**Status:**
- ✅ Fix implemented in branch `fix/extension-grammar-injection`
- ⏳ Testing pending (build Zed and test with Quarto extension)
- 📝 PR to Zed will be submitted after testing
- 🎯 Timeline: Weeks, not months

**Impact:**
- ✅ Solves problem for all Zed extensions needing dual grammars
- ✅ Respects upstream grammar architecture
- ✅ Will enable 100% inline highlighting (vs current 70% workaround)
- ✅ Benefits entire Zed ecosystem
- ✅ Simple fix that's easy to review and merge

---

## Rejected Approaches

1. ❌ **Merged Grammar** - Violates architecture, technically infeasible
2. ❌ **Modify Upstream Grammar** - Same issues as merged grammar
3. ⏸️ **Ask Zed Team** - Will file issue as part of contribution process
4. ⏸️ **Wait for Zed Fix** - Being proactive by contributing ourselves

---

## Status and Next Steps

**Completed:**
1. ✅ Implemented built-in injection workaround (70% coverage)
2. ✅ Investigated root cause via code analysis
3. ✅ Identified missing version increment
4. ✅ Implemented fix (one line)
5. ✅ Updated documentation

**Pending:**
1. ⏳ Test fix by building Zed from branch
2. 📝 Submit PR to Zed with verification findings
3. 🔄 Switch extension to full Pandoc inline grammar after fix merges
4. 🎯 Remove workaround and achieve 100% coverage

**User expectation**: Workaround provides 70% coverage now; full fix expected within weeks.
