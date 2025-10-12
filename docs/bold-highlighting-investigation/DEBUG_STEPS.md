# Debug Steps for Bold Highlighting

## Current Status
- Extension installs successfully
- Both grammars compile without errors
- No bold text highlighting (**text** not highlighted)

## Quick Diagnostic

### 1. Check what IS working
Open `test-bold.qmd` and verify:
- ✅ Does `# Heading` show as a title/heading? (tests block grammar)
- ✅ Does ` ```python ` code block get Python highlighting? (tests code injection)
- ✅ Does YAML frontmatter get highlighted? (tests YAML injection)
- ❌ Does **bold** get highlighted? (tests inline grammar injection)

### 2. Check injection query location
The injection query is in `languages/quarto/injections.scm` line 9-10:
```scheme
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))
```

**Question**: Should this be in `languages/quarto/injections.scm` OR in `languages/pandoc_markdown_inline/injections.scm`?

Looking at PHP extension:
- `languages/php/injections.scm` - contains injection TO phpdoc
- `languages/phpdoc/` - no injections.scm file

So injections.scm in the PARENT language is correct.

### 3. Possible Issues

#### Issue A: Injection not triggering
The `(inline)` node might not exist or might be named differently.

**Test**: Check what nodes exist in the AST
```bash
cd "/Users/ck432/Partners HealthCare Dropbox/Chris Kennedy/Code/zed-quarto-extension"
tree-sitter parse test-bold.qmd | grep -i inline
```

#### Issue B: Language name mismatch
The injection says `"pandoc_markdown_inline"` but maybe Zed expects different name.

**Files to verify**:
- `extension.toml` line 14: `[grammars.pandoc_markdown_inline]` ← grammar name
- `languages/pandoc_markdown_inline/config.toml` line 2: `grammar = "pandoc_markdown_inline"` ← must match

#### Issue C: Zed doesn't support grammar-to-grammar injection
Maybe Zed only supports injecting into existing Zed languages (yaml, python, etc.), not custom grammars?

**Evidence needed**: Find an example of a Zed extension that injects one custom grammar into another custom grammar.

### 4. Alternative Approach: Simpler Test

Create a minimal test without dual grammars:

**Option**: Use only block grammar, but copy inline highlights into it
- Remove inline grammar from extension.toml
- Remove inline language directory
- Copy `(strong_emphasis) @text.strong` etc. into `languages/quarto/highlights.scm`
- Remove the injection query

This tests: Can the block grammar's highlights work on inline nodes?

### 5. Check Zed's actual markdown implementation

Need to find if Zed's built-in markdown:
- Has `languages/markdown/` AND `languages/markdown_inline/` directories?
- Or does it handle this differently?

The markdown we saw earlier had:
- `languages/markdown/config.toml` with `grammar = "markdown"`
- `languages/markdown/injections.scm` with inline injection
- But no `languages/markdown_inline/` directory!

**Key question**: Where does Zed's markdown-inline grammar get its highlights from?

## Next Test to Try

### Test 7: Follow Zed's Markdown Pattern Exactly

Configuration:
1. Keep both grammars in extension.toml
2. Keep `languages/pandoc_markdown_inline/config.toml` with `hidden = true`
3. **Remove** `languages/pandoc_markdown_inline/highlights.scm`
4. Let Zed use the grammar's built-in highlights (from `grammars/pandoc_markdown_inline/tree-sitter-pandoc-markdown-inline/queries/highlights.scm`)

Hypothesis: Zed automatically loads highlights from the grammar's own queries directory, and our custom highlights.scm might be interfering.
