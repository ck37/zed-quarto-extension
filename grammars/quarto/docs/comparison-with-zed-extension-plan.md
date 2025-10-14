# Comparison with zed-quarto-extension Plan

**Source:** https://github.com/ck37/zed-quarto-extension/blob/use-pandoc-inline-grammar/docs/tree-sitter-quarto-plan.md

This document compares the plan from the Zed Quarto extension repository with the current tree-sitter-quarto project.

## Key Differences

### 1. Base Grammar Choice

**Zed Extension Plan:**
```javascript
const markdown = require('tree-sitter-markdown/common/common.js');
module.exports = grammar(markdown, {
  name: 'quarto',
  // ...
});
```
- Extends tree-sitter-markdown directly
- Single unified grammar

**Current tree-sitter-quarto Plan:**
```javascript
const pandoc = require('../tree-sitter-pandoc-markdown/grammar.js');
module.exports = grammar(pandoc, {
  name: 'quarto',
  // ...
});
```
- Extends tree-sitter-pandoc-markdown (which itself extends tree-sitter-markdown)
- Split block/inline grammar architecture

### 2. Architecture Approach

| Aspect | Zed Extension Plan | Current Plan |
|--------|-------------------|--------------|
| **Base** | tree-sitter-markdown | tree-sitter-pandoc-markdown |
| **Structure** | Unified grammar | Split block/inline |
| **Pandoc features** | Would need to re-implement | Already included |
| **Complexity** | Simpler, single grammar | More complex, better modularity |

### 3. Feature Priorities

**Zed Extension Plan emphasizes:**
1. Executable chunk option lines (`#| echo: false`)
2. Cell attribute blocks
3. Layout directives
4. Shortcodes
5. Execution metadata

**Current Plan emphasizes:**
1. Executable code cells
2. Chunk options (single-line, then multi-line)
3. Cross-reference distinction
4. Inline code cells
5. Callouts and divs

**Common priorities:**
- Chunk options (`#| key: value`)
- Executable cells
- Callouts
- Shortcodes

## Why Use tree-sitter-pandoc-markdown as Base?

### Advantages

1. **Pandoc Features Already Implemented:**
   - Citations (distinct from cross-references)
   - Divs with attributes (`::: {.class}`)
   - Spans with attributes
   - Definition lists
   - Tables (pipe, grid, simple)
   - Pandoc-specific extensions

2. **Split Grammar Architecture:**
   - Block grammar handles block-level elements
   - Inline grammar handles inline elements
   - Reduces parsing complexity
   - Matches Pandoc's internal structure

3. **Proven Approach:**
   - Already working in editors
   - Well-tested with real documents
   - Performance optimized

4. **Avoid Re-implementing:**
   - Don't need to add Pandoc features from scratch
   - Can focus on Quarto-specific additions

### Disadvantages

1. **Additional Dependency:**
   - Depends on tree-sitter-pandoc-markdown
   - Need to track upstream changes
   - More complex build

2. **Heavier Base:**
   - More features than basic markdown
   - Larger grammar size
   - More parser complexity

## Why NOT Use tree-sitter-markdown as Base?

### If we used tree-sitter-markdown:

**Advantages:**
- Simpler dependency chain
- Smaller base grammar
- Direct control over all features

**Disadvantages:**
- **Must re-implement Pandoc features:**
  - Citations
  - Divs with attributes
  - Spans
  - Tables variants
  - All Pandoc extensions
- **Duplicate effort** with tree-sitter-pandoc-markdown
- **Miss improvements** to pandoc-markdown grammar

## Recommendation: Use tree-sitter-pandoc-markdown

### Rationale

Quarto documents ARE Pandoc Markdown documents with additional features. The relationship is:

```
CommonMark
    ↓ extends
Pandoc Markdown (adds citations, divs, tables, etc.)
    ↓ extends
Quarto Markdown (adds executable cells, chunk options, cross-refs)
```

Since tree-sitter-pandoc-markdown already implements Pandoc features correctly, we should:
1. **Build on it** rather than re-implement
2. **Focus our effort** on Quarto-specific features
3. **Benefit from** ongoing pandoc-markdown improvements

### Implementation Strategy

**Extend tree-sitter-pandoc-markdown:**
```javascript
// Copy base grammar or use as dependency
const pandoc = require('./pandoc-markdown-base');

module.exports = grammar(pandoc, {
  name: 'quarto',

  externals: $ => [
    ...pandoc.externals,
    $.chunk_option_marker,  // #| at cell start
    $.cell_boundary         // ``` with Quarto context
  ],

  rules: {
    // Extend block rules with executable cells
    _block: $ => choice(
      ...pandoc.rules._block,
      $.executable_code_cell,
      $.enhanced_callout  // Quarto-specific callout features
    ),

    // Extend inline rules with inline cells
    _inline: $ => choice(
      ...pandoc.rules._inline,
      $.inline_code_cell,
      $.cross_reference  // Distinct from citation
    ),

    // New Quarto-specific rules
    executable_code_cell: $ => seq(
      $.cell_delimiter,
      '{', $.language_name, optional($.cell_attributes), '}',
      '\n',
      optional($.chunk_options),
      $.cell_content,
      $.cell_delimiter
    ),

    chunk_options: $ => repeat1($.chunk_option),

    chunk_option: $ => seq(
      $.chunk_option_marker,  // external: #| at start
      field('key', $.chunk_option_key),
      ':',
      field('value', $.chunk_option_value),
      '\n'
    ),

    cross_reference: $ => token(/@(fig|tbl|eq|sec|lst)-[a-zA-Z0-9_-]+/),

    inline_code_cell: $ => seq(
      '`',
      '{', $.language_name, '}',
      $.cell_content,
      '`'
    )
  }
});
```

## Features from Zed Plan to Incorporate

The Zed extension plan highlights some important features we should ensure are covered:

### 1. Cell Attribute Blocks
```markdown
```{python}
#| label: fig-plot
#| echo: false
#| fig-cap: "My plot"
```
```
✅ Already in our plan as "chunk options"

### 2. Layout Directives
```markdown
::: {.column-page}
Content spans full page width
:::
```
✅ Already covered by pandoc-markdown divs

### 3. Shortcodes
```markdown
{{< video https://example.com/video.mp4 >}}
```
⬜ **Need to add** to our feature list

### 4. Execution Metadata
```markdown
```{python}
#| eval: true
#| warning: false
```
```
✅ Already in our plan as chunk options

## Updated Feature Priority

Based on Zed extension plan, our priorities should be:

**Phase 1: Foundation (Weeks 1-2)**
1. ✅ Executable code cells with language specifiers
2. ✅ Simple chunk options (single-line `#| key: value`)
3. ✅ Cross-reference distinction (`@fig-plot` vs `@citation`)
4. ✅ Inline code cells (`` `{python} expr` ``)

**Phase 2: Advanced Quarto (Weeks 3-4)**
1. ✅ Multi-line chunk option values
2. ✅ Callouts (already in pandoc-markdown, may need enhancement)
3. ⬜ **Shortcodes** (need to add)
4. ✅ Enhanced divs (already in pandoc-markdown)

**Phase 3: Tooling (Weeks 5-6)**
1. ✅ Injection queries for language-specific highlighting
2. ✅ Folding queries
3. ✅ Editor integration tests
4. ⬜ Chunk option validation queries

## Decision: Proceed with tree-sitter-pandoc-markdown Base

### Final Architecture

```
tree-sitter-quarto/
├── grammar.js                    # Extends pandoc-markdown
├── src/
│   └── scanner.c                 # External scanner for chunk options
├── queries/
│   ├── highlights.scm           # Quarto-specific highlighting
│   ├── injections.scm           # Language injection for cells
│   ├── folds.scm                # Code folding
│   └── locals.scm               # Local variable tracking
├── test/
│   └── corpus/                  # Test cases
│       ├── executable-cells.txt
│       ├── chunk-options.txt
│       ├── cross-references.txt
│       ├── inline-cells.txt
│       └── shortcodes.txt
└── examples/
    └── sample.qmd               # Real-world test document
```

### Dependency Chain

```
tree-sitter-markdown (CommonMark)
    ↓ extends
tree-sitter-pandoc-markdown (citations, divs, tables)
    ↓ extends
tree-sitter-quarto (executable cells, chunk options, cross-refs)
```

### Benefits of This Approach

1. **Build on existing work** - Don't re-implement Pandoc features
2. **Focus on Quarto** - Spend time on Quarto-specific features
3. **Maintain compatibility** - Pandoc features work correctly
4. **Benefit from improvements** - Get pandoc-markdown updates
5. **Proven architecture** - Split grammar approach is tested

## Action Items

1. ✅ Document relationship to zed-quarto-extension plan
2. ⬜ Add shortcodes to feature list
3. ⬜ Confirm chunk option external scanner strategy
4. ⬜ Review tree-sitter-pandoc-markdown's current state
5. ⬜ Begin grammar implementation extending pandoc-markdown

## Conclusion

While the Zed extension plan proposed extending tree-sitter-markdown directly, we should **extend tree-sitter-pandoc-markdown** instead because:

1. Quarto IS Pandoc Markdown + executable features
2. Pandoc features are already implemented correctly
3. Split grammar architecture is proven
4. We can focus on Quarto-specific additions

This approach aligns with the actual relationship between Quarto and Pandoc, avoids duplicate work, and allows us to focus on what makes Quarto unique.
