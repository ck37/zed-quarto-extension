# Project Context

## Purpose
Provide Quarto (`.qmd`) syntax highlighting support for the Zed editor using tree-sitter grammars. This extension enables developers and data scientists to work with Quarto documents (computational notebooks combining Markdown, code, and scientific output) in Zed with proper syntax highlighting for Pandoc Markdown features and embedded code languages.

**Key Goal:** Bring Quarto support to Zed through tree-sitter grammar integration, prioritizing syntax highlighting quality and Pandoc Markdown compatibility.

## Tech Stack
- **Rust** - Extension implementation and build system
- **tree-sitter** - Grammar parsing and syntax highlighting
- **tree-sitter-pandoc-markdown** - Core Pandoc Markdown grammar (current)
- **tree-sitter-quarto** - Unified Quarto grammar (future migration)
- **Zed Extension API** (`zed_extension_api`) - Official extension interface
- **cc crate** - C grammar compilation during build
- **cargo** - Rust build toolchain

### Supporting Dependencies
- **which** - Executable location (native builds only)
- **tree-sitter-highlight** - Highlight testing
- **tree-sitter-markdown** - Base grammar for tests

## Project Conventions

### Code Style
- Use lowercase filenames across the project
- Rust code follows standard `rustfmt` formatting
- Tree-sitter query files (`.scm`) use consistent indentation
- Keep code comments focused on "why" not "what"

### Architecture Patterns
1. **Minimal Extension Implementation** - `src/lib.rs` provides minimal `zed::Extension` trait implementation (no LSP server)
2. **Build-time Grammar Compilation** - `build.rs` clones and compiles tree-sitter grammars from GitHub
3. **Query-based Highlighting** - Uses tree-sitter `.scm` query files to map AST nodes to semantic scopes
4. **Language Injection** - `injections.scm` enables nested syntax highlighting for 20+ embedded languages
5. **Scope Compatibility Layer** - Uses Zed's legacy scope names (`@text.*`, `@emphasis.*`) instead of modern nvim-treesitter conventions (`@markup.*`) because Zed's themes don't yet support newer scopes

**Key Architecture Decision:** Quarto documents are based on Pandoc Markdown, not standard Markdown. This extension uses `tree-sitter-pandoc-markdown` to provide Pandoc-aware syntax highlighting. Zed requires tree-sitter grammars; it cannot use TextMate grammars like VSCode does.

### Testing Strategy
- **Automated Tests** (`cargo test`) validate:
  - Highlight query syntax and coverage
  - Extension manifest structure
  - Grammar compilation
  - CLI tool availability (smoke tests)
- **Manual Testing** via `./install-dev.sh` for real-world usage
- **Test Fixtures** in `tests/fixtures/*.qmd` cover common Quarto patterns
- Always run `cargo test --workspace --all-features` before commits
- Clean install (`./install-dev.sh`) required to test extension changes in Zed

### Git Workflow
- **main branch** - Stable extension using dual grammar (pandoc_markdown + pandoc_markdown_inline)
- **tree-sitter-quarto-migration branch** - Experimental unified `tree-sitter-quarto` grammar (currently blocked by Zed query loading issues)
- **Commit conventions:**
  - Use lowercase, concise messages following existing style
  - **DO NOT include** `Co-Authored-By: Claude <noreply@anthropic.com>` trailers
  - **DO NOT include** "Generated with Claude Code" markers
- **Commit message format:** Standard semantic prefixes (feat:, fix:, docs:, test:, chore:, refactor:)

## Domain Context

### Quarto vs Standard Markdown
Quarto documents (`.qmd`) are **not** standard Markdown. They use Pandoc Markdown with extensions:
- **Fenced divs:** `:::` blocks with attributes like `{.callout-note}`
- **Citations:** `@smith2024` and `[@smith2024]` syntax
- **Cross-references:** `@fig-plot`, `@tbl-data`
- **Shortcodes:** `{{< include file.qmd >}}`
- **Attribute lists:** `{.class #id key=value}` on various elements
- **Extended YAML front matter:** Document metadata
- **Code chunks:** Executable code blocks with options (Python, R, Julia, etc.)

### Tree-sitter Highlighting Pipeline
1. Grammar parses `.qmd` files → produces AST nodes (`(fenced_div)`, `(citation)`, etc.)
2. `highlights.scm` maps nodes → semantic scopes (`@text.title`, `@constant`, etc.)
3. Zed theme applies colors based on semantic scopes
4. `injections.scm` triggers nested parsing for embedded languages

### Semantic Scope Naming
- `@text.title` - Headings
- `@text.emphasis` - Italic text
- `@emphasis.strong` - Bold text
- `@text.literal` - Code spans and fenced blocks
- `@text.uri` - Links
- `@constant` - Citations, cross-references, footnotes
- `@comment` - YAML front matter, chunk options
- `@constant.macro` - Shortcodes
- `@property` - Attribute lists
- `@punctuation.special` - Markdown markers (headings, lists, etc.)

### Language Server Status
**This extension does not provide language server support.** Quarto does not have a built-in language server. Users can configure external tools for linting/formatting if needed. See `docs/lsp-status.md` for details.

## Important Constraints

### Technical Constraints
1. **Zed Grammar Loading Behavior** - Zed loads grammar's built-in queries over extension overrides in some cases, blocking tree-sitter-quarto migration
2. **Scope Naming Compatibility** - Must use Zed's legacy scope names until theme support improves
3. **Build System Limitations** - Grammar compilation happens at build time; must skip WASM targets for C grammar compilation
4. **No TextMate Fallback** - Zed requires tree-sitter grammars; cannot use VSCode's TextMate grammar approach

### Known Limitations
1. **Bold/italic highlighting** - Partially working (~70% coverage) due to Zed's grammar injection limitations
2. **No preview/render support** - Use Quarto CLI or VSCode for visual preview
3. **Grammar coverage** - Some Pandoc edge cases not yet supported

### Platform Support
- Primarily developed and tested on macOS (darwin)
- Extension should work cross-platform, but testing focuses on macOS

## External Dependencies

### Grammar Sources

**tree-sitter-quarto-migration branch (current):**
- **Extension runtime:** tree-sitter-quarto
  - GitHub: ck37/tree-sitter-quarto
  - Commit: e9e22ece2b98c0838cbddf0b223a7bf54b42a295 (per extension.toml)
  - Unified grammar with built-in Quarto feature support
  - Currently blocked by Zed query loading issues (loads grammar's @markup.* queries instead of extension's @text.* queries)

- **Test compilation:** tree-sitter-pandoc-markdown (for native tests only)
  - GitHub: ck37/tree-sitter-pandoc-markdown
  - Branch: feat/phase-1-pandoc-grammar
  - Commit: f2e5718a1b2190cf59dd22d9a97fc9b7329a25b6
  - Provides dual grammar: block-level + inline-level
  - Uses Zed-compatible scopes directly (no runtime patching needed as of commit 4f184e2)

- **Test validation:** tree-sitter-quarto (for heading highlight tests)
  - Commit: b1b4cbd88fc6f787c660bf52b0e23879a8fc66c2
  - Grammar queries patched at build time to use extension's Zed-compatible queries

**main branch:**
- Uses tree-sitter-pandoc-markdown dual grammar (ck37 fork with Zed-compatible scopes)
- Stable and working implementation

### Build-time Dependencies
- Git (for cloning grammar repos)
- C compiler (for grammar compilation)
- Rust toolchain (cargo, rustc)

### Runtime Dependencies
- Zed editor (extension host)
- No external language servers or CLI tools required for basic functionality
