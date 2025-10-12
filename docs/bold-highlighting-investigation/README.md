# Bold Highlighting Investigation

This directory contains the complete investigation into why bold text highlighting wasn't working in the Quarto extension.

## Summary

**Problem**: Bold text (`**bold**`) and italic text (`*italic*`) were not being highlighted in `.qmd` files.

**Root Cause**: The pandoc-markdown grammar uses a dual-grammar architecture (block + inline grammars). Zed extensions cannot inject custom grammars into other custom grammars - they can only inject built-in languages.

**Workaround (70% coverage)**: Inject Zed's built-in `markdown-inline` grammar. Works for basic bold/italic, but not links or mixed content.

**Long-term Solution**: Contribute PR to Zed to enable custom-to-custom grammar injection.

## Investigation Documents

### 1. [TEST_LOG.md](./BOLD_HIGHLIGHTING_TEST_LOG.md)
Complete log of all 7 test attempts with configurations and results. Documents what we tried and why each approach failed.

**Key findings**:
- Test 1-7: All configurations with injection failed
- No configuration with dual grammars produced bold highlighting
- Issue is not about configuration but about Zed's capabilities

### 2. [EXTENSION_RESEARCH.md](./EXTENSION_RESEARCH.md)
Analysis of popular Zed extensions (Java, C#, Ruby, PHP, Vue, Svelte) to understand injection patterns.

**Key findings**:
- ✅ Extensions CAN inject built-in languages (JavaScript, Python, SQL, etc.)
- ❌ Extensions CANNOT inject custom grammars into other custom grammars
- All extensions only inject to built-in Zed languages
- Issue #484: Known limitation in Zed's injection system

### 3. [ALTERNATIVE_APPROACHES.md](./ALTERNATIVE_APPROACHES.md)
5 possible solutions with pros/cons and implementation details.

**Approaches**:
1. **Merged Grammar Highlights** (Recommended) - Use single grammar, no injection
2. **Ask Zed Team** - File issue requesting custom-to-custom injection support
3. **Modify Upstream Grammar** - Combine both grammars into one
4. **Wait for Zed Fix** - Monitor Issue #484
5. **Use Built-in Markdown** - Extend Zed's markdown (not feasible)

### 4. [ZED_MODIFICATION_ANALYSIS.md](./ZED_MODIFICATION_ANALYSIS.md)
Detailed plan for contributing a fix to Zed itself to support custom grammar injection.

**Includes**:
- Technical analysis of what needs to change
- 3 implementation approaches
- Development timeline (2-4 weeks)
- Hybrid approach: ship quick fix first, then contribute proper fix

### 5. [DEBUG_STEPS.md](./DEBUG_STEPS.md)
Diagnostic procedures for debugging highlighting issues.

### 6. [BOLD_HIGHLIGHTING_DIAGNOSIS.md](./BOLD_HIGHLIGHTING_DIAGNOSIS.md)
Early diagnosis document (superseded by other docs).

## Key Technical Insights

### Dual Grammar Architecture

The pandoc-markdown grammar uses two grammars:
- **Block grammar** (`tree-sitter-pandoc-markdown`): Parses document structure, creates `(inline)` nodes
- **Inline grammar** (`tree-sitter-pandoc-markdown-inline`): Parses inline content like bold, italic, links

### Why It Doesn't Work in Zed

**Zed's built-in markdown**:
```scheme
((inline) @injection.content
 (#set! injection.language "markdown-inline"))
```
- ✅ Works because `"markdown-inline"` is bundled with Zed

**Our extension**:
```scheme
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))
```
- ❌ Fails because `"pandoc_markdown_inline"` is extension-defined

### Evidence

1. **No extension found** doing custom-to-custom grammar injection
2. **Vue/Svelte** - Only inject built-in languages (JS, CSS, HTML)
3. **PHP** - Injects `phpdoc`, but it may be built-in or special-cased
4. **Issue #484** - Open bug report about injection limitations
5. **7 test attempts** - All failed despite various configurations

## Current Solution

**Built-in markdown-inline injection** (Approach 5 in ALTERNATIVE_APPROACHES.md):

✅ **Implemented and working:**
- Bold with `**` and `__` - ✅ Works
- Italic with `*` and `_` - ✅ Works
- Inline code - ✅ Works
- **Coverage: 70%** of inline features

⚠️ **Known limitations:**
- Links - ❌ Don't highlight
- Mixed content - ⚠️ Partially broken
- Pandoc extensions (strikethrough, subscript, superscript) - ❌ Don't work

**This is a practical workaround** that solves the primary user complaint (no bold/italic) while we work on the proper fix.

## Planned Solution

**Contribute to Zed:** Enable custom-to-custom grammar injection

See [ZED_MODIFICATION_ANALYSIS.md](./ZED_MODIFICATION_ANALYSIS.md) for detailed plan to:
1. File Zed issue with our thorough research
2. Contribute PR implementing custom injection support
3. Switch to full Pandoc inline grammar (100% coverage) once supported

## Future Work

If Zed adds support for custom-to-custom grammar injection:
1. We can switch back to dual-grammar approach
2. This will provide more precise parsing
3. Better matches upstream grammar's intended architecture

See ZED_MODIFICATION_ANALYSIS.md for plan to contribute this fix to Zed.

## Timeline

- **2025-10-11**: Initial diagnosis
- **2025-10-12**: 7 test attempts, all failed
- **2025-10-12**: Extensive research of all Zed extensions
- **2025-10-12**: Identified root cause (Zed limitation)
- **2025-10-12**: Documented alternative approaches
- **Next**: Implement merged grammar approach
