# Add Pandoc Inline Formatting Extensions

## Why

tree-sitter-quarto currently supports basic inline formatting (bold, italic) but lacks support for several commonly-used Pandoc Markdown inline formatting extensions that are essential for scientific and academic writing. Without these features, documents with strikethrough (`~~text~~`), highlight (`==text==`), subscript (`H~2~O`), and superscript (`x^2^`) are either parsed as plain text or produce parse errors, resulting in poor editor experience for Quarto users writing scientific/academic content.

## What Changes

- Add **strikethrough** node type for `~~text~~` syntax
- Add **highlight** node type for `==text==` syntax
- Add **subscript** node type for `H~2~O` syntax
- Add **superscript** node type for `x^2^` syntax
- Update inline parsing rules to recognize these formatting patterns
- Add external scanner support for disambiguating `~` and `^` characters
- Add highlight queries for semantic scoping of new node types
- Add comprehensive corpus tests covering basic usage, nesting, and edge cases

## Impact

- **Affected specs**: pandoc-inline-formatting (new capability)
- **Affected code**:
  - `grammar.js`: Add inline formatting rules
  - `src/scanner.c`: Add external scanner logic for delimiter disambiguation
  - `queries/highlights.scm`: Add modern semantic scopes
  - `queries/zed/highlights.scm`: Add Zed-compatible scopes
  - `test/corpus/inline-formatting.txt`: New test corpus file
- **Breaking changes**: None - this is purely additive
- **Related issue**: [#3](https://github.com/ck37/tree-sitter-quarto/issues/3)
