# Relationship to quarto-dev/quarto-markdown

**Important Discovery:** The official Quarto organization (quarto-dev) has an existing parser project at https://github.com/quarto-dev/quarto-markdown

## What is quarto-markdown?

**Status:** NOT READY FOR PRODUCTION USE (as of 2025-10-13)

**Description:** A standalone Rust-based parser for Quarto Markdown (QMD) that:
- Uses "a pair of tree-sitter grammars forked from the tree-sitter grammar repo"
- Provides first-class support for external tooling
- Recognizes syntax errors
- Emits parse trees in Pandoc's JSON and native formats

**Key Features:**
- Code cell blocks with `{language}` syntax
- Shortcodes
- "Reader syntax" for using other Pandoc reader formats
- Stricter than CommonMark (intentionally provides syntax errors)

**Current Limitations:**
- Not yet integrated into Quarto
- Performance issues
- Error messages need improvement
- Explicitly marked as not ready for public consumption

## Key Differences: quarto-markdown vs tree-sitter-quarto

| Aspect | quarto-markdown | tree-sitter-quarto (this project) |
|--------|-----------------|-----------------------------------|
| **Primary Goal** | Standalone parser for Quarto rendering pipeline | Editor integration (highlighting, navigation, folding) |
| **Output Format** | Pandoc AST (JSON/native) | Tree-sitter parse tree |
| **Use Case** | Document compilation/rendering | Editor features (LSP, syntax highlighting) |
| **Status** | Experimental, not production-ready | Planning phase |
| **Integration** | Intended for Quarto CLI | Intended for editors (Neovim, Zed, Helix) |
| **Architecture** | Rust binary with tree-sitter grammars | Pure tree-sitter grammar for editor consumption |
| **Error Handling** | Strict syntax error reporting | Incremental, error-tolerant parsing for editors |
| **Chunk Options** | Likely parsed but converted to Pandoc AST | First-class nodes for editor features |

## Why tree-sitter-quarto is Still Needed

Despite quarto-markdown's existence, tree-sitter-quarto serves a **different purpose**:

### 1. **Different Use Cases**
- **quarto-markdown:** Rendering pipeline (replaces/augments Pandoc for Quarto)
- **tree-sitter-quarto:** Editor integration (syntax highlighting, navigation, autocomplete)

### 2. **Different Consumers**
- **quarto-markdown:** Quarto CLI, build tools
- **tree-sitter-quarto:** Editors (Neovim, Zed, Helix, VSCode)

### 3. **Different Architecture**
- **quarto-markdown:** Rust binary that produces Pandoc AST
- **tree-sitter-quarto:** Pure tree-sitter grammar that editors can consume directly via tree-sitter bindings

### 4. **Different Parsing Goals**
- **quarto-markdown:** Parse for compilation (strict, complete)
- **tree-sitter-quarto:** Parse for editing (incremental, error-tolerant, real-time)

## Potential Synergies

While the projects serve different purposes, there are opportunities for collaboration:

1. **Grammar Reuse:** quarto-markdown's tree-sitter grammars could potentially be adapted for editor use
2. **Feature Parity:** Ensure both parsers recognize the same Quarto syntax
3. **Testing:** Share test cases for Quarto syntax features
4. **Community:** Coordinate with Quarto team to avoid duplication of effort

## Decision: Should We Continue tree-sitter-quarto?

**Recommendation: YES, with coordination**

### Reasons to Continue:

1. **Different Target:** Editors need tree-sitter grammars directly, not Rust binaries that output Pandoc AST
2. **Editor Integration:** Tree-sitter grammars are the standard way editors integrate language support
3. **quarto-markdown Status:** It's not production-ready and may not prioritize editor features
4. **Editor-Specific Features:** Our goals (semantic highlighting, chunk option autocomplete, cross-reference navigation) are editor-focused

### Recommended Actions:

1. **Contact Quarto Team:**
   - Introduce our project
   - Ask if they'd be interested in collaboration
   - Understand their timeline and priorities
   - Offer to contribute or coordinate

2. **Examine quarto-markdown Grammars:**
   - Review their forked tree-sitter grammars
   - Determine if they can be used for editor integration
   - Identify gaps for editor-specific features

3. **Consider Contribution vs Separate Project:**
   - If their grammars work for editors: contribute editor-specific queries
   - If their grammars are rendering-focused: continue as separate editor-focused project
   - Either way: maintain close coordination

4. **Update Project Goals:**
   - Clarify this is specifically for **editor integration**
   - Position as complementary to quarto-markdown, not competing
   - Focus on features editors need: highlighting, injections, folding, navigation

## Next Steps

1. ✅ Document this relationship
2. ⬜ Investigate quarto-markdown's tree-sitter grammar implementation
3. ⬜ Reach out to Quarto team (open GitHub issue or discussion)
4. ⬜ Update README to clarify relationship and positioning
5. ⬜ Decide whether to build on their grammars or start fresh

## Conclusion

tree-sitter-quarto is still valuable and needed because:
- Editors require tree-sitter grammars for syntax highlighting and features
- quarto-markdown is a Rust binary for rendering, not an editor grammar
- Editor-specific needs (incremental parsing, error tolerance, semantic highlighting) differ from rendering needs
- The project can fill the gap between Quarto and editor ecosystems

However, we should coordinate with the Quarto team to avoid unnecessary duplication and explore collaboration opportunities.
