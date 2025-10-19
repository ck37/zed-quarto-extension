# October 2025 Migration Archive

This directory contains historical documentation from the migration from `tree-sitter-pandoc-markdown` to `tree-sitter-quarto` (October 2025).

## Migration Summary

- **Date**: October 2025
- **Completion**: Commit 1877b3a
- **Goal**: Migrate from dual grammar (pandoc-markdown block + inline) to unified tree-sitter-quarto grammar
- **Outcome**: ✅ Successful - Extension now uses tree-sitter-quarto with comprehensive Quarto support

## What's Here

### Migration Status
- **migration-status-2025-10-18.md** - Comprehensive migration status, test results, and verification steps

### Investigation & Debugging
- **current-debugging-notes.md** - Real-time debugging session notes
- **investigation-notes-2025-10-17.md** - Investigation findings during migration
- **bold-highlighting-investigation/** - Detailed investigation of bold/italic highlighting issues
- **bold-italic-highlighting-debugging.md** - Additional debugging notes
- **manual-test-steps.md** - Manual testing procedures for bold/italic highlighting debugging

### Grammar Updates
- **tree-sitter-quarto-update-2025-10-17.md** - Grammar update notes (v1)
- **tree-sitter-quarto-update-2025-10-17-v2.md** - Grammar update notes (v2)
- **update-to-28111dc-2025-10-18.md** - Update to specific grammar commit
- **update-to-4012bc7-2025-10-18.md** - Update to specific grammar commit

### Issues & Fixes
- **code-injection-fix-2025-10-18.md** - Code injection fix
- **double-capture-fix-2025-10-17.md** - Double capture issue resolution
- **catch-all-query-issue.md** - Catch-all query pattern issue
- **citation-highlighting-issue.md** - Citation highlighting investigation
- **emphasis-delimiter-limitation.md** - Emphasis delimiter limitations
- **emphasis-delimiter-support.md** - Emphasis delimiter support notes
- **hyphenated-headings-issue.md** - Hyphenated heading parsing
- **reference-style-links-issue.md** - Reference-style link highlighting

### Analysis & Planning
- **highlighting-failure-analysis.md** - Root cause analysis of highlighting failures
- **inline-highlighting-investigation.md** - Inline content highlighting investigation
- **github-issue-pandoc-inline-formatting.md** - GitHub issue documentation
- **scope-documentation-added.md** - Scope documentation additions
- **scope-validation-summary.md** - Scope validation results
- **one-dark-scope-analysis.md** - One Dark theme scope analysis
- **markdown-scope-compatibility.md** - Markdown scope compatibility notes
- **language-grammar-requirements.md** - Grammar requirement analysis
- **zed-query-loading-test.md** - Query loading mechanism tests
- **zed-query-loading-test-results.md** - Test results

### Technical Implementation
- **tree-sitter-0.25-migration.md** - Tree-sitter 0.25 API migration (Oct 17, 2025)
- **tree-sitter-quarto-multi-editor-proposal.md** - Historical proposal for multi-editor query support
- **wasm-testing-design.md** - WASM testing infrastructure design
- **wasm-test-fixes.md** - WASM integration test fixes (Oct 17, 2025)

## Current Implementation

For current documentation about how the extension works post-migration, see:
- [../../grammar-roadmap.md](../../grammar-roadmap.md) - Updated roadmap with Phase 2 complete
- [../../tree-sitter-quarto-plan.md](../../tree-sitter-quarto-plan.md) - Implementation details
- [../../syntax-highlighting-architecture.md](../../syntax-highlighting-architecture.md) - Current architecture

## Key Learnings

1. **Upstream grammar scopes**: We patched tree-sitter-quarto to use Zed-compatible scopes (`@text.*`) by default, avoiding extension-side workarounds
2. **Unified grammar**: Single tree-sitter-quarto grammar provides better Quarto support than dual pandoc-markdown grammars
3. **Test infrastructure**: Comprehensive automated tests (67 tests) ensure highlighting works correctly
4. **Query loading**: Zed loads grammar's built-in queries, so queries must live in grammar repo, not just extension

## Historical Context

These documents were created during active development and debugging. They contain:
- ✅ Valuable investigation process documentation
- ✅ Problem-solving approaches and lessons learned
- ⚠️ Some outdated information (references to planned features now implemented)
- ⚠️ Real-time debugging notes that may be incomplete

For accurate current information, refer to the main docs/ directory.
