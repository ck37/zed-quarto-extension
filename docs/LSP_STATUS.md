# Quarto Language Server Status

## Why There's No LSP in This Extension

**Quarto does not provide a built-in language server.** Running `quarto language-server` returns an error because this command doesn't exist in the Quarto CLI.

> **Note**: This extension focuses on providing excellent syntax highlighting using [`tree-sitter-pandoc-markdown`](https://github.com/ck37/tree-sitter-pandoc-markdown). Phase 1 of the grammar is now complete, supporting most Pandoc markdown features that Quarto relies on.

## What is a Language Server?

A Language Server Protocol (LSP) implementation provides IDE features:
- **Code Completion**: Autocomplete for functions, variables, imports
- **Hover Documentation**: See docs/signatures when hovering over code
- **Go to Definition**: Jump to where symbols are defined
- **Diagnostics**: Real-time error checking and warnings
- **Code Actions**: Quick fixes, refactoring suggestions
- **Signature Help**: Parameter hints while typing function calls

## The Challenge for Quarto

Quarto documents (`.qmd` files) contain multiple languages:

```qmd
---
title: "My Document"  # YAML
---

# Introduction        # Markdown

```{python}           # Python
import pandas as pd
```

```{r}                # R
summary(data)
```
```

A "Quarto LSP" would need to:
1. Parse the document structure
2. Identify language boundaries
3. Route requests to Python LSP, R LSP, etc.
4. Handle YAML and Markdown separately
5. Coordinate results from multiple servers

This is extremely complex.

## Existing LSP Solutions

### 1. VS Code Extension (Official)
The [Quarto VS Code extension](https://marketplace.visualstudio.com/items?itemName=quarto.quarto) provides LSP-like features through a custom TypeScript implementation.

**Features:**
- YAML completion for front matter
- Markdown assistance
- Embedded language support via virtual documents

**Limitation:** VS Code specific, not a standalone server.

### 2. otter.nvim (Neovim)
[otter.nvim](https://github.com/jmbuhr/otter.nvim) is the most sophisticated solution:

**How it works:**
- Creates hidden buffers for each embedded language
- Runs separate LSP instances per language
- Routes requests from main buffer to appropriate hidden buffer
- Synchronizes changes between buffers

**Features:**
- Full LSP support for Python, R, Julia chunks
- Works with existing language servers
- Handles line number mapping

**Limitation:** Neovim-specific, uses Neovim's Lua API.

### 3. Could We Build One?

Building a standalone Quarto LSP would require:

**Architecture:**
```
User Editor (Zed)
    ↓
Quarto LSP Server
    ↓ (coordinates between)
    ├─> Python LSP (pyright)
    ├─> R LSP (r-languageserver)
    ├─> Julia LSP
    ├─> YAML LSP
    └─> Markdown LSP
```

**Complexity:**
- **Document Parsing**: ~200 hours - Parse .qmd, identify code chunks, track positions
- **Multi-LSP Coordination**: ~300 hours - Spawn/manage multiple LSP processes, route requests
- **Position Mapping**: ~200 hours - Map positions between main doc and virtual docs
- **Synchronization**: ~150 hours - Keep virtual documents in sync with edits
- **Configuration**: ~50 hours - Discover and configure language-specific LSPs
- **Testing**: ~100 hours - Comprehensive test suite

**Total Effort:** ~1000 hours (6 months full-time)

**Challenges:**
- Each embedded language needs its own LSP installed
- Complex position mapping (chunk line 5 ≠ document line 5)
- Handling incomplete/invalid chunks
- Performance with many embedded languages

## What This Extension Provides

✅ **Syntax Highlighting** via `tree-sitter-pandoc-markdown` (Phase 1 complete):
- **Core Markdown**: Headings, links, code blocks, lists, YAML front matter
- **Math**: Inline (`$...$`) and display (`$$...$$`) with LaTeX syntax
- **Tables**: Pipe tables with alignment markers
- **Footnotes**: References (`[^1]`) and inline footnotes (`^[text]`)
- **Pandoc extensions**: Fenced divs, citations, cross-references, shortcodes, attribute lists, raw blocks
- **Language injections**: Python, R, Julia, SQL chunks with syntax highlighting
- **Chunk options**: `#| echo: false` style options

⚠️ **Known Limitation**: Bold/italic highlighting currently not working due to Zed's grammar injection limitation. See [`bold-highlighting-investigation/`](bold-highlighting-investigation/) for details.

✅ **Document Outline** for navigation (headings structure)

## Future Possibilities

### Option 1: Wait for Official Support
The Quarto team may eventually provide a standalone LSP. Track [this GitHub issue](https://github.com/quarto-dev/quarto-cli/issues/239).

### Option 2: Adapt otter-ls
[otter-ls](https://github.com/jmbuhr/otter.nvim) could potentially be extracted from Neovim and made standalone. This would require:
- Removing Neovim dependencies
- Implementing generic LSP client communication
- Creating configuration system for language detection

This is still significant work (~300 hours) but more feasible than building from scratch.

### Option 3: Community Contribution
If there's strong demand, the Zed community could collaborate on a standalone implementation, potentially as a separate project that benefits all editors.

## Recommendations

**Using This Extension:**
1. ✅ **Excellent syntax highlighting** for Quarto/Pandoc markdown (Phase 1 complete)
2. ✅ **Language injections** provide syntax highlighting within code chunks
3. ⚠️ **No code intelligence** - For LSP features within code chunks, use:
   - **Python chunks**: Open in Python-aware editor with Pyright/pylance
   - **R chunks**: Use RStudio or Positron for R LSP features
   - **Julia chunks**: Use Julia-aware editor with Julia LSP
4. 💡 **Workflow tip**: Edit Quarto structure in Zed, edit complex code chunks in language-specific editor

**What Works Well:**
- Document structure and navigation
- Markdown formatting and organization
- Quarto-specific syntax (callouts, divs, citations, cross-references)
- Math and table editing
- Quick edits to code chunks (with syntax highlighting)

**What Requires External Tools:**
- Code completion within chunks
- Type checking and diagnostics for embedded languages
- Go-to-definition for functions used in chunks
- Refactoring tools for code within chunks
- Document rendering and preview (use `quarto preview` or VSCode)

**Contributing:**
If you're interested in LSP support for Quarto:
1. Star/watch [Quarto CLI Issue #239: Language Server](https://github.com/quarto-dev/quarto-cli/issues/239)
2. Consider contributing to [otter.nvim](https://github.com/jmbuhr/otter.nvim)'s standalone efforts
3. Reach out if you want to collaborate on a Zed-specific solution

## Related Resources

- [Quarto CLI Issue #239: Language Server](https://github.com/quarto-dev/quarto-cli/issues/239)
- [otter.nvim Repository](https://github.com/jmbuhr/otter.nvim)
- [Quarto VS Code Extension](https://github.com/quarto-dev/quarto/tree/main/apps/vscode)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
