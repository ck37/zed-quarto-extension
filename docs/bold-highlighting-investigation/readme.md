# Bold Highlighting Investigation

Investigation into why bold/italic text highlighting wasn't working in Quarto `.qmd` files, and the workaround implemented.

## Summary

**Problem**: Bold (`**text**`) and italic (`*text*`) weren't highlighted in `.qmd` files.

**Root Cause (Confirmed)**: When extension grammars load asynchronously, the LanguageRegistry version wasn't incremented. This prevented SyntaxMap from rechecking pending injections after the grammar loaded. Built-in grammars worked because they're immediately available (no async loading needed).

**Current Solution (70% coverage)**: Inject Zed's built-in `markdown-inline` grammar as temporary workaround.

**Fix Implemented**: One-line change in Zed to increment registry version when languages load. Fix is in branch `fix/extension-grammar-injection`, pending testing and PR.

**Status**: Workaround active now; full solution expected within weeks after Zed PR review.

## Technical Background

### Dual Grammar Architecture

The pandoc-markdown grammar uses two grammars:
- **Block grammar** (`tree-sitter-pandoc-markdown`): Parses document structure, creates `(inline)` nodes
- **Inline grammar** (`tree-sitter-pandoc-markdown-inline`): Parses inline content like bold, italic, links

The inline grammar must be injected into `(inline)` nodes for emphasis highlighting to work.

### The Root Cause

**Zed's built-in markdown** (works):
```scheme
((inline) @injection.content
 (#set! injection.language "markdown-inline"))
```
✅ Works because `"markdown-inline"` is immediately available (compiled into Zed binary)

**Our extension** (didn't work):
```scheme
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))
```
❌ Failed because `"pandoc_markdown_inline"` loads asynchronously as WASM, and the registry version wasn't incremented after loading

### The Investigation

Research confirmed the issue:
1. **Extension grammars ARE registered** in the same language registry as built-ins
2. **Loading IS triggered** when injection is first encountered
3. **Missing piece**: Registry version wasn't incremented when loading completed
4. **Result**: SyntaxMap never rechecked pending injections after grammar loaded

See [verification-findings.md](./verification-findings.md) for complete code analysis and [extension-research.md](./extension-research.md) for testing history.

## Current Solution: Built-in markdown-inline Injection

**Implementation**: Inject Zed's built-in `markdown-inline` grammar into Pandoc's `(inline)` nodes.

**Coverage**: ~70% of inline formatting works

✅ **Works**:
- Bold with `**` and `__`
- Italic with `*` and `_`
- Inline code

❌ **Doesn't work**:
- Links
- Mixed content (partially)
- Pandoc extensions (strikethrough, subscript, superscript)

See [builtin-injection-test.md](./builtin-injection-test.md) for detailed test results.

### Why This Is Acceptable

1. **Significant improvement**: 0% → 70% coverage
2. **Solves primary user complaint**: Bold and italic now work
3. **Works for most common use cases**: Simple emphasis
4. **Can be improved later**: Switch to full Pandoc inline grammar once Zed adds support

## The Fix: Registry Version Increment

**Implementation**: One-line change in `crates/language/src/language_registry.rs`:

```rust
state.version += 1;  // Increment version so pending injections can be resolved
```

**How it works**:
1. Extension grammar not loaded → injection marked as "pending"
2. Background task loads grammar asynchronously
3. **NEW**: Version increments when loading completes
4. SyntaxMap detects version change → rechecks pending injections
5. Grammar now available → injection resolved ✅

**Status**:
- ✅ **Fix implemented** in branch `fix/extension-grammar-injection`
- ⏳ **Testing**: Pending build and test with Quarto extension
- 📝 **PR to Zed**: Will submit after testing confirms fix works
- 🎯 **Timeline**: Weeks, not months

See [zed-fix-implemented.md](./zed-fix-implemented.md) for complete implementation details.

## Alternative Approaches Considered

See [alternative-approaches.md](./alternative-approaches.md) for full analysis:

1. ❌ **Merged Grammar** - Violates grammar architecture, technically infeasible
2. ✅ **Built-in markdown-inline** - Current workaround (implemented)
3. 📋 **Ask Zed Team** - Will file issue as part of contribution
4. 📋 **Contribute to Zed** - Long-term proper fix (planned)
5. ⏸️ **Wait for Zed Fix** - Being proactive by contributing ourselves

## Key Findings

1. **Root cause identified**: Registry version not incremented when extension grammars load asynchronously
2. **Simple fix**: One-line change to increment version after loading completes
3. **Architecture sound**: Pandoc's dual-grammar design is correct and intentional
4. **Fix implemented**: In branch `fix/extension-grammar-injection`, pending testing and PR

## Timeline

- **2025-10-11**: Initial investigation, implemented workaround
- **2025-10-12**: Deep investigation, researched all major Zed extensions
- **2025-10-12**: Code analysis of Zed's injection resolution system
- **2025-10-12**: Confirmed root cause via source code examination
- **2025-10-12**: Implemented fix (one-line change)
- **Next**: Test fix, submit PR to Zed

## References

### Investigation Documents
- [verification-findings.md](./verification-findings.md) - Complete code analysis confirming root cause
- [verification-plan.md](./verification-plan.md) - Investigation methodology used
- [extension-research.md](./extension-research.md) - Testing history and extension pattern analysis

### Solution Documents
- [zed-fix-implemented.md](./zed-fix-implemented.md) - **Implementation details for the fix**
- [builtin-injection-test.md](./builtin-injection-test.md) - Test results for current workaround
- [alternative-approaches.md](./alternative-approaches.md) - All approaches considered
- [zed-modification-analysis.md](./zed-modification-analysis.md) - Original hypotheses and proposed solutions
