# Project Context

## Purpose

tree-sitter-quarto is a tree-sitter parser for Quarto Markdown (`.qmd` files), optimized for editor integration. It provides rich syntax trees that enable advanced editor features like:

- **Semantic syntax highlighting** - Distinct colors for chunk options, cross-references, executable cells
- **Jump-to-definition** - Navigate from `@fig-plot` to figure definition
- **Validation** - Catch typos in chunk options, undefined cross-references
- **Autocomplete** - Suggest valid chunk option names and values
- **Code folding** - Collapse executable cells and divs
- **Outline view** - Navigate document structure including cells

**Gap Being Filled:** This project bridges the gap between tree-sitter-pandoc-markdown (editor-focused but not Quarto-aware) and the Quarto Markdown Parser (rendering-focused, not optimized for pre-execution editor experience). It provides parse-time semantic information for Quarto documents as they're being authored, before execution.

**Current Status:** Alpha - All Core Features Implemented
- ✅ 122/122 tests passing (100%)
- ✅ 62/63 requirements implemented (98%) across 8 OpenSpec specifications
- ✅ CI/CD pipeline green (Ubuntu + macOS, Node 18.x + 20.x)
- ✅ All core Quarto features parsed (cells, chunk options, cross-refs, inline cells, shortcodes, enhanced divs)
- ✅ Enhanced divs implemented (9/11 requirements: callouts, tabsets, conditional content)
- ✅ Comprehensive test coverage (footnotes, inline attributes, pipe tables, test refinements)
- ⚠️ Known limitation: Generic fenced divs (`::: {.custom-class}`) not parsing (base grammar issue)
- ⏳ Editor integration pending

**Total Specifications:** 8 (all implemented)

## Tech Stack

- **JavaScript** - Grammar definition (`grammar.js`)
- **C** - External scanner for context-sensitive parsing (`src/scanner.c`)
- **Node.js** + **npm** - Build tooling and tree-sitter CLI
- **Tree-sitter** - Parser generator framework
- **Scheme** - Query files for syntax highlighting, injections, folding (`queries/*.scm`)

### Planned Language Support (Injection)

Executable cells will support language injection for:
- Python
- R
- Julia
- SQL
- Bash
- Others as needed

## Project Conventions

### Code Style

**File Naming:**
- Use lowercase filenames: `plan.md`, `todo.md`, `readme.md`
- Exceptions: `README.md`, `CONTRIBUTING.md`, `LICENSE` (conventional names)

**Grammar Rules (JavaScript):**
- Use descriptive rule names
- Add comments for complex rules
- Group related rules together
- Use `field()` for important nodes

**Scanner Code (C):**
- Follow existing C style conventions
- Add comments explaining logic
- Handle edge cases carefully
- Test boundary conditions

**Query Files (Scheme):**
- Use consistent indentation (2 spaces)
- Group related patterns
- Add comments for non-obvious patterns
- Test with real documents

### Architecture Patterns

**"Copy & Extend" Strategy:**
- Copy grammar from tree-sitter-pandoc-markdown into this repository
- Extend with Quarto-specific rules on top
- Maintain compatibility with base Pandoc features
- Document source commit hash for tracking

**Editor-First Design:**
- Parse raw `.qmd` files (before execution)
- Provide semantic nodes for all Quarto constructs
- Enable rich editor features (autocomplete, validation, navigation)

**Execution-Aware Parsing:**
- Distinguish executable cells from regular code blocks
- Parse chunk options as structured data
- Identify cell language and execution context

**External Scanner Usage:**
- Handle context-sensitive parsing beyond LR(1) capability
- Required tokens: `CHUNK_OPTION_MARKER` (for `#|` lines), `CELL_BOUNDARY`
- Distinguish `#| option` from `# comment` based on cell context

**Scope Naming Philosophy:**
- Use **standard tree-sitter scopes** (`@markup.*`, `@function`, `@property`)
- Remain editor-agnostic - same grammar works in Neovim, Helix, VSCode, Zed
- Editor extensions handle scope remapping (e.g., `@markup.heading` → `@text.title` for Zed)
- Separation of concerns: grammar = semantic parsing, extension = visual presentation
- Single source of truth: one `queries/highlights.scm` for all editors
- Reference: https://github.com/ck37/zed-quarto-extension/issues/4

### Testing Strategy

**Test Framework:** Tree-sitter corpus format in `test/corpus/*.txt`

**Test Categories:**
1. **Unit Tests** - Individual features (executable-cells.txt, chunk-options.txt, cross-references.txt)
2. **Edge Cases** - Boundary conditions and complex nesting
3. **Integration Tests** - Feature combinations and interactions
4. **Real-World Tests** - Parse actual Quarto documents from quarto-web repository

**Test Structure:**
```
==================
Test name
==================

Input markdown here

---

(expected_ast_structure)
```

**Testing Commands:**
- `npx tree-sitter test` - Run all tests
- `npx tree-sitter test -f <name>` - Run specific test file
- `npx tree-sitter parse examples/sample.qmd` - Parse example document
- `npx tree-sitter parse <file> --debug` - Debug parse tree

**Success Criteria:**
- ✅ All test cases passing (42/42 tests, 100%)
- ✅ 7 OpenSpec specifications verified (53/54 requirements, 98%)
- ✅ CI/CD pipeline passing on Ubuntu and macOS
- ⏳ Parse quarto-web without errors (not yet tested)
- ⏳ Performance: <100ms for typical documents (not yet measured)
- ⏳ Editor integration validated in 3+ editors (pending)

### Git Workflow

**Branching:**
- `main` - Production-ready code
- `feature/feature-name` - Feature branches

**Commit Message Format:**
```
type: brief description

Detailed explanation if needed.

Fixes #issue-number
```

**Commit Types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code restructuring
- `chore:` - Maintenance tasks

**Examples:**
- `feat: add shortcode parsing support`
- `fix: correct chunk option detection in scanner`
- `docs: update architecture decisions in plan.md`

**Important:** Do NOT co-sign commits as Claude

## Domain Context

### Quarto Markdown Ecosystem

**Quarto** is a scientific publishing system that extends Pandoc Markdown with:
- Executable code cells (Python, R, Julia, etc.) ✅ **Implemented**
- Chunk options for controlling cell behavior (`#| label: fig-plot`) ✅ **Implemented**
- Cross-references to figures, tables, equations (`@fig-plot`, `@tbl-data`) ✅ **Implemented**
- Inline code cells (`` `{python} expr` ``) ✅ **Implemented**
- Shortcodes (`{{< video url >}}`) ✅ **Implemented**
- Enhanced divs (callouts, tabsets, conditional content) ⏳ **Planned**

### Key Quarto Constructs

**Executable Code Cells:**
````markdown
```{python}
#| label: fig-plot
#| echo: false
import matplotlib.pyplot as plt
plt.plot([1, 2, 3])
```
````

**Cross-References vs Citations:**
- `@fig-plot` - Cross-reference to figure (type prefix: `fig`, `tbl`, `eq`, `sec`, `lst`)
- `@smith2020` - Citation (no type prefix)

**Chunk Options:**
- Format: `#| key: value`
- Must appear at start of cell content
- Support single-line and multi-line values with `|` continuation

**Inline Code Cells:**
- `` `{python} expr` `` - Inline execution with language injection

**Shortcodes:**
```markdown
{{< video https://example.com/video.mp4 >}}
{{< embed notebook.ipynb#fig-plot >}}
{{< include _content.qmd >}}
{{< var variable.name >}}
{{< meta title >}}
```
- Block-level: Standalone on their own line
- Inline: Within paragraph text
- Support common Quarto shortcodes: video, embed, include, var, meta

### Tree-sitter Parsing Context

**Parser Capabilities:**
- Incremental parsing for editor performance
- Error recovery for partial documents
- Language injection for multi-language support
- Query system for syntax highlighting, folding, navigation

**LR(1) Limitations:**
- Cannot handle unlimited lookahead
- Context-sensitive parsing requires external scanner
- Must carefully handle ambiguous constructs

### Sibling Project

**tree-sitter-pandoc-markdown** is the base grammar this project extends:
- Repository: https://github.com/ck37/tree-sitter-pandoc-markdown
- Provides: Pandoc Markdown features (citations, divs, spans, attributes)
- Missing: Quarto-specific features (chunk options, executable cell semantics)

## Important Constraints

### Technical Constraints

1. **LR(1) Parsing:** Tree-sitter uses LR(1) algorithm - context-sensitive features need external scanner
2. **External Scanner Required:** Must handle `#|` chunk option detection and cell boundary context
3. **Performance Target:** <100ms parsing time for typical documents (not yet measured)
4. **Compatibility:** Must maintain compatibility with base Pandoc Markdown features
5. **Editor Agnostic:** Uses standard tree-sitter scopes (`@markup.*`) - editor extensions handle remapping

### Design Constraints

1. **Editor-First Focus:** Parse raw `.qmd` files before execution (not post-execution markdown)
2. **No Submodules:** Use "Copy & Extend" strategy rather than git submodules for simplicity
3. **Semantic Precision:** Provide first-class nodes for all Quarto constructs (not generic fallbacks)
4. **Language Injection:** Support multiple language grammars injected into single document

### Implementation Constraints

1. **Validation Separation:** Grammar handles structure only; validation belongs in separate language server
2. **Manual Sync:** Must manually synchronize updates from tree-sitter-pandoc-markdown
3. **Documentation Version:** Track source commit hash from base grammar

## External Dependencies

### Core Dependencies

- **tree-sitter** (npm) - Parser generator framework
- **tree-sitter-cli** (npm) - Command-line tools for development
- **tree-sitter-pandoc-markdown** - Base grammar (copied, not npm dependency yet)
- **Node.js** (v16+) - Runtime for build tools
- **C compiler** - Required for compiling external scanner

### Runtime Dependencies

- Editor plugins (nvim-treesitter, Zed, Helix) will depend on compiled parser
- Language injection depends on availability of language-specific parsers (tree-sitter-python, tree-sitter-r, etc.)

### External Services

- **GitHub** - Repository hosting, issue tracking, CI/CD
- **Quarto** - Reference implementation for syntax specification
- **quarto-web** - Real-world test corpus for validation

### Related Systems

- **Quarto Markdown Parser** (Rust) - Rendering-focused parser, not competitive but complementary
- **tree-sitter-markdown** - Original upstream grammar (via tree-sitter-pandoc-markdown)
- **Pandoc** - Markdown processing system that defines much of the base syntax
