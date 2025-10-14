# Comparison with Other Parsers

This document provides detailed comparisons between tree-sitter-quarto and related parsing projects.

## Parser Ecosystem

```
tree-sitter-quarto
    ↓ extends
tree-sitter-pandoc-markdown
    ↓ fork of
tree-sitter-markdown
```

## Overview

The Quarto ecosystem has **two tree-sitter grammar implementations**:

1. **tree-sitter-quarto** (this project) - Production-ready for editor integration (2025)
2. **quarto-markdown tree-sitter grammars** - Official but experimental, planned for production in early 2026

**Why two parsers?** tree-sitter-quarto was created to provide **immediate, production-ready** editor support for Quarto Markdown, filling the gap until the official quarto-markdown tree-sitter grammars are production-ready. The official grammars are intentionally tightly coupled with quarto-markdown-pandoc and are not yet recommended for external use.

**Which should you use?**
- **Now (2025):** Use tree-sitter-quarto for editor integration
- **Future (2026+):** Consider migrating to official quarto-markdown grammars when they reach production status

Both approaches are valid, and the Quarto team (@cscheid) is supportive of community efforts while they work toward official support.

## Quick Comparison Table

| Feature | tree-sitter-quarto | quarto-markdown (tree-sitter) | quarto-markdown (pandoc) | tree-sitter-pandoc-markdown |
|---------|-------------------|-------------------------------|--------------------------|----------------------------|
| **Parses chunk options** | ✅ As structured data | ✅ As structured data | ❌ Handled by knitr | ❌ Not Quarto-aware |
| **Distinguishes xrefs from citations** | ✅ Semantic nodes | ✅ Semantic nodes | ❌ Both as citations | ✅ Via Pandoc features |
| **Recognizes executable cells** | ✅ First-class nodes | ✅ First-class nodes | ⚠️ As code blocks | ❌ Not supported |
| **Callout semantic parsing** | ✅ Specific node types | ✅ Specific node types | ⚠️ Generic divs | ⚠️ Generic divs |
| **Primary use case** | **Editor integration** | **Editor integration** | **Document rendering** | **Pandoc editing** |
| **Grammar type** | Unified | Dual (block + inline) | Pulldown-cmark | Dual (block + inline) |
| **Output format** | tree-sitter AST | tree-sitter AST | Pandoc AST | tree-sitter AST |
| **Language** | JavaScript + C | Rust + C | Rust | JavaScript + C |
| **Status** | Alpha (functional) | Experimental (pre-production) | Experimental | Production |
| **Production-ready** | ✅ Yes (2025) | ⏳ Planned (early 2026) | ⏳ In development | ✅ Yes |

## Detailed Comparison

### tree-sitter-quarto (This Project)

**Purpose:** Editor integration and tooling for Quarto Markdown authoring

**Strengths:**
- ✅ Parses `.qmd` files **before execution** (what you type in your editor)
- ✅ Rich semantic AST for editor features (syntax highlighting, navigation, autocomplete)
- ✅ First-class nodes for Quarto constructs (chunk options, executable cells, cross-references)
- ✅ Incremental parsing (fast updates as you type)
- ✅ Multi-language injection (Python, R, Julia syntax highlighting)
- ✅ Comprehensive query files for syntax highlighting and folding

**Limitations:**
- ⚠️ Not designed for rendering (use Quarto Parser for that)
- ⚠️ Generic fenced divs limitation (see [technical analysis](./generic-fenced-div-limitation.md))
- ⚠️ No validation (requires language server)

**Best for:**
- Editor plugins (Neovim, Zed, Helix, VSCode)
- Syntax highlighting during authoring
- Pre-execution tooling (linters, formatters)
- Jump-to-definition and navigation
- Autocomplete and code intelligence

### quarto-markdown (Official Tree-sitter Grammars)

**Repository:** https://github.com/quarto-dev/quarto-markdown

**Grammars:**
- Block: https://github.com/quarto-dev/quarto-markdown/tree/main/crates/tree-sitter-qmd/tree-sitter-markdown
- Inline: https://github.com/quarto-dev/quarto-markdown/tree/main/crates/tree-sitter-qmd/tree-sitter-markdown-inline

**Purpose:** Official tree-sitter grammars for Quarto Markdown, separate from the Pandoc AST conversion

**Strengths:**
- ✅ Official grammars from Quarto team (maintained by @cscheid)
- ✅ Pure tree-sitter implementation (can be used independently)
- ✅ Dual grammar architecture (block + inline) like tree-sitter-pandoc-markdown
- ✅ Rust + C implementation (fast, safe)
- ✅ Designed to work with quarto-cli (planned for early 2026)
- ✅ Will be "blessed" frontend parser for Markdown in Posit products
- ✅ Authors willing to help navigate the grammars

**Limitations:**
- ⚠️ **Not production-ready yet** (planned for early 2026)
- ⚠️ Experimental status (opt-in in quarto-cli initially)
- ⚠️ **No PRs accepted for grammar changes** (must be controlled by Quarto project)
- ⚠️ Deep dependency with quarto-markdown-pandoc (intentional design)
- ⚠️ Dual grammar complexity (block/inline interplay)
- ⚠️ No tree-sitter query files yet
- ⚠️ Documentation minimal (pre-production)

**Best for:**
- Future editor integration (when production-ready in 2026)
- Projects that can wait for official support
- Integrating with future Quarto tooling
- Alignment with Quarto project roadmap

**Current status:**
- Available in repository but **not recommended for production use**
- Team will take bug reports seriously
- Intended for use in quarto-cli starting early 2026
- Migration expected to take a long time

**Contact:** @cscheid (author) is happy to help navigate the grammars

### quarto-markdown (Pandoc Parser)

**Repository:** https://github.com/quarto-dev/quarto-markdown (quarto-markdown-pandoc crate)

**Purpose:** Document rendering and compilation to Pandoc AST

**Strengths:**
- ✅ Produces Pandoc AST for filter pipeline
- ✅ Integrated with Quarto rendering engine
- ✅ Handles post-execution markdown (after knitr/jupyter runs)

**Limitations:**
- ⚠️ Experimental status
- ⚠️ Parses **after** code execution (post-knitr output)
- ⚠️ Not designed for editor integration
- ⚠️ Chunk options handled by knitr (not parsed as first-class constructs)

**Best for:**
- Document rendering pipeline
- Pandoc filter development
- Post-execution markdown processing

### tree-sitter-pandoc-markdown (Base Grammar)

**Repository:** https://github.com/ck37/tree-sitter-pandoc-markdown

**Purpose:** Editor-focused parser for Pandoc Markdown

**Strengths:**
- ✅ Production-ready parser for Pandoc Markdown
- ✅ Rich semantic nodes (citations, divs, spans, attributes)
- ✅ Comprehensive query files
- ✅ Battle-tested in editors
- ✅ Handles complex Pandoc features (footnotes, definition lists, etc.)

**Limitations:**
- ⚠️ Not Quarto-aware (no chunk options, no executable cell semantics)
- ⚠️ Doesn't distinguish cross-references from citations
- ⚠️ No language injection for executable cells

**Best for:**
- Editing Pandoc Markdown (non-Quarto)
- Base grammar to extend for other formats
- Production use where Quarto features not needed

**Relationship to tree-sitter-quarto:**
- tree-sitter-quarto **extends** tree-sitter-pandoc-markdown
- Uses "Copy & Extend" strategy (not git submodules)
- Adds Quarto-specific features on top of Pandoc base

## Use Case Decision Matrix

### Choose tree-sitter-quarto when:

- ✅ Building editor plugins/extensions **today** (production-ready)
- ✅ Need syntax highlighting for `.qmd` files now
- ✅ Want to parse **before** code execution
- ✅ Need semantic understanding of Quarto constructs
- ✅ Building autocomplete/navigation features
- ✅ Creating linters or formatters
- ✅ Can't wait until 2026 for official tree-sitter support
- ✅ Prefer unified grammar architecture (simpler)
- ✅ Want comprehensive query files included

### Choose quarto-markdown tree-sitter grammars when:

- ✅ Can wait until early 2026 for production-ready support
- ✅ Want official Quarto project grammars
- ✅ Building for long-term alignment with Quarto ecosystem
- ✅ Need dual grammar architecture (block + inline)
- ✅ Comfortable with experimental/pre-production tools
- ✅ Want to contribute bug reports to official project
- ✅ Don't need query files immediately (can write your own)

### Choose quarto-markdown Pandoc parser when:

- ✅ Building rendering pipeline tools
- ✅ Processing **after** code execution
- ✅ Need Pandoc AST output
- ✅ Developing Quarto filters
- ✅ Integrating with Quarto CLI

### Choose tree-sitter-pandoc-markdown when:

- ✅ Working with Pandoc Markdown (non-Quarto)
- ✅ Need production-ready parser today
- ✅ Quarto-specific features not required
- ✅ Building general Pandoc tooling

## Architectural Differences

### Parsing Phases

#### tree-sitter-quarto (Pre-execution)

```
.qmd file
   ↓
[tree-sitter-quarto] ← Parses raw .qmd
   ↓
tree-sitter AST (semantic nodes for editor)
```

**Input example:**
````markdown
```{python}
#| label: fig-plot
import matplotlib.pyplot as plt
plt.plot([1, 2, 3])
```
````

**Output:** AST with `executable_code_cell`, `chunk_options` nodes

#### Quarto Parser (Post-execution)

```
.qmd file
   ↓
[knitr/jupyter] ← Executes code
   ↓
.md file (with outputs)
   ↓
[Quarto Parser] ← Parses rendered markdown
   ↓
Pandoc AST (for rendering)
```

**Input example:**
```markdown
![](figure.png)

<div class="output">
...
</div>
```

**Output:** Pandoc AST with images, divs, etc.

### Grammar Type

| Parser | Type | Language | ABI |
|--------|------|----------|-----|
| tree-sitter-quarto | LR(1) | JavaScript + C | tree-sitter |
| Quarto Parser | Pulldown-cmark | Rust | N/A |
| tree-sitter-pandoc-markdown | LR(1) | JavaScript + C | tree-sitter |

## Complementary, Not Competitive

**Important:** These parsers serve **different purposes** in the Quarto ecosystem:

```
┌─────────────────────────────────────────────────────┐
│ Authoring Phase (in editor)                        │
│                                                     │
│  .qmd file → [tree-sitter-quarto] → Editor features│
│                                                     │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│ Execution Phase (Quarto CLI)                       │
│                                                     │
│  .qmd → [knitr/jupyter] → .md                      │
│                                                     │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│ Rendering Phase (Quarto CLI)                       │
│                                                     │
│  .md → [Quarto Parser] → Pandoc AST → HTML/PDF    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**You can use both:**
- **tree-sitter-quarto** in your editor for authoring
- **Quarto Parser** in Quarto CLI for rendering

## Feature Coverage Comparison

### Executable Code Cells

| Feature | tree-sitter-quarto | quarto-markdown (tree-sitter) | quarto-markdown (pandoc) |
|---------|-------------------|-------------------------------|--------------------------|
| Recognize `{python}` syntax | ✅ | ✅ | ⚠️ (as code block) |
| Parse chunk options | ✅ | ✅ | ❌ (knitr does this) |
| Semantic `chunk_option` nodes | ✅ | ✅ | ❌ |
| Language injection | ✅ | ✅ (with queries) | N/A |
| Query files included | ✅ | ⚠️ (not yet) | N/A |

### Cross-References

| Feature | tree-sitter-quarto | quarto-markdown (tree-sitter) | quarto-markdown (pandoc) |
|---------|-------------------|-------------------------------|--------------------------|
| Parse `@fig-plot` | ✅ | ✅ | ✅ |
| Distinguish from `@author2020` | ✅ | ✅ | ❌ |
| Semantic `cross_reference` node | ✅ | ✅ | ❌ (both as citations) |
| Extract type prefix | ✅ | ✅ | ❌ |

### Enhanced Divs

| Feature | tree-sitter-quarto | quarto-markdown (tree-sitter) | quarto-markdown (pandoc) |
|---------|-------------------|-------------------------------|--------------------------|
| Parse callout blocks | ✅ Semantic nodes | ✅ Semantic nodes | ⚠️ Generic divs |
| Parse tabsets | ✅ Semantic nodes | ✅ Semantic nodes | ⚠️ Generic divs |
| Parse conditional content | ✅ Semantic nodes | ✅ Semantic nodes | ⚠️ Generic divs |
| Attribute parsing | ✅ Captured in tokens | ✅ Full parsing | ✅ Full parsing |

### Shortcodes

| Feature | tree-sitter-quarto | quarto-markdown (tree-sitter) | quarto-markdown (pandoc) |
|---------|-------------------|-------------------------------|--------------------------|
| Parse `{{< video >}}` | ✅ | ✅ | ✅ |
| Block vs inline distinction | ✅ | ✅ | ✅ |
| Semantic nodes | ✅ | ✅ | ✅ |

## Performance Characteristics

### tree-sitter-quarto

- **Parsing speed:** O(n) linear time (LR(1))
- **Incremental parsing:** ✅ Yes (only re-parses changed sections)
- **Target latency:** <100ms for typical documents
- **Memory usage:** Low (streaming parser)
- **Best for:** Interactive editing with live updates

### quarto-markdown tree-sitter grammars

- **Parsing speed:** Fast (Rust + C implementation)
- **Incremental parsing:** ✅ Yes (tree-sitter)
- **Target latency:** <100ms for typical documents
- **Memory usage:** Low (streaming parser)
- **Best for:** Interactive editing with live updates (when production-ready)

### quarto-markdown Pandoc parser

- **Parsing speed:** Fast (Rust implementation)
- **Incremental parsing:** ❌ No (batch processing)
- **Target latency:** Not critical (offline rendering)
- **Memory usage:** Moderate (full AST in memory)
- **Best for:** Batch rendering of documents

## Relationship to Official Quarto Project

**Acknowledgment:** Thanks to @cscheid (Carlos Scheidegger, author of quarto-markdown) for clarifying the architecture of quarto-markdown and the existence of the official tree-sitter grammars (see [Zed issue #12406](https://github.com/zed-industries/zed/issues/12406#issuecomment-3402303659)).

**Key points:**
- The official quarto-markdown repository **does contain pure tree-sitter grammars** (block + inline)
- These are separate from the tree-sitter-to-pandoc-AST conversion (quarto-markdown-pandoc crate)
- The grammars can be consumed independently of the Pandoc conversion
- **The official grammars ARE intended for editor integration** - they will be the "blessed front-end parser for Markdown in other Posit products" including RStudio, Positron, and future Posit IDEs
- They are intentionally tightly coupled with quarto-markdown-pandoc for release coordination
- The official grammars are **not yet production-ready** but are on the roadmap for early 2026

**tree-sitter-quarto's role:**
- **Necessary bridge solution** that fills the 1-year gap while official grammars mature
- Provides **immediate production-ready support** (2025) while official grammars stabilize
- Uses unified grammar architecture (simpler for some use cases)
- Includes comprehensive query files for syntax highlighting
- Designed specifically for editor integration
- Community-maintained with different architectural choices

**Both projects serve the same goal (editor integration):**
- tree-sitter-quarto serves immediate needs (2025)
- quarto-markdown grammars will be the blessed official solution (2026+)
- **Migration path recommended** when official grammars reach production status
- Quarto team is supportive of community efforts filling the gap

**Why migrate to official grammars in 2026+:**
- Battle-tested in Posit's production editors (RStudio, Positron)
- Official support and long-term maintenance guaranteed
- The "blessed" standard across Quarto/Posit ecosystem
- Better ecosystem alignment as adoption grows
- Will have comprehensive query files (needed for their editors)

**Contribution policy:**
- ❌ PRs to grammars not accepted (must be controlled by Quarto project for release coordination)
- ✅ Bug reports welcome and taken seriously
- ✅ Authors happy to help navigate the grammars
- ✅ External use explicitly encouraged

**Contact:** The quarto-markdown maintainers welcome bug reports and are happy to help navigate their grammars for those interested in using them.

## Related Documentation

- **Architecture deep-dive:** [plan.md](./plan.md)
- **Relationship to Quarto Parser:** [relationship-to-quarto-markdown.md](./relationship-to-quarto-markdown.md)
- **Editor integration:** [editor-integration.md](./editor-integration.md)
- **Technical limitations:** [generic-fenced-div-limitation.md](./generic-fenced-div-limitation.md)

## References

### Related Projects

- **tree-sitter-pandoc-markdown:** https://github.com/ck37/tree-sitter-pandoc-markdown
- **Quarto Markdown Parser:** https://github.com/quarto-dev/quarto-markdown
- **tree-sitter-markdown:** https://github.com/tree-sitter-grammars/tree-sitter-markdown
- **Quarto:** https://quarto.org/

### Further Reading

- **Quarto Documentation:** https://quarto.org/docs/
- **Tree-sitter Documentation:** https://tree-sitter.github.io/tree-sitter/
- **Pandoc Documentation:** https://pandoc.org/
