# Next Steps: Debugging No Syntax Highlighting

## Current Status

Extension installed successfully but showing no syntax highlighting in Zed.

## Branch Information

- **Current branch**: `tree-sitter-quarto-migration`
- **Last commit**: `52c8594` - feat: vendor tree-sitter-quarto grammar and add comprehensive test coverage
- **Grammar**: tree-sitter-quarto commit `c9f444e` (vendored in `grammars/quarto/`)
- **Queries**: Using `grammars/quarto/queries/zed/highlights.scm` for Zed compatibility

## Investigation Steps

### 1. Verify Extension Installation

```bash
# Check if extension is installed
ls -la ~/Library/Application\ Support/Zed/extensions/installed/quarto
ls -la ~/Library/Application\ Support/Zed/extensions/work/quarto

# Check Zed logs
tail -f ~/Library/Logs/Zed/Zed.log
```

### 2. Check Extension Manifest

The extension needs to tell Zed where the grammar is located. Check `extension.toml`:

```bash
cat extension.toml
```

**Expected content:**
```toml
[grammars.quarto]
path = "grammars/quarto"
```

**Problem**: Zed may not know to use `queries/zed/highlights.scm` instead of the default `queries/highlights.scm`.

### 3. Possible Issues

#### Issue A: Grammar Path Configuration
The extension.toml uses `path = "grammars/quarto"` which means Zed will look for queries at:
- `grammars/quarto/queries/highlights.scm` (standard nvim-treesitter scopes)

But we need it to use:
- `grammars/quarto/queries/zed/highlights.scm` (Zed-compatible scopes)

**Potential Fix**: Check if Zed supports editor-specific query directories. If not, we may need to:
1. Copy `queries/zed/highlights.scm` to `queries/highlights.scm`
2. OR patch the grammar's default highlights.scm with Zed scopes
3. OR use a build.rs step to copy Zed queries over default queries

#### Issue B: Extension Not Loading Queries
Zed might be using the grammar's built-in queries instead of extension-provided queries.

**Test**: Check what queries are actually in the installed extension:
```bash
ls -la ~/Library/Application\ Support/Zed/extensions/work/quarto/grammars/quarto/queries/
```

#### Issue C: WASM Build Missing Queries
The WASM build might not include the query files.

**Test**: Check if queries are embedded in the WASM:
```bash
ls -la target/wasm32-wasip2/release/*.wasm
```

### 4. Quick Tests

#### Test 1: Check Grammar Location
```bash
# See if grammar was compiled and installed
find ~/Library/Application\ Support/Zed/extensions -name "*.wasm" -path "*/quarto/*"
```

#### Test 2: Verify File Association
Open a `.qmd` file in Zed and check:
1. Bottom right corner - does it say "Quarto" as the language?
2. Command palette → "Select Language" → Is "Quarto" available?

#### Test 3: Check Zed's Extension API
```bash
# Look at what the extension API provides
grep -r "queries" ~/Library/Application\ Support/Zed/extensions/work/quarto/
```

### 5. Likely Solution Approaches

#### Approach A: Copy Zed Queries to Default Location
Modify `grammars/quarto/queries/highlights.scm` to match `queries/zed/highlights.scm`:

```bash
cd grammars/quarto
cp queries/zed/highlights.scm queries/highlights.scm
git add queries/highlights.scm
```

This makes the default queries use Zed scopes.

#### Approach B: Build-Time Query Patching
Update `build.rs` to copy Zed queries over default queries before building:

```rust
// In build.rs, after vendoring grammar
fn patch_quarto_queries(quarto_dir: &Path) {
    let zed_highlights = quarto_dir.join("queries/zed/highlights.scm");
    let default_highlights = quarto_dir.join("queries/highlights.scm");

    if zed_highlights.exists() {
        std::fs::copy(&zed_highlights, &default_highlights)
            .expect("failed to copy Zed highlights");
        eprintln!("✓ Using Zed-compatible queries");
    }
}
```

#### Approach C: Check Extension.toml Grammar Configuration
Verify if Zed extensions support custom query paths. Check Zed extension documentation.

### 6. Main Branch Comparison

The `main` branch uses `tree-sitter-pandoc-markdown` (dual grammar approach) which works. Compare:

```bash
# Switch to main and check its configuration
git stash
git checkout main
cat extension.toml
cat languages/quarto/highlights.scm
```

See what's different in the working version.

### 7. Debugging Commands

```bash
# Full extension status check
echo "=== Extension Files ==="
ls -la ~/Library/Application\ Support/Zed/extensions/installed/quarto
ls -la ~/Library/Application\ Support/Zed/extensions/work/quarto

echo "=== Grammar Files ==="
find ~/Library/Application\ Support/Zed/extensions/work/quarto -name "*.wasm" -o -name "*.scm"

echo "=== Extension.toml ==="
cat extension.toml

echo "=== Languages Config ==="
ls -la languages/quarto/

echo "=== Grammar Config ==="
ls -la grammars/quarto/queries/
```

## Most Likely Fix

Based on the architecture, the issue is probably that Zed loads `grammars/quarto/queries/highlights.scm` (which has @markup.* scopes) instead of `queries/zed/highlights.scm` (which has Zed-compatible scopes).

**Quick fix to test**:
```bash
cd grammars/quarto
cp queries/zed/highlights.scm queries/highlights.scm
cd ../..
git add grammars/quarto/queries/highlights.scm
git commit -m "fix: use Zed-compatible queries as default"
./install-dev.sh
# Then reinstall in Zed
```

This overwrites the default queries with Zed-compatible ones, which should restore syntax highlighting.

## Resume Session Command

When resuming, start with:
```bash
cd "/Users/ck432/Partners HealthCare Dropbox/Chris Kennedy/Code/zed-quarto-extension"
git status
cat NEXT_STEPS.md
```
