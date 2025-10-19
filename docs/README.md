# Documentation

This directory contains technical documentation for the Zed Quarto extension.

## Current Documentation

Essential reference documentation for using and developing the extension:

### Language Server & Features
- **[LSP_STATUS.md](LSP_STATUS.md)** - Why there's no LSP, what the extension provides, and future possibilities

### Architecture & Design
- **[syntax-highlighting-architecture.md](syntax-highlighting-architecture.md)** - How syntax highlighting works (tree-sitter vs TextMate)
- **[scope-naming-decision.md](scope-naming-decision.md)** - Why we use Zed's legacy scopes (`@text.*`) vs modern scopes (`@markup.*`)

### Grammar Development
- **[grammar-roadmap.md](grammar-roadmap.md)** - Development phases (both Phase 1 and Phase 2 complete)
- **[grammar-feature-needs.md](grammar-feature-needs.md)** - Missing Pandoc features and future enhancements
- **[tree-sitter-quarto-plan.md](tree-sitter-quarto-plan.md)** - Implementation details for tree-sitter-quarto

### Limitations & Issues
- **[pandoc-extensions-scope-issue.md](pandoc-extensions-scope-issue.md)** - Why strikethrough, highlight, subscript, superscript don't highlight
- **[NEXT_STEPS.md](NEXT_STEPS.md)** - Current priorities and next development steps
- **[TRIPLE_ASTERISK_ISSUE.md](TRIPLE_ASTERISK_ISSUE.md)** - Known issue with triple asterisk bold+italic syntax

### Technical Reference
- **[zed-syntax-scopes.md](zed-syntax-scopes.md)** - Complete list of Zed-supported scopes
- **[zed-theme-scope-limitations.md](zed-theme-scope-limitations.md)** - Zed theme system limitations
- **[DEBUG_THEME.md](DEBUG_THEME.md)** - How to debug theme and scope issues
- **[UPSTREAM_WARNINGS.md](UPSTREAM_WARNINGS.md)** - Upstream grammar warnings and issues

### Testing & Development
- **[manual-test-steps.md](manual-test-steps.md)** - Manual testing procedures
- **[tree-sitter-0.25-migration.md](tree-sitter-0.25-migration.md)** - Tree-sitter 0.25 upgrade notes
- **[tree-sitter-quarto-multi-editor-proposal.md](tree-sitter-quarto-multi-editor-proposal.md)** - Proposal for multi-editor grammar support
- **[wasm-testing-design.md](wasm-testing-design.md)** - WASM testing infrastructure
- **[wasm-test-fixes.md](wasm-test-fixes.md)** - WASM compilation fixes

## Historical Documentation

Documentation from the October 2025 migration from tree-sitter-pandoc-markdown to tree-sitter-quarto:

- **[archive/migration-2025-10/](archive/migration-2025-10/)** - Investigation notes, debugging sessions, and implementation details from the migration

These files document the development process and troubleshooting steps. They're preserved for historical reference but may contain outdated information about the current implementation.

## Quick Links

- **Extension Repository**: https://github.com/ck37/zed-quarto-extension
- **Grammar Repository**: https://github.com/ck37/tree-sitter-quarto
- **Contributing Guide**: [../CONTRIBUTING.md](../CONTRIBUTING.md)
- **Main README**: [../README.md](../README.md)
