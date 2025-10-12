# Bold Highlighting Investigation

Investigation into why bold/italic text highlighting wasn't working in Quarto `.qmd` files, and the workaround implemented.

## Summary

**Problem**: Bold (`**text**`) and italic (`*text*`) weren't highlighted in `.qmd` files.

**Root Cause**: The pandoc-markdown grammar uses dual-grammar architecture (block + inline grammars). Zed extensions cannot inject custom grammars into other custom grammars—only built-in languages.

**Current Solution (70% coverage)**: Inject Zed's built-in `markdown-inline` grammar as workaround.

**Long-term Solution**: Contribute PR to Zed enabling custom-to-custom grammar injection.

## Technical Background

### Dual Grammar Architecture

The pandoc-markdown grammar uses two grammars:
- **Block grammar** (`tree-sitter-pandoc-markdown`): Parses document structure, creates `(inline)` nodes
- **Inline grammar** (`tree-sitter-pandoc-markdown-inline`): Parses inline content like bold, italic, links

The inline grammar must be injected into `(inline)` nodes for emphasis highlighting to work.

### The Limitation

**Zed's built-in markdown** (works):
```scheme
((inline) @injection.content
 (#set! injection.language "markdown-inline"))
```
✅ Works because `"markdown-inline"` is bundled with Zed

**Our extension** (doesn't work):
```scheme
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))
```
❌ Fails because `"pandoc_markdown_inline"` is extension-defined

### Evidence

Research of all major Zed extensions found:
- ✅ Extensions CAN inject built-in languages (JavaScript, Python, SQL, CSS, HTML)
- ❌ Extensions CANNOT inject custom grammars into other custom grammars
- All successful injections target built-in Zed languages only
- [Issue #484](https://github.com/zed-industries/extensions/issues/484): Open bug about injection limitations

See [extension-research.md](./extension-research.md) for detailed analysis.

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

## Long-term Solution: Contribute to Zed

**Plan**: Contribute PR to Zed to enable custom-to-custom grammar injection.

**Timeline**:
- ✅ **Now**: Built-in injection workaround (70% coverage)
- **1-2 months**: File Zed issue with research findings
- **2-4 months**: Contribute PR to Zed
- **Future**: Switch to full Pandoc inline grammar (100% coverage)

See [zed-modification-analysis.md](./zed-modification-analysis.md) for detailed contribution plan.

## Alternative Approaches Considered

See [alternative-approaches.md](./alternative-approaches.md) for full analysis:

1. ❌ **Merged Grammar** - Violates grammar architecture, technically infeasible
2. ✅ **Built-in markdown-inline** - Current workaround (implemented)
3. 📋 **Ask Zed Team** - Will file issue as part of contribution
4. 📋 **Contribute to Zed** - Long-term proper fix (planned)
5. ⏸️ **Wait for Zed Fix** - Being proactive by contributing ourselves

## Key Findings

1. **Zed limitation confirmed**: Extensions cannot inject extension-defined grammars
2. **No workaround exists**: Built-in injection is best available option
3. **Architecture sound**: Pandoc's dual-grammar design is correct and intentional
4. **Solution requires Zed change**: Must extend Zed's grammar resolution to include extension grammars

## Timeline

- **2025-10-11**: Initial investigation
- **2025-10-12**: Identified root cause (Zed limitation)
- **2025-10-12**: Researched all major Zed extensions
- **2025-10-12**: Implemented built-in injection workaround (70% coverage)
- **Next**: File Zed issue and prepare contribution

## References

- [builtin-injection-test.md](./builtin-injection-test.md) - Test results for current solution
- [alternative-approaches.md](./alternative-approaches.md) - All approaches considered
- [zed-modification-analysis.md](./zed-modification-analysis.md) - Plan for contributing to Zed
- [extension-research.md](./extension-research.md) - Detailed extension pattern analysis
- [Zed Issue #484](https://github.com/zed-industries/extensions/issues/484) - Injection limitations
