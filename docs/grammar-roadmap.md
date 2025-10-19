# Grammar Development Roadmap

This document outlines the staged approach to building comprehensive Quarto support in Zed through tree-sitter grammars.

## Phase 1 – Strengthen `tree-sitter-pandoc-markdown`

**Goal**: Upstream missing Pandoc constructs that Quarto relies on to the `tree-sitter-pandoc-markdown` grammar.

**Status**: ✅ Complete!

**Accomplished**:
- Upstream missing Pandoc constructs that Quarto relies on (callout div fences, shortcodes, cross-references, attribute parsing, YAML front matter, etc.)
- Expose richer node types in the inline grammar so editors can apply differentiated highlighting immediately
- Share the improvements with every consumer of Pandoc Markdown while keeping the grammar strictly Pandoc-compatible

**Benefits**:
- Provides immediate value to all editors using `tree-sitter-pandoc-markdown`
- Maintains strict Pandoc compatibility
- Establishes foundation for Quarto-specific extensions

## Phase 2 – Build `tree-sitter-quarto`

**Goal**: Create a dedicated Quarto grammar that extends Pandoc markdown with Quarto-only syntax.

**Status**: ✅ Complete! (Migrated in commit 1877b3a)

**Accomplished**:
- Created dedicated [`tree-sitter-quarto`](https://github.com/ck37/tree-sitter-quarto) grammar
- Supports Quarto-specific syntax (chunk option comment lines `#|`, cell attributes, Quarto-specific shortcodes)
- Extends tree-sitter-markdown with both Pandoc and Quarto features
- Extension now uses tree-sitter-quarto for comprehensive Quarto support
- Grammar uses Zed-compatible scopes (`@text.*`, `@emphasis.*`) for theme support

**Benefits**:
- First-class support for all Quarto-specific syntax
- Can be adopted by other editors (Neovim, Helix, etc.)
- Unified grammar for Quarto documents

**Remaining work**: Feature enhancements tracked in [`grammar-feature-needs.md`](grammar-feature-needs.md)

## Historical Note

This extension initially used `tree-sitter-pandoc-markdown` (dual grammar: block + inline) as an interim solution. As of October 2025, we've successfully migrated to `tree-sitter-quarto`, which provides unified Quarto-aware syntax highlighting. See [`tree-sitter-quarto-plan.md`](tree-sitter-quarto-plan.md) for the implementation approach taken.
