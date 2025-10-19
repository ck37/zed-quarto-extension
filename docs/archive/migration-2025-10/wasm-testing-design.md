# WASM-Based Integration Testing for Zed Extension

## Goal

Create automated tests that emulate how Zed uses the extension to parse `.qmd` files, eliminating the need to manually install and test in Zed during development.

## Why WASM?

Zed compiles tree-sitter grammars to WASM and loads them via the WASM runtime. By testing with WASM, we:

1. **Match Zed's environment** - Test the exact same binary Zed will use
2. **Test query loading** - Verify our extension queries override grammar queries
3. **Test highlighting** - Ensure semantic scopes are correct
4. **Catch regressions** - Automated tests prevent breaking changes

## Architecture

### Option 1: Node.js + tree-sitter WASM (Recommended)

**Pros:**
- Simple setup - just `npm install tree-sitter`
- Fast execution
- Easy to integrate with existing test infrastructure
- Grammar repo already has this pattern (`bindings/node/wasm_test.js`)

**Cons:**
- Requires Node.js in CI
- Different from Zed's exact WASM runtime (but close enough)

### Option 2: Rust + wasmtime

**Pros:**
- Same language as extension
- Can use existing Cargo test infrastructure
- More control over WASM execution

**Cons:**
- More complex setup
- Slower compile times
- Need to manually load queries

### Option 3: tree-sitter CLI + playground

**Pros:**
- Official tree-sitter tool
- Can visualize parse trees

**Cons:**
- Harder to automate
- No query testing
- Manual workflow

## Recommended Approach: Node.js WASM Tests

### Test Structure

```
tests/
  wasm/
    integration.test.js    # Main integration tests
    fixtures/              # Sample .qmd files
      basic.qmd
      code-chunks.qmd
      cross-references.qmd
      callouts.qmd
    queries/               # Extension queries (symlink to languages/quarto/)
```

### What to Test

1. **Parse tree correctness**
   - Verify grammar parses key Quarto features
   - Check for ERROR nodes
   - Validate structure (headings, code blocks, etc.)

2. **Highlighting with extension queries**
   - Load `.wasm` grammar
   - Load our extension's `highlights.scm` and `injections.scm`
   - Verify correct scopes are applied
   - Test that `@text.*` scopes work (not `@markup.*`)

3. **Real-world fixtures**
   - Test against actual `.qmd` files
   - Cover all major features: chunks, cross-refs, callouts, shortcodes
   - Regression tests for known issues

### Implementation Plan

#### Phase 1: Basic WASM Parsing Tests
- [ ] Set up Node.js test environment
- [ ] Build grammar WASM in CI
- [ ] Test parse trees for fixture files
- [ ] Verify no ERROR nodes

#### Phase 2: Highlighting Tests
- [ ] Load extension queries into tree-sitter highlighter
- [ ] Test scope application
- [ ] Verify Zed-compatible scopes (`@text.*`)
- [ ] Test language injections (Python, R, etc.)

#### Phase 3: CI Integration
- [ ] Add GitHub Actions workflow
- [ ] Build WASM on each commit
- [ ] Run test suite
- [ ] Report coverage

## Example Test Code

```javascript
const assert = require("node:assert");
const { test } = require("node:test");
const Parser = require("tree-sitter");
const fs = require("fs");
const path = require("path");

// Load WASM grammar (built from tree-sitter-quarto)
const wasmPath = path.join(__dirname, "../../tree-sitter-quarto.wasm");

// Load extension queries
const highlightsQuery = fs.readFileSync(
  path.join(__dirname, "../../languages/quarto/highlights.scm"),
  "utf8"
);

test("parse basic quarto document", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const source = fs.readFileSync(
    path.join(__dirname, "fixtures/basic.qmd"),
    "utf8"
  );

  const tree = parser.parse(source);
  const root = tree.rootNode;

  // Should parse without errors
  assert.ok(!root.hasError(), "Document should parse without errors");

  // Should have expected structure
  const headings = root.descendantsOfType("atx_heading");
  assert.ok(headings.length > 0, "Should have headings");

  const codeCells = root.descendantsOfType("executable_code_cell");
  assert.ok(codeCells.length > 0, "Should have code cells");
});

test("extension queries use correct scopes", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  // Parse with extension's highlight query
  const Query = Parser.Query;
  const query = new Query(Quarto, highlightsQuery);

  const source = "## Heading\n\nSome text.";
  const tree = parser.parse(source);

  const captures = query.captures(tree.rootNode);

  // Should use Zed-compatible scopes
  const scopeNames = captures.map(c => c.name);
  assert.ok(
    scopeNames.includes("text.title"),
    "Should use @text.title for headings"
  );
  assert.ok(
    scopeNames.includes("punctuation.special"),
    "Should use @punctuation.special for markers"
  );

  // Should NOT use grammar's @markup.* scopes
  assert.ok(
    !scopeNames.includes("markup.heading"),
    "Should not use @markup.heading"
  );
});
```

## Benefits

1. **Fast feedback** - No need to manually test in Zed
2. **Regression prevention** - Catch breaking changes before they ship
3. **Query validation** - Ensure extension queries work as expected
4. **CI/CD ready** - Automated testing in GitHub Actions
5. **Fixture library** - Build up test cases for all Quarto features

## Next Steps

1. Set up Node.js test infrastructure
2. Add npm scripts for WASM build
3. Create fixture `.qmd` files covering key features
4. Implement basic parsing tests
5. Add highlighting/query tests
6. Integrate with CI

## Related Files

- Grammar WASM tests: `https://github.com/ck37/tree-sitter-quarto/blob/main/bindings/node/wasm_test.js`
- Extension queries: `languages/quarto/*.scm`
- Test fixtures: `tests/fixtures/*.qmd` (to be created)
