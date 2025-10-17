/// <reference types="node" />

const assert = require("node:assert");
const { test } = require("node:test");

// tree-sitter is a peer dependency, may not be installed
let Parser;
try {
  Parser = require("tree-sitter");
} catch (e) {
  console.log("⚠️  tree-sitter not found - binding tests skipped");
  console.log("   To run binding tests: npm install tree-sitter");
  process.exit(0);
}

test("can load grammar", () => {
  const parser = new Parser();
  assert.doesNotThrow(() => parser.setLanguage(require(".")));
});
