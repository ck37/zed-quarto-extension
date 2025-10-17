/// <reference types="node" />

/**
 * WASM Integration Tests for Zed Quarto Extension
 *
 * These tests emulate how Zed loads and uses the tree-sitter-quarto grammar
 * to parse .qmd files, allowing us to test the extension without manually
 * installing it in Zed.
 */

const assert = require("node:assert");
const { test } = require("node:test");
const fs = require("fs");
const path = require("path");

// web-tree-sitter for WASM support
let TreeSitter;
try {
  TreeSitter = require("web-tree-sitter");
} catch (e) {
  console.log("⚠️  web-tree-sitter not found - WASM tests skipped");
  console.log("   To run WASM tests: npm install web-tree-sitter");
  process.exit(0);
}

const Parser = TreeSitter.Parser;
const Language = TreeSitter.Language;
const Query = TreeSitter.Query;

const wasmPath = path.join(__dirname, "tree-sitter-quarto.wasm");
const fixturesDir = path.join(__dirname, "fixtures");
const queriesDir = path.join(__dirname, "../../languages/quarto");

// Load extension queries
const highlightsQuery = fs.readFileSync(
  path.join(queriesDir, "highlights.scm"),
  "utf8"
);
const injectionsQuery = fs.readFileSync(
  path.join(queriesDir, "injections.scm"),
  "utf8"
);

// Initialize Parser once
let parserInitialized = false;
async function getParser() {
  if (!parserInitialized) {
    await Parser.init();
    parserInitialized = true;
  }
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);
  return { parser, language: Quarto };
}

test("can load WASM grammar", async () => {
  if (!fs.existsSync(wasmPath)) {
    console.log("⚠️  WASM file not found. Run: npm run build:wasm");
    process.exit(1);
  }

  const { parser } = await getParser();
  assert.ok(parser.language !== null, "Language should be set");
});

test("parse basic.qmd without errors", async () => {
  const { parser } = await getParser();

  const source = fs.readFileSync(
    path.join(fixturesDir, "basic.qmd"),
    "utf8"
  );
  const tree = parser.parse(source);
  const root = tree.rootNode;

  // Should parse without syntax errors
  assert.ok(!root.hasError, "Document should parse without ERROR nodes");

  // Check for expected node types
  const headings = root.descendantsOfType("atx_heading");
  assert.ok(headings.length >= 2, "Should have multiple headings");

  const codeCells = root.descendantsOfType("executable_code_cell");
  assert.ok(codeCells.length > 0, "Should have code cells");

  const crossRefs = root.descendantsOfType("cross_reference");
  assert.ok(crossRefs.length > 0, "Should have cross-references");

  const inlineCodeCells = root.descendantsOfType("inline_code_cell");
  assert.ok(inlineCodeCells.length > 0, "Should have inline code cells");
});

test("parse callouts.qmd with fenced divs", async () => {
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const source = fs.readFileSync(
    path.join(fixturesDir, "callouts.qmd"),
    "utf8"
  );
  const tree = parser.parse(source);
  const root = tree.rootNode;

  assert.ok(!root.hasError, "Callouts document should parse without errors");

  // Check for Quarto-specific block types (not generic fenced_div)
  const callouts = root.descendantsOfType("callout_block");
  const conditional = root.descendantsOfType("conditional_block");
  const tabsets = root.descendantsOfType("tabset_block");

  assert.ok(callouts.length >= 3, "Should have multiple callout blocks");
  assert.ok(conditional.length >= 1, "Should have conditional block");
  assert.ok(tabsets.length >= 1, "Should have tabset block");
});

test("parse advanced.qmd with all features", async () => {
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const source = fs.readFileSync(
    path.join(fixturesDir, "advanced.qmd"),
    "utf8"
  );
  const tree = parser.parse(source);
  const root = tree.rootNode;

  assert.ok(!root.hasError, "Advanced document should parse without errors");

  // Check for various features
  const citations = root.descendantsOfType("citation");
  assert.ok(citations.length > 0, "Should have citations");

  const footnotes = root.descendantsOfType("inline_footnote");
  assert.ok(footnotes.length > 0, "Should have inline footnotes");

  const shortcodes = root.descendantsOfType("shortcode_block");
  assert.ok(shortcodes.length >= 2, "Should have shortcode blocks");

  // Check for inline attributes
  const links = root.descendantsOfType("link");
  const linksWithAttrs = links.filter(link =>
    link.childForFieldName("attributes") !== null
  );
  assert.ok(linksWithAttrs.length > 0, "Should have links with inline attributes");

  // Check for multiple language code cells
  const codeCells = root.descendantsOfType("executable_code_cell");
  assert.ok(codeCells.length >= 3, "Should have code cells in multiple languages");
});

test("extension queries use Zed-compatible scopes", async () => {
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);

  // Test that our highlights.scm uses @text.* instead of @markup.*
  assert.ok(
    highlightsQuery.includes("@text.title"),
    "highlights.scm should use @text.title"
  );
  assert.ok(
    highlightsQuery.includes("@text.emphasis"),
    "highlights.scm should use @text.emphasis"
  );
  assert.ok(
    highlightsQuery.includes("@emphasis.strong"),
    "highlights.scm should use @emphasis.strong"
  );
  assert.ok(
    highlightsQuery.includes("@punctuation.special"),
    "highlights.scm should use @punctuation.special"
  );

  // Should NOT use @markup.* scopes in actual queries (only in comments explaining the mapping)
  const queryLines = highlightsQuery.split('\n').filter(line => !line.trim().startsWith(';'));
  const queryContent = queryLines.join('\n');

  assert.ok(
    !queryContent.includes("@markup.heading"),
    "highlights.scm should NOT use @markup.heading in queries"
  );
  assert.ok(
    !queryContent.includes("@markup.bold"),
    "highlights.scm should NOT use @markup.bold in queries"
  );
});

test("highlight queries can be loaded", async () => {
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);

  // Load our extension's highlight query
  // Query already imported above
  assert.doesNotThrow(() => {
    new Query(Quarto, highlightsQuery);
  }, "Extension highlights.scm should load without errors");

  assert.doesNotThrow(() => {
    new Query(Quarto, injectionsQuery);
  }, "Extension injections.scm should load without errors");
});

test("highlighting produces correct scopes", async () => {
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);

  // Query already imported above
  const query = new Query(Quarto, highlightsQuery);

  const source = "## Heading\n\nSome **bold** text.";
  const tree = parser.parse(source);

  const captures = query.captures(tree.rootNode);
  const scopeNames = captures.map(c => c.name);

  // Verify Zed-compatible scopes are used
  assert.ok(
    scopeNames.some(s => s === "text.title" || s.startsWith("text.title")),
    "Should have text.title scope for heading content"
  );
  assert.ok(
    scopeNames.some(s => s === "punctuation.special" || s.startsWith("punctuation.special")),
    "Should have punctuation.special for heading marker"
  );
  assert.ok(
    scopeNames.some(s => s === "emphasis.strong" || s.startsWith("emphasis.strong")),
    "Should have emphasis.strong for bold text"
  );
});

test("code chunks have correct structure", async () => {
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const source = `\`\`\`{python}
#| label: fig-test
#| echo: false

print("hello")
\`\`\``;

  const tree = parser.parse(source);
  const root = tree.rootNode;

  const cell = root.descendantsOfType("executable_code_cell")[0];
  assert.ok(cell, "Should have executable_code_cell node");

  // Check for language
  const lang = cell.childForFieldName("language");
  assert.ok(lang, "Cell should have language field");
  assert.strictEqual(lang.text, "python");

  // Check for chunk options
  const options = cell.descendantsOfType("chunk_option");
  assert.ok(options.length >= 2, "Should have chunk options");

  // Verify structure
  const labelOption = options.find(opt =>
    opt.text.includes("label")
  );
  assert.ok(labelOption, "Should have label option");
});

test("cross-references parse correctly", async () => {
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const source = "See @fig-plot and @tbl-data for details.";
  const tree = parser.parse(source);
  const root = tree.rootNode;

  const xrefs = root.descendantsOfType("cross_reference");
  assert.strictEqual(xrefs.length, 2, "Should have 2 cross-references");

  const figRef = xrefs[0];
  const type = figRef.childForFieldName("type");
  const id = figRef.childForFieldName("id");

  assert.strictEqual(type.text, "fig", "First xref should be fig type");
  assert.strictEqual(id.text, "plot", "First xref should have 'plot' id");
});

test("inline code cells parse correctly", async () => {
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const source = "The answer is `{python} 2 + 2`.";
  const tree = parser.parse(source);
  const root = tree.rootNode;

  const inlineCell = root.descendantsOfType("inline_code_cell")[0];
  assert.ok(inlineCell, "Should have inline_code_cell node");

  const lang = inlineCell.childForFieldName("language");
  const content = inlineCell.childForFieldName("content");

  assert.strictEqual(lang.text, "python");
  assert.ok(content.text.includes("2 + 2"), "Should contain expression");
});

test("footnotes parse correctly", async () => {
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const source = "Text with ^[inline footnote] and reference[^1].";
  const tree = parser.parse(source);
  const root = tree.rootNode;

  const inlineFootnotes = root.descendantsOfType("inline_footnote");
  assert.ok(inlineFootnotes.length > 0, "Should have inline footnote");

  const footnoteRefs = root.descendantsOfType("footnote_reference");
  assert.ok(footnoteRefs.length > 0, "Should have footnote reference");
});

test("inline attributes parse correctly", async () => {
  const parser = new Parser();
  const Quarto = await Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const source = "Here is [styled text]{.class .another} with attributes.";
  const tree = parser.parse(source);
  const root = tree.rootNode;

  const links = root.descendantsOfType("link");
  const linkWithAttrs = links.find(link =>
    link.childForFieldName("attributes") !== null
  );

  assert.ok(linkWithAttrs, "Should have link with attributes");

  const attrList = linkWithAttrs.childForFieldName("attributes");
  assert.strictEqual(attrList.type, "attribute_list");

  const classes = attrList.descendantsOfType("attribute_class");

  assert.ok(classes.length >= 2, "Should have multiple class attributes");
});

console.log("\n✅ All WASM integration tests configured");
console.log("   These tests emulate how Zed loads and parses .qmd files\n");
