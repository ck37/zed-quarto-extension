# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Zed editor extension that provides Quarto (`.qmd`) support with syntax highlighting via tree-sitter-pandoc-markdown and language server integration via the Quarto CLI's built-in language server.

**Key Architecture Decision:** Quarto documents are based on Pandoc Markdown, not standard Markdown. This extension uses `tree-sitter-pandoc-markdown` (feat/phase-1-pandoc-grammar branch) to provide Pandoc-aware syntax highlighting. Zed requires tree-sitter grammars; it cannot use TextMate grammars like VSCode does.

## Development Commands

### Build and Test
```bash
# Run all tests (includes highlight coverage and LSP smoke tests)
cargo test --workspace --all-features

# Build the extension (produces extension.wasm)
cargo build --release

# Install as dev extension in Zed
# In Zed: Cmd+Shift+P -> "zed: install dev extension" -> select this directory
```

### Grammar Updates
The pandoc-markdown grammar is fetched and compiled by `build.rs` at build time. To update to a new commit:

1. Update the `COMMIT` constant in `build.rs`
2. Update the `commit` field in `extension.toml` under `[grammars.pandoc_markdown]`
3. Run `cargo build` to fetch and compile the new grammar version

### Git Commits
When creating commits for this repository:
- **Do NOT include** `Co-Authored-By: Claude <noreply@anthropic.com>` trailers
- Keep commit messages concise and descriptive
- Follow the existing commit style (see `git log` for examples)

## Architecture

### Extension Structure

**Extension Entry Point** (`src/lib.rs`):
- Implements `zed::Extension` trait
- `language_server_command()` resolves `quarto` CLI path and returns `quarto language-server` command
- Uses `which` crate on native platforms to locate Quarto CLI in PATH
- Falls back to simple "quarto" string on WASM

**Build System** (`build.rs`):
- Clones `tree-sitter-pandoc-markdown` from GitHub at specific commit
- Compiles C grammar using `cc` crate for native test builds only (skips WASM)
- Grammar source lives in `grammars/pandoc_markdown/tree-sitter-pandoc-markdown/`

**Language Configuration** (`languages/quarto/`):
- `config.toml`: Language metadata (file extension `.qmd`, comment syntax, tab settings)
- `highlights.scm`: Tree-sitter highlight queries mapping grammar nodes to semantic scopes
- `injections.scm`: Language injection rules for embedded code (Python, R, Julia, SQL in code chunks; YAML in front matter)
- `indents.scm`: Indentation rules
- `outline.scm`: Document outline/structure queries

**Extension Manifest** (`extension.toml`):
- Declares grammar source (repo URL, commit, path)
- Declares language server (command: `quarto language-server`)
- Requires process:exec capability for spawning language server

### Highlighting Architecture

The extension uses tree-sitter queries to map parsed nodes to semantic highlight scopes:

1. `tree-sitter-pandoc-markdown` parses `.qmd` files and produces AST nodes like `(fenced_div)`, `(citation)`, `(attribute_list)`, `(yaml_front_matter)`
2. `highlights.scm` maps these nodes to semantic scopes like `@markup.raw.block`, `@string.special.symbol`, `@attribute`
3. Zed's theme applies colors based on these semantic scopes
4. `injections.scm` triggers nested parsing for embedded languages (e.g., Python code in fenced blocks)

**Key Scopes Used:**
- `@text.title` - Headings
- `@text.emphasis` / `@text.strong` - Emphasis/bold (matches upstream Markdown for theme consistency)
- `@text.literal` - Code spans and fenced code blocks
- `@text.uri` - Links
- `@comment.documentation` - YAML front matter content
- `@markup.raw.block` - Fenced divs (Pandoc/Quarto-specific)
- `@attribute` - Attribute lists like `{.class #id}`
- `@string.special.symbol` - Citations and cross-references
- `@function.macro` - Shortcodes like `{{< include file.qmd >}}`

### Language Server Integration

The Quarto CLI provides a built-in language server (`quarto language-server`). The extension:
1. Locates `quarto` executable via PATH
2. Spawns `quarto language-server` as subprocess
3. Zed handles LSP communication (completions, diagnostics, hover, etc.)

No custom initialization options are passed to the language server.

## Testing

**Highlight Coverage Test** (`tests/highlights.rs`):
- Links to compiled pandoc-markdown grammar (native only)
- Parses fixture files from `tests/fixtures/`
- Runs tree-sitter-highlight with extension's queries
- Validates that highlighting produces output without errors
- Ensures grammar nodes are covered by highlight queries

**LSP Smoke Test** (`tests/lsp_smoke.rs`):
- Verifies Quarto CLI is discoverable on PATH
- Runs `quarto --version` to confirm installation
- Skips gracefully if Quarto not installed

**Manifest Test** (`tests/manifest.rs`):
- Validates `extension.toml` structure
- (Details not fully examined, but likely validates TOML parsing)

## Common Development Workflows

### Adding Support for New Pandoc/Quarto Syntax

To add highlighting for new syntax constructs:

1. **Verify grammar support:** Check if `tree-sitter-pandoc-markdown` exposes the construct as a named node. If not, coordinate with grammar upstream.
2. **Add highlight query:** Edit `languages/quarto/highlights.scm` to map the new node type to an appropriate semantic scope.
3. **Test coverage:** Add a fixture file in `tests/fixtures/` demonstrating the syntax, then run `cargo test` to verify.

### Updating the Grammar

When upstream `tree-sitter-pandoc-markdown` adds new features:

1. Identify the commit hash to update to
2. Update `build.rs` COMMIT constant
3. Update `extension.toml` grammar commit
4. Run `cargo clean && cargo build` to fetch and compile
5. Update `highlights.scm` if new node types are exposed
6. Add test fixtures and validate with `cargo test`

### Debugging Highlighting Issues

1. **Check grammar parsing:** Use tree-sitter CLI to inspect parse tree: `tree-sitter parse file.qmd`
2. **Check highlight queries:** Use tree-sitter highlight: `tree-sitter highlight file.qmd --query-paths languages/quarto/highlights.scm`
3. **Check node types:** Run `tree-sitter parse file.qmd` and examine the AST for expected node names
4. **Validate query syntax:** Tree-sitter will error on invalid S-expression syntax in `.scm` files

## Grammar Development Context

### Pandoc vs Standard Markdown

Quarto documents are **not** standard Markdown. They're Pandoc Markdown, which includes:
- **Fenced divs:** `:::` blocks with attributes like `{.callout-note}`
- **Citations:** `@smith2024` and `[@smith2024]` syntax
- **Cross-references:** `@fig-plot`, `@tbl-data`
- **Shortcodes:** `{{< include file.qmd >}}`
- **Attribute lists:** `{.class #id key=value}` on various elements
- **Extended YAML front matter:** More metadata than standard Markdown

The extension currently uses `tree-sitter-pandoc-markdown` (Phase 1), which supports core Pandoc features Quarto relies on. See `grammar-feature-needs.md` for missing Pandoc features (math, tables, footnotes, etc.) still being developed upstream.

### Future: tree-sitter-quarto

Long-term plan is to develop a dedicated `tree-sitter-quarto` grammar that extends pandoc-markdown with Quarto-only syntax:
- Executable chunk option lines: `#| echo: false`
- Cell attribute blocks and layout directives
- Quarto-specific shortcodes and publishing directives
- Execution metadata and YAML overrides

This is outlined in README.md "Planned: tree-sitter-quarto Grammar" section. For now, the extension focuses on strengthening Pandoc support.

## Known Limitations

- **No preview/render support:** Use Quarto CLI or VSCode for visual preview
- **Grammar coverage:** Some Pandoc edge cases not yet supported (see `grammar-feature-needs.md`)
- **No tree-sitter-quarto:** Currently using Pandoc grammar as closest approximation
- **WASM build:** Grammar is only compiled for native test builds, not WASM (grammar is bundled separately for Zed runtime)

## Dependencies

- **Zed Extension API** (`zed_extension_api`): Official API for Zed extensions
- **which** (native only): Locates executables in PATH
- **cc** (build): Compiles C tree-sitter grammar
- **tree-sitter** + **tree-sitter-highlight** (tests): Validates highlighting
- **tree-sitter-markdown** (tests): Base grammar dependency for tests

## File Organization

```
.
├── src/lib.rs                    # Extension implementation
├── build.rs                      # Grammar fetch and compilation
├── extension.toml                # Extension manifest
├── Cargo.toml                    # Rust dependencies
├── languages/quarto/             # Language configuration
│   ├── config.toml               # Language settings
│   ├── highlights.scm            # Syntax highlighting
│   ├── injections.scm            # Language injections
│   ├── indents.scm               # Indentation rules
│   └── outline.scm               # Document outline
├── tests/                        # Automated tests
│   ├── highlights.rs             # Highlight coverage
│   ├── lsp_smoke.rs              # LSP availability
│   ├── manifest.rs               # Extension manifest validation
│   └── fixtures/                 # Test .qmd files
├── grammars/pandoc_markdown/     # Grammar source (git-managed by build.rs)
└── README.md                     # User-facing documentation
```
