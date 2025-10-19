# Contributing to Zed Quarto Extension

Thank you for your interest in contributing! This guide covers development setup, testing, and common workflows.

## Development Commands

### Build and Test

```bash
# Run all tests (includes highlight coverage)
cargo test --workspace --all-features

# Build the extension for Zed (produces extension.wasm)
cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/quarto_zed.wasm extension.wasm
```

### Installing Dev Extension in Zed

**IMPORTANT**: Zed's grammar compiler conflicts with local development artifacts. Always clean before installing:

```bash
# Use the install script (recommended)
./install-dev.sh

# Or manually
cargo clean
rm -rf grammars/
cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/quarto_zed.wasm extension.wasm
# Then in Zed: Cmd+Shift+P -> "zed: install dev extension" -> select this directory
```

**Why this is needed**: The `grammars/` directory is created during local development (`cargo build`, `cargo test`) for native compilation. Zed compiles grammars independently from `extension.toml` and gets confused by these artifacts. The directory is in `.gitignore` and not committed.

### Grammar Updates

The pandoc-markdown grammar is fetched and compiled by `build.rs` at build time. To update to a new commit:

1. Update the `COMMIT` constant in `build.rs`
2. Update the `commit` field in `extension.toml` under `[grammars.pandoc_markdown]`
3. Run `cargo build` to fetch and compile the new grammar version

## Testing

The test suite validates syntax highlighting coverage and configuration.

### Highlight Coverage Test (`tests/highlights.rs`)

- Links to compiled pandoc-markdown grammar (native only)
- Parses fixture files from `tests/fixtures/`
- Runs tree-sitter-highlight with extension's queries
- Validates that highlighting produces output without errors
- Ensures grammar nodes are covered by highlight queries

### Quarto CLI Test (`tests/lsp_smoke.rs`)

- Verifies Quarto CLI is available (optional)
- Runs `quarto --version` if installed
- Skips gracefully if Quarto not installed
- Note: Despite the filename, this doesn't test LSP (Quarto has no LSP)

### Manifest Test (`tests/manifest.rs`)

- Validates `extension.toml` structure

## Common Development Workflows

### Adding Support for New Pandoc/Quarto Syntax

To add highlighting for new syntax constructs:

1. **Verify grammar support**: Check if `tree-sitter-pandoc-markdown` exposes the construct as a named node. If not, coordinate with grammar upstream.
2. **Add highlight query**: Edit `languages/quarto/highlights.scm` to map the new node type to an appropriate semantic scope.
3. **Test coverage**: Add a fixture file in `tests/fixtures/` demonstrating the syntax, then run `cargo test` to verify.

### Updating the Grammar

When upstream `tree-sitter-pandoc-markdown` adds new features:

1. Identify the commit hash to update to
2. Update `build.rs` COMMIT constant
3. Update `extension.toml` grammar commit
4. Run `cargo clean && cargo build` to fetch and compile
5. Update `highlights.scm` if new node types are exposed
6. Add test fixtures and validate with `cargo test`

### Debugging Highlighting Issues

1. **Check grammar parsing**: Use tree-sitter CLI to inspect parse tree: `tree-sitter parse file.qmd`
2. **Check highlight queries**: Use tree-sitter highlight: `tree-sitter highlight file.qmd --query-paths languages/quarto/highlights.scm`
3. **Check node types**: Run `tree-sitter parse file.qmd` and examine the AST for expected node names
4. **Validate query syntax**: Tree-sitter will error on invalid S-expression syntax in `.scm` files

## Project Architecture

### Extension Structure

**Extension Entry Point** (`src/lib.rs`):
- Implements `zed::Extension` trait (minimal implementation)
- No language server support (Quarto doesn't provide one)
- Extension provides only syntax highlighting via grammar

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
- Requires process:exec capability for spawning language server

### Highlighting Architecture

The extension uses tree-sitter queries to map parsed nodes to semantic highlight scopes:

1. `tree-sitter-pandoc-markdown` parses `.qmd` files and produces AST nodes like `(fenced_div)`, `(citation)`, `(attribute_list)`, `(yaml_front_matter)`
2. `highlights.scm` maps these nodes to semantic scopes like `@markup.raw.block`, `@string.special.symbol`, `@attribute`
3. Zed's theme applies colors based on these semantic scopes
4. `injections.scm` triggers nested parsing for embedded languages (e.g., Python code in fenced blocks)

**Key Scopes Used:**
- `@text.title` - Headings
- `@text.emphasis` - Italic text
- `@emphasis.strong` - Bold text
- `@text.literal` - Code spans and fenced code blocks
- `@text.uri` - Links
- `@text.reference` - Citations, cross-references, footnotes
- `@comment` - YAML front matter, chunk options
- `@constant.macro` - Shortcodes like `{{< include file.qmd >}}`
- `@property` - Attribute lists like `{.class #id}`
- `@punctuation.special` - Markers (headings, lists, block quotes, etc.)

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
│   ├── lsp_smoke.rs              # CLI availability
│   ├── manifest.rs               # Extension manifest validation
│   └── fixtures/                 # Test .qmd files
├── docs/                         # Documentation
└── README.md                     # User-facing documentation
```

## Building a Quarto Grammar

If you're interested in helping create a dedicated `tree-sitter-quarto` grammar, see [`docs/tree-sitter-quarto-plan.md`](docs/tree-sitter-quarto-plan.md) for implementation details. This would benefit the entire Quarto ecosystem across all editors.
