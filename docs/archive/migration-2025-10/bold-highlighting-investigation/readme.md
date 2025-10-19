# Bold Highlighting Investigation

Investigation into why bold/italic text highlighting wasn't working in Quarto `.qmd` files.

## Summary

**Problem**: Bold (`**text**`) and italic (`*text*`) weren't highlighted in `.qmd` files when using the Quarto extension.

**Root Cause (Investigated)**: Initial investigation suggested the LanguageRegistry version wasn't being incremented when extension grammars loaded asynchronously, preventing SyntaxMap from rechecking pending injections.

**Resolution**: Testing confirmed that extension-to-extension grammar injection **already works** in Zed. The mechanism for incrementing version and resolving pending injections has been present since February 2024 (via `state.add()`). No Zed modifications were needed.

**Current Solution**: Full `pandoc_markdown_inline` grammar injection with 100% inline formatting support.

**Status**: ✅ Working as of main branch. See [ck37/zed#1](https://github.com/ck37/zed/issues/1) for complete investigation findings.

## Technical Background

### Dual Grammar Architecture

The pandoc-markdown grammar uses two grammars:
- **Block grammar** (`tree-sitter-pandoc-markdown`): Parses document structure, creates `(inline)` nodes
- **Inline grammar** (`tree-sitter-pandoc-markdown-inline`): Parses inline content like bold, italic, links

The inline grammar must be injected into `(inline)` nodes for emphasis highlighting to work.

### The Solution

**Extension injection** (working):
```scheme
((inline) @injection.content
 (#set! injection.language "Pandoc Markdown Inline"))
```
✅ Extension grammars loaded as WASM can be injection targets

### The Investigation

Research and testing confirmed:
1. **Extension grammars ARE registered** in the same language registry as built-ins
2. **Loading IS triggered** when injection is first encountered
3. **Version increment happens**: `state.add()` increments version when loading completes (language_registry.rs:1211)
4. **Subscribers notified**: `state.add()` notifies watch channel subscribers (language_registry.rs:1212)
5. **LSP store reacts**: Subscription wakes up and reparses buffers with unknown injections (lsp_store.rs:4056, 4132)
6. **Pending injections resolve**: SyntaxMap detects version change and rechecks pending injections (syntax_map.rs:418-456)
7. **Mechanism works**: Extension-to-extension injection has worked correctly since February 2024

See [verification-findings.md](./verification-findings.md) for code analysis and [ck37/zed#1](https://github.com/ck37/zed/issues/1) for complete investigation.

## Current Implementation

**Full Pandoc Inline Grammar** - 100% coverage

✅ **All features supported**:
- Bold (`**`, `__`)
- Italic (`*`, `_`)
- Bold+italic (`***`)
- Inline code (`` ` ``)
- Links (`[text](url)`)
- Pandoc extensions: strikethrough (`~~`), subscript (`~`), superscript (`^`), highlight (`==`), underline

## Investigation Outcome

**Initial hypothesis**: Zed needed modifications to support extension-to-extension injection. Proposed adding explicit `state.version += 1` increment.

**Actual finding**: The mechanism already worked. The proposed fix would have been redundant - `state.add()` already increments version and notifies subscribers. Testing with actual Quarto extension confirmed injection works correctly.

**Lesson learned**: Always test the actual behavior before assuming modifications are needed. The subscription-based architecture was already complete and functional.

## Timeline

- **2025-10-11**: Initial investigation, identified dual-grammar architecture
- **2025-10-12**: Deep investigation of Zed's injection resolution system
- **2025-10-12**: Proposed fix to increment registry version
- **2025-10-12**: Implemented and tested proposed fix
- **2025-10-14**: Discovered proposed fix was redundant with existing `state.add()` behavior
- **2025-10-14**: Tested and confirmed extension injection works without modifications
- **2025-10-14**: Merged `use-pandoc-inline-grammar` to `main` - full highlighting now default

## References

### Investigation Documents
- [verification-findings.md](./verification-findings.md) - Initial investigation of injection mechanism
- [zed-fix-implemented.md](./zed-fix-implemented.md) - Investigation into proposed (but unnecessary) fix
- [ck37/zed#1](https://github.com/ck37/zed/issues/1) - Complete analysis showing redundancy
- [verification-plan.md](./verification-plan.md) - Investigation methodology
- [extension-research.md](./extension-research.md) - Testing history

### Solution Documents
- [builtin-injection-test.md](./builtin-injection-test.md) - Test results for built-in markdown-inline (historical)
- [alternative-approaches.md](./alternative-approaches.md) - All approaches considered
- [zed-modification-analysis.md](./zed-modification-analysis.md) - Original hypotheses
