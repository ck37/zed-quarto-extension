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
- Remaining features tracked in [`grammar-feature-needs.md`](grammar-feature-needs.md)

## Phase 2 – Build `tree-sitter-quarto` on top of those improvements

**Goal**: Create a dedicated Quarto grammar that extends Pandoc markdown with Quarto-only syntax.

**Status**: 📋 Planned

**Target features**:
- Layer Quarto-only syntax (chunk option comment lines `#|`, cell attribute blocks, layout/new shortcode directives, execution option cascades) that are out of scope for Pandoc itself
- Provide semantic nodes that unlock Quarto-specific tooling without fragmenting the Pandoc ecosystem

**Why a separate grammar?**
Even after Phase 1, Quarto introduces syntax (e.g., executable option lines, cell attribute cascades) that goes beyond Pandoc's spec. Capturing those semantics cleanly warrants a separate grammar that can depend on—but not compromise—the upstream Pandoc parser.

**Benefits**:
- First-class support for all Quarto-specific syntax
- Can be adopted by other editors (Neovim, Helix, etc.)
- Maintained in collaboration with the Quarto project
- Potentially lives in the official `tree-sitter-grammars` organization

For implementation details, see [`tree-sitter-quarto-plan.md`](tree-sitter-quarto-plan.md).

## Why This Staged Approach?

This plan avoids duplicating work, gives near-term wins for existing editors, and positions a Quarto grammar to focus solely on features Pandoc cannot represent.
