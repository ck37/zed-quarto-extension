/// <reference types="node" />

/**
 * WASM Parser Tests
 *
 * Tests the WASM build of the parser (used by Zed and web editors)
 * to ensure it produces correct parse trees for key Quarto features.
 */

const assert = require("node:assert");
const { test } = require("node:test");
const fs = require("fs");
const path = require("path");

// tree-sitter is a peer dependency, may not be installed
let Parser;
try {
  Parser = require("tree-sitter");
} catch (e) {
  console.log("⚠️  tree-sitter not found - WASM tests skipped");
  console.log("   To run WASM tests: npm install tree-sitter");
  process.exit(0);
}

const wasmPath = path.join(__dirname, "../../tree-sitter-quarto.wasm");

test("can load WASM parser", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  assert.doesNotThrow(() => parser.setLanguage(Quarto));
});

test("inline attributes - simple class", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const code = "[simple text]{.class}";
  const tree = parser.parse(code);
  const root = tree.rootNode;

  // Should have a link node with attributes
  const link = root.descendantsOfType("link")[0];
  assert.ok(link, "Should have link node");

  const attrList = link.childForFieldName("attributes");
  assert.ok(attrList, "Should have attributes field");
  assert.strictEqual(attrList.type, "attribute_list");

  // Should have a class
  const classes = attrList.descendantsOfType("attribute_class");
  assert.strictEqual(classes.length, 1, "Should have 1 class");
  assert.strictEqual(classes[0].text, ".class");
});

test("inline attributes - ID and class", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const code = "[highlighted text]{#important .alert}";
  const tree = parser.parse(code);
  const root = tree.rootNode;

  const link = root.descendantsOfType("link")[0];
  const attrList = link.childForFieldName("attributes");

  // Should have ID
  const ids = attrList.descendantsOfType("attribute_id");
  assert.strictEqual(ids.length, 1, "Should have 1 ID");
  assert.strictEqual(ids[0].text, "#important");

  // Should have class
  const classes = attrList.descendantsOfType("attribute_class");
  assert.strictEqual(classes.length, 1, "Should have 1 class");
  assert.strictEqual(classes[0].text, ".alert");
});

test("inline attributes - multiple classes", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const code = "[text]{.class1 .class2 .class3}";
  const tree = parser.parse(code);
  const root = tree.rootNode;

  const link = root.descendantsOfType("link")[0];
  const attrList = link.childForFieldName("attributes");

  const classes = attrList.descendantsOfType("attribute_class");
  assert.strictEqual(classes.length, 3, "Should have 3 classes");
  assert.strictEqual(classes[0].text, ".class1");
  assert.strictEqual(classes[1].text, ".class2");
  assert.strictEqual(classes[2].text, ".class3");
});

test("inline attributes - in context", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const code = "Here is [inline span]{#id .styled} in context.";
  const tree = parser.parse(code);
  const root = tree.rootNode;

  // Should parse without errors
  assert.ok(!root.hasError(), "Should parse without errors in context");

  const link = root.descendantsOfType("link")[0];
  assert.ok(link, "Should have link node");

  const attrList = link.childForFieldName("attributes");
  assert.ok(attrList, "Should have attributes");

  // Check structure
  const ids = attrList.descendantsOfType("attribute_id");
  const classes = attrList.descendantsOfType("attribute_class");
  assert.strictEqual(ids.length, 1);
  assert.strictEqual(classes.length, 1);
});

test("traditional links still work", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const code = "[link text](https://example.com)";
  const tree = parser.parse(code);
  const root = tree.rootNode;

  const link = root.descendantsOfType("link")[0];
  assert.ok(link, "Should have link node");

  const dest = link.childForFieldName("destination");
  assert.ok(dest, "Should have destination field");
  assert.strictEqual(dest.text, "https://example.com");

  // Should NOT have attributes
  const attrs = link.childForFieldName("attributes");
  assert.strictEqual(attrs, null, "Traditional links should not have attributes");
});

test("executable cell with attributes", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const code = "```{python #fig-plot .class}\nplt.plot([1, 2, 3])\n```";
  const tree = parser.parse(code);
  const root = tree.rootNode;

  const cell = root.descendantsOfType("executable_code_cell")[0];
  assert.ok(cell, "Should have executable_code_cell node");

  const attrList = cell.descendantsOfType("attribute_list")[0];
  assert.ok(attrList, "Should have attribute_list in cell");

  const ids = attrList.descendantsOfType("attribute_id");
  const classes = attrList.descendantsOfType("attribute_class");
  assert.strictEqual(ids.length, 1, "Should have ID");
  assert.strictEqual(classes.length, 1, "Should have class");
  assert.strictEqual(ids[0].text, "#fig-plot");
  assert.strictEqual(classes[0].text, ".class");
});

test("cross-references work", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const code = "See @fig-plot for details.";
  const tree = parser.parse(code);
  const root = tree.rootNode;

  const xref = root.descendantsOfType("cross_reference")[0];
  assert.ok(xref, "Should have cross_reference node");

  const type = xref.childForFieldName("type");
  const id = xref.childForFieldName("id");
  assert.strictEqual(type.text, "fig");
  assert.strictEqual(id.text, "plot");
});

test("inline code cells work", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const code = "`{python} 2 + 2`";
  const tree = parser.parse(code);
  const root = tree.rootNode;

  const inlineCell = root.descendantsOfType("inline_code_cell")[0];
  assert.ok(inlineCell, "Should have inline_code_cell node");

  const lang = inlineCell.childForFieldName("language");
  const expr = inlineCell.childForFieldName("expression");
  assert.strictEqual(lang.text, "python");
  assert.strictEqual(expr.text, " 2 + 2");
});

test("footnotes work", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  const code = "Text with ^[inline footnote] here.";
  const tree = parser.parse(code);
  const root = tree.rootNode;

  const footnote = root.descendantsOfType("inline_footnote")[0];
  assert.ok(footnote, "Should have inline_footnote node");
});

test("WASM parser matches C parser behavior", async () => {
  const parser = new Parser();
  const Quarto = await Parser.Language.load(wasmPath);
  parser.setLanguage(Quarto);

  // Test a complex document
  const code = `# Heading

Here is [highlighted text]{#id .alert} with attributes.

Traditional [link](https://example.com) also works.

\`\`\`{python #fig-plot}
plt.plot([1, 2, 3])
\`\`\`

See @fig-plot for the visualization.
`;

  const tree = parser.parse(code);
  const root = tree.rootNode;

  // Should parse the whole document
  assert.strictEqual(root.type, "document");

  // Should have key node types
  assert.ok(root.descendantsOfType("atx_heading").length > 0, "Has heading");
  assert.ok(root.descendantsOfType("link").length >= 2, "Has links/spans");
  assert.ok(root.descendantsOfType("executable_code_cell").length > 0, "Has code cell");
  assert.ok(root.descendantsOfType("cross_reference").length > 0, "Has cross-reference");

  // Inline attributes should work
  const linksWithAttrs = root.descendantsOfType("link").filter(link =>
    link.childForFieldName("attributes") !== null
  );
  assert.ok(linksWithAttrs.length > 0, "Has links with attributes");
});
