# Archived Migration Scripts

These scripts were used during the October 2025 migration from tree-sitter-pandoc-markdown to tree-sitter-quarto. They are no longer needed and preserved here for historical reference.

## Scripts

### install-dev.sh
**Purpose**: Clean build artifacts and Zed caches before installing dev extension

**Why archived**: Overly cautious. The standard workflow (build WASM + install in Zed) works fine without extensive cleanup. The grammars/ directory is only for tests and doesn't interfere with Zed's grammar compilation.

**Standard workflow now**:
```bash
cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/quarto_zed.wasm extension.wasm
# Then in Zed: Cmd+Shift+P -> "zed: install dev extension"
```

### fix-highlighting.sh
**Purpose**: Copy extension queries over grammar queries in grammars/ directory

**Why archived**: Obsolete workaround. tree-sitter-quarto now has Zed-compatible scopes (`@text.*`, `@emphasis.*`) built into the grammar by default. No patching needed.

### fix-zed-queries.sh
**Purpose**: Copy extension queries to Zed's work directory after installation

**Why archived**: Same reason as fix-highlighting.sh. The upstream grammar provides Zed-compatible scopes, so no post-installation patching is needed.

### patch-inline-grammar.sh
**Purpose**: Patch pandoc_markdown_inline grammar with Zed-compatible scopes

**Why archived**: Completely obsolete. References `grammars/pandoc_markdown_inline/` which no longer exists since we migrated to tree-sitter-quarto (single unified grammar).

## Historical Context

These scripts addressed the issue where:
1. Old setup used tree-sitter-pandoc-markdown with dual grammar (block + inline)
2. Grammar had `@markup.*` scopes (nvim-treesitter conventions)
3. Zed themes didn't support `@markup.*` scopes
4. Scripts patched grammar files to use `@text.*` scopes

**Solution**: We solved this upstream by modifying tree-sitter-quarto to use Zed-compatible scopes by default. The grammar provides both:
- `queries/highlights.scm` with Zed-compatible scopes (default)
- `queries/nvim/highlights.scm` with modern scopes for Neovim

This approach eliminated the need for extension-side workarounds.
