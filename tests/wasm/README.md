# WASM Integration Tests

Automated tests that emulate how Zed uses the tree-sitter-quarto grammar to parse `.qmd` files.

## Purpose

These tests eliminate the need to manually install the extension in Zed during development by:

1. **Loading the WASM grammar** - Same binary Zed uses
2. **Parsing fixture files** - Test real `.qmd` documents
3. **Validating queries** - Ensure extension queries use correct scopes
4. **Checking node types** - Verify grammar structure

## Prerequisites

```bash
# Install Node.js dependencies
npm install

# Install tree-sitter (peer dependency)
npm install tree-sitter
```

## Running Tests

```bash
# Build WASM grammar from tree-sitter-quarto
npm run build:wasm

# Run all WASM tests
npm run test:wasm

# Or run both steps together
npm test
```

## Test Files

- **integration.test.js** - Main test suite
  - Grammar loading
  - Parse tree validation
  - Query testing
  - Scope checking

- **fixtures/** - Sample `.qmd` files
  - `basic.qmd` - Common features (chunks, cross-refs, inline code)
  - `callouts.qmd` - Fenced divs (callouts, tabsets, conditional content)
  - `advanced.qmd` - All features (citations, footnotes, shortcodes, inline attributes)

## What's Tested

### Parse Tree Structure
- ✅ Documents parse without ERROR nodes
- ✅ Headings (`atx_heading`)
- ✅ Code cells (`executable_code_cell`)
- ✅ Cross-references (`cross_reference`)
- ✅ Inline code cells (`inline_code_cell`)
- ✅ Fenced divs (`fenced_div`)
- ✅ Citations (`citation`)
- ✅ Footnotes (`inline_footnote`, `footnote_reference`)
- ✅ Shortcodes (`shortcode`)
- ✅ Inline attributes (`link` with `attributes`)

### Query Validation
- ✅ Extension queries load without errors
- ✅ Zed-compatible scopes (`@text.*`, `@emphasis.strong`)
- ✅ NO grammar scopes (`@markup.*`)
- ✅ Correct scope application

### Feature Coverage
- ✅ Multiple languages (Python, R, Julia, SQL)
- ✅ Chunk options (`#| key: value`)
- ✅ Cross-reference types (`@fig-`, `@tbl-`)
- ✅ Callout types (note, warning, important, tip, caution)
- ✅ Tabsets and conditional content
- ✅ Inline attributes (`[text]{.class}`)

## Adding New Tests

1. **Add fixture file**: Create `.qmd` in `fixtures/`
2. **Add test case**: Add `test()` in `integration.test.js`
3. **Run tests**: `npm test`

Example:

```javascript
test("parse my-feature.qmd", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const source = fs.readFileSync(
    path.join(fixturesDir, "my-feature.qmd"),
    "utf8"
  );
  const tree = parser.parse(source);

  // Assert structure
  assert.ok(!tree.rootNode.hasError());
  // ... more assertions
});
```

## Troubleshooting

### "tree-sitter not found"

```bash
npm install tree-sitter
```

### "WASM file not found"

```bash
npm run build:wasm
```

This requires `tree-sitter-cli` and the compiled grammar in `grammars/quarto-vendored/`.

### Tests fail after grammar update

1. Clean and rebuild WASM:
   ```bash
   cd grammars/quarto
   tree-sitter generate
   cd ../..
   npm run build:wasm
   ```

2. Update queries if grammar nodes changed:
   - Check `languages/quarto/highlights.scm`
   - Check `languages/quarto/injections.scm`

3. Re-run tests:
   ```bash
   npm test
   ```

## CI Integration

The GitHub Actions workflow should:

1. Checkout repository with grammar submodule
2. Install Node.js and dependencies
3. Build WASM grammar
4. Run test suite
5. Report results

See `.github/workflows/test.yml` for configuration.

## Comparison with Rust Tests

| Aspect | Rust Tests (`cargo test`) | WASM Tests (`npm test`) |
|--------|---------------------------|-------------------------|
| **Grammar** | tree-sitter-pandoc-markdown | tree-sitter-quarto |
| **Format** | Native C library | WASM binary |
| **Environment** | Matches dev build | Matches Zed runtime |
| **Speed** | Faster (native) | Slower (WASM overhead) |
| **Purpose** | Unit tests | Integration tests |
| **Queries** | Requires manual patching | Uses extension queries directly |

Use WASM tests for end-to-end validation and Rust tests for quick iteration.

## Resources

- [tree-sitter WASM](https://tree-sitter.github.io/tree-sitter/using-parsers#webassembly)
- [tree-sitter-quarto](https://github.com/ck37/tree-sitter-quarto)
- [Node.js test runner](https://nodejs.org/api/test.html)
- [Design doc](../../docs/wasm-testing-design.md)
