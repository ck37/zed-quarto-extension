# Documentation

This directory contains technical documentation for the Zed Quarto extension.

## Current Documentation

Essential reference documentation for using and developing the extension:

### Language Server & Features
- **[lsp-status.md](lsp-status.md)** - Why there's no LSP, what the extension provides, and future possibilities

### Architecture & Design
- **[syntax-highlighting-architecture.md](syntax-highlighting-architecture.md)** - How syntax highlighting works (tree-sitter vs TextMate)
- **[scope-naming-decision.md](scope-naming-decision.md)** - Why we use Zed's legacy scopes (`@text.*`) vs modern scopes (`@markup.*`)

### Grammar Development
- **[grammar-roadmap.md](grammar-roadmap.md)** - Development phases (both Phase 1 and Phase 2 complete)
- **[grammar-feature-needs.md](grammar-feature-needs.md)** - Missing Pandoc features and future enhancements
- **[tree-sitter-quarto-plan.md](tree-sitter-quarto-plan.md)** - Implementation details for tree-sitter-quarto

### Technical Reference
- **[scope-reference.md](scope-reference.md)** - Comprehensive scope reference: supported scopes, limitations, Pandoc extensions
- **[debug-theme.md](debug-theme.md)** - How to debug theme and scope issues
- **[upstream-warnings.md](upstream-warnings.md)** - Upstream grammar warnings and issues

### Current Issues
- **[next-steps.md](next-steps.md)** - Current priorities and next development steps
- **[triple-asterisk-issue.md](triple-asterisk-issue.md)** - Known issue with triple asterisk bold+italic syntax

## Historical Documentation

Documentation from the October 2025 migration from tree-sitter-pandoc-markdown to tree-sitter-quarto:

- **[archive/migration-2025-10/](archive/migration-2025-10/)** - Investigation notes, debugging sessions, and implementation details from the migration

These files document the development process and troubleshooting steps. They're preserved for historical reference but may contain outdated information about the current implementation.

## Quick Links

- **Extension Repository**: https://github.com/ck37/zed-quarto-extension
- **Grammar Repository**: https://github.com/ck37/tree-sitter-quarto
- **Contributing Guide**: [../CONTRIBUTING.md](../CONTRIBUTING.md)
- **Main README**: [../README.md](../README.md)
