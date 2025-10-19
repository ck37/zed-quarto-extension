# Proposal: Fix Inline Content Highlighting

## Problem Statement

Users report that inline content (text inside emphasis, strong emphasis, and headings) is not being highlighted in Zed when using the tree-sitter-quarto grammar. Specifically:

1. **Emphasis/Strong**: The content inside `*italic*`, `**bold**`, and `***bold italic***` is not highlighted - only the markers are styled
2. **Headings**: The content inside `# heading text` is not highlighted - only the `#` marker is styled
3. **New Pandoc formatting**: Recently added inline formatting (strikethrough, highlight, subscript, superscript) may have similar issues

This affects the user experience as formatted text is not visually distinguished from regular text, reducing the value of syntax highlighting.

## Current Behavior

Based on investigation:
- The tree-sitter-quarto grammar correctly parses the AST structure with child `(text)` nodes inside `(emphasis)`, `(strong_emphasis)`, and `(inline)` nodes
- The extension's `highlights.scm` queries capture parent nodes: `(emphasis) @text.emphasis`, `(strong_emphasis) @emphasis.strong`, `content: (inline) @text.title`
- However, the child text content is not being highlighted in Zed's display

## Root Cause (Hypothesis)

Tree-sitter highlighting applies scopes on a per-node basis. When we apply `@text.emphasis` to the parent `(emphasis)` node, it may not automatically propagate styling to descendant `(text)` nodes. This could be due to:
1. Zed's theme not inheriting styles for child nodes
2. The `(text) @text` catch-all query (line 189 in highlights.scm) overriding parent styles with lower priority
3. Query ordering or priority issues

## Proposed Solution

Investigate and fix the highlighting queries to ensure inline content inherits or explicitly receives appropriate styling:

1. **Diagnostic Phase**: Create comprehensive test cases to validate current highlighting behavior and isolate the issue
2. **Query Analysis**: Examine how tree-sitter query captures interact with Zed's highlighting system
3. **Fix Implementation**: Update `highlights.scm` with appropriate queries to ensure child content is highlighted
4. **Validation**: Add automated tests to prevent regression

## Success Criteria

1. Text inside `*italic*` displays with italic styling in Zed
2. Text inside `**bold**` displays with bold styling in Zed
3. Text inside `***bold italic***` displays with combined styling in Zed
4. Heading content (e.g., `# My Heading`) displays with heading color/style in Zed
5. New Pandoc inline formatting (strikethrough, highlight, subscript, superscript) displays correctly
6. Automated tests validate highlighting coverage for all inline formatting patterns

## Non-Goals

- Fixing grammar-level parsing issues (the AST structure is correct)
- Changing the semantic scope names (we use Zed-compatible legacy scopes)
- Adding new inline formatting features beyond what tree-sitter-quarto already supports

## Dependencies

- Requires understanding of Zed's theme inheritance and highlight priority system
- May require testing with tree-sitter-highlight to validate query behavior
- Should reference existing bold-highlighting-investigation docs for context

## Risks

- Query changes might affect other highlighting patterns
- Solution may require Zed-specific workarounds that differ from standard tree-sitter conventions
- Priority rules may interact unpredictably with existing queries
