# Node.js Bindings Tests

This directory contains tests for the tree-sitter-quarto Node.js bindings.

## Test Files

- **`binding_test.js`** - Tests that the compiled C parser can be loaded as a Node module
- **`wasm_test.js`** - Tests the WASM parser (used by Zed, web editors, etc.)

## Running Tests

### Corpus Tests (Always Available)

```bash
npm run test:corpus
# or
npx tree-sitter test
```

These tests use the tree-sitter CLI and don't require the `tree-sitter` npm module.

**Status:** ✅ 87/87 tests passing (100%)

### Binding Tests (Requires tree-sitter npm module)

```bash
npm run test:bindings
```

Tests that the C parser compiles and loads correctly in Node.js.

**Note:** Requires `tree-sitter` npm module, which has C++20 build requirements. If not available, tests gracefully skip.

### WASM Tests (Requires tree-sitter npm module + WASM build)

```bash
npm run test:wasm
```

Tests the WebAssembly parser used by Zed and web-based editors.

**Features tested:**
- Inline attributes (`[text]{.class}`, `[text]{#id .class}`)
- Traditional links (`[text](url)`)
- Executable cells with attributes
- Cross-references
- Inline code cells
- Footnotes
- WASM/C parser equivalence

**Note:** Requires `tree-sitter` npm module, which has C++20 build requirements. If not available, tests gracefully skip.

### All Tests

```bash
npm test              # Corpus + bindings (skips if tree-sitter unavailable)
npm run test:all      # Corpus + bindings + WASM
```

## Why Tests Skip

The `tree-sitter` npm module requires:
- C++20 compiler
- node-gyp build toolchain
- Native compilation support

If these aren't available, the binding and WASM tests gracefully skip with a message:

```
⚠️  tree-sitter not found - binding tests skipped
   To run binding tests: npm install tree-sitter
```

**This is expected behavior** - the corpus tests (which are the primary test suite) don't require the npm module and always run.

## CI/CD Considerations

For CI pipelines:

```bash
# Minimum - corpus tests only (always works)
npm run test:corpus

# Full suite (if C++20 compiler available)
npm install tree-sitter
npm run test:all
```

Most users only need the corpus tests. The binding and WASM tests are primarily for:
- Development of the parser itself
- Testing Zed integration
- Ensuring WASM build correctness

## WASM Parser

The WASM parser is built with:

```bash
npm run build:wasm
```

This creates `tree-sitter-quarto.wasm` at the project root, which is used by:
- Zed editor (via dev extension in `zed-extension/`)
- tree-sitter playground (`npm start`)
- Web-based editors
- The WASM test suite

## Inline Attributes Testing

The WASM tests specifically verify inline attributes work correctly:

```javascript
// Simple class
"[text]{.class}" → link with attribute_class ✅

// ID and class
"[text]{#id .class}" → link with attribute_id + attribute_class ✅

// Multiple classes
"[text]{.a .b .c}" → link with 3 attribute_class nodes ✅

// In context (no ERROR nodes)
"Here is [text]{.class} foo." → Parses cleanly ✅
```

See `wasm_test.js` for the full test suite.
