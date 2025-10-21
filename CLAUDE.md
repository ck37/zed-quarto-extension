<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Zed editor extension that provides Quarto (`.qmd`) support with syntax highlighting via tree-sitter-quarto. Note: Quarto does not have a built-in language server (see `docs/lsp-status.md` for details).

**Key Architecture Decision:** Quarto documents are based on Pandoc Markdown, not standard Markdown. This extension uses `tree-sitter-quarto` to provide comprehensive Quarto-aware syntax highlighting. Zed requires tree-sitter grammars; it cannot use TextMate grammars like VSCode does.

## Quick Commands

```bash
# Build and test
cargo test --workspace --all-features
cargo build --release --target wasm32-wasip2

# Install dev extension
cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/quarto_zed.wasm extension.wasm
# Then in Zed: Cmd+Shift+P -> "zed: install dev extension"
```

**For detailed workflows, debugging, and testing**: See `CONTRIBUTING.md`

### Git Commits
- **Do NOT include** `Co-Authored-By: Claude <noreply@anthropic.com>` trailers
- Keep messages concise and follow existing style

### GitHub Issues
- **NEVER** create issues in `zed-industries/extensions` unless explicitly approved by the user
- Use `ck37/zed-quarto-extension` for extension-specific issues
- Use `ck37/tree-sitter-quarto` for grammar-related issues

## Claude Code Preferences
- **Documentation files**: Create in `docs/` directory, NOT in `/tmp`
- Analysis, investigation, and debugging documents belong in `docs/`

## Architecture

### Extension Structure

**Extension Entry Point** (`src/lib.rs`):
- Implements `zed::Extension` trait (minimal implementation)
- No language server support (Quarto doesn't provide one)
- Extension provides only syntax highlighting via grammar

**Build System** (`build.rs`):
- Clones `tree-sitter-quarto` from GitHub at specific commit for tests
- Compiles C grammar using `cc` crate for native test builds only (skips WASM)
- Grammar source lives in `grammars/quarto/`

**Language Configuration** (`languages/quarto/`):
- `config.toml`: Language metadata (file extension `.qmd`, comment syntax, tab settings)
- `highlights.scm`: Tree-sitter highlight queries mapping grammar nodes to semantic scopes
- `injections.scm`: Language injection rules for embedded code (Python, R, Julia, SQL in code chunks; YAML in front matter)
- `indents.scm`: Indentation rules
- `outline.scm`: Document outline/structure queries

**Extension Manifest** (`extension.toml`):
- Declares grammar: `quarto` with repo URL and commit hash
- Zed fetches and compiles grammar from GitHub

### Highlighting Architecture

The extension uses tree-sitter queries to map parsed nodes to semantic highlight scopes:

1. `tree-sitter-quarto` parses `.qmd` files and produces AST nodes like `(fenced_div)`, `(citation)`, `(attribute_list)`, `(yaml_front_matter)`
2. `highlights.scm` maps these nodes to semantic scopes like `@text.title`, `@text.emphasis`, `@text.literal`
3. Zed's theme applies colors based on these semantic scopes
4. `injections.scm` triggers nested parsing for embedded languages (e.g., Python code in fenced blocks)

**Scope Naming Convention:**
This extension uses **Zed's legacy scope names** (`@text.*`, `@emphasis.*`) instead of modern nvim-treesitter conventions (`@markup.*`) because Zed's themes don't yet support the newer scopes. The tree-sitter-quarto grammar provides Zed-compatible scopes by default. See `docs/scope-naming-decision.md` for the full rationale and future migration path.

**Key Scopes Used:**
- `@text.title` - Headings
- `@text.emphasis` - Italic text
- `@emphasis.strong` - Bold text
- `@text.literal` - Code spans and fenced code blocks
- `@text.uri` - Links
- `@constant` - Citations, cross-references, footnotes
- `@comment` - YAML front matter, chunk options
- `@constant.macro` - Shortcodes like `{{< include file.qmd >}}`
- `@property` - Attribute lists like `{.class #id}`
- `@punctuation.special` - Markers (headings, lists, block quotes, etc.)

### Language Server Support

**This extension does not provide language server support.** Quarto does not have a built-in language server. See `docs/lsp-status.md` for details on existing solutions and future possibilities.

## Grammar Development Context

### Pandoc vs Standard Markdown

Quarto documents are **not** standard Markdown. They're Pandoc Markdown, which includes:
- **Fenced divs:** `:::` blocks with attributes like `{.callout-note}`
- **Citations:** `@smith2024` and `[@smith2024]` syntax
- **Cross-references:** `@fig-plot`, `@tbl-data`
- **Shortcodes:** `{{< include file.qmd >}}`
- **Attribute lists:** `{.class #id key=value}` on various elements
- **Extended YAML front matter:** More metadata than standard Markdown

The extension uses `tree-sitter-quarto`, which supports Quarto-specific syntax, Pandoc extensions, and core Markdown features. See `docs/grammar-feature-needs.md` for feature coverage and `docs/grammar-roadmap.md` for development history.

## Known Limitations

- **Some Pandoc extensions don't highlight:** Strikethrough, highlight, subscript, superscript parse correctly but don't highlight due to Zed theme limitations. See `docs/pandoc-extensions-scope-issue.md` for details.
- **No preview/render support:** Use Quarto CLI or VSCode for visual preview
- **Grammar coverage:** Some Pandoc edge cases not yet supported (see `docs/grammar-feature-needs.md`)

## Grammar Repository

- **tree-sitter-quarto**: https://github.com/ck37/tree-sitter-quarto
- Currently at commit: `4012bc7d9930654c81f1ade1d2070e0b951aa689`
- See `extension.toml` for the exact commit being used

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
├── CONTRIBUTING.md               # Development workflows and testing
├── languages/
│   └── quarto/                   # Quarto language configuration
│       ├── config.toml           # Language settings
│       ├── highlights.scm        # Syntax highlighting (Zed-compatible scopes)
│       ├── injections.scm        # Language injections (Quarto + 20+ languages)
│       ├── indents.scm           # Indentation rules
│       ├── outline.scm           # Document outline
│       ├── folds.scm             # Code folding
│       ├── tags.scm              # Symbol navigation
│       ├── locals.scm            # Local scope support
│       └── textobjects.scm       # Text object selection
├── tests/                        # Automated tests
│   ├── highlights.rs             # Highlight coverage
│   ├── lsp_smoke.rs              # CLI availability
│   ├── manifest.rs               # Manifest validation
│   └── fixtures/                 # Test .qmd files
├── docs/                         # Technical documentation
│   ├── lsp-status.md             # Language server details
│   ├── scope-naming-decision.md  # Why we use Zed's legacy scopes
│   ├── grammar-feature-needs.md  # Feature coverage
│   ├── grammar-roadmap.md        # Development phases
│   ├── syntax-highlighting-architecture.md
│   ├── tree-sitter-quarto-plan.md
│   ├── pandoc-extensions-scope-issue.md
│   └── archive/migration-2025-10/  # Historical migration documentation
├── grammars/                     # Grammar artifacts (gitignored)
└── README.md                     # User-facing documentation
```
