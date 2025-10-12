# Bold Text Highlighting Test Log

## Problem Statement
Bold text (**bold** and __bold__) is not highlighting in .qmd files in Zed editor.

## Root Cause Analysis
The pandoc-markdown grammar uses a dual-grammar architecture:
- **Block grammar** (`tree-sitter-pandoc-markdown`): Parses document structure, creates `(inline)` nodes
- **Inline grammar** (`tree-sitter-pandoc-markdown-inline`): Parses inline content like `(strong_emphasis)`, `(emphasis)`, etc.

The inline grammar must be injected into `(inline)` nodes for bold/italic to work.

## Test History

### Test 1: Block Grammar Only (FAILED)
**Date**: 2025-10-12 (initial)
**Configuration**:
- Only `pandoc_markdown` grammar in `extension.toml`
- No inline grammar injection
- No `languages/pandoc_markdown_inline/` directory

**Hypothesis**: Block grammar alone can handle all highlighting.

**Result**: ❌ FAILED - No bold highlighting
- Extension installed successfully
- Basic syntax highlighting works (headings, code blocks)
- Bold text (**text**) not highlighted
- **Reason**: Block grammar only creates `(inline)` nodes as containers; it doesn't parse their contents

---

### Test 2: Both Grammars + Inline Language Directory (FAILED)
**Date**: 2025-10-12 ~09:25
**Configuration**:
- Both grammars in `extension.toml`:
  ```toml
  [grammars.pandoc_markdown]
  repository = "https://github.com/ck37/tree-sitter-pandoc-markdown"
  commit = "f9d68613baef187daffcfbd4af09b5eab1005d38"
  path = "tree-sitter-pandoc-markdown"

  [grammars.pandoc_markdown_inline]
  repository = "https://github.com/ck37/tree-sitter-pandoc-markdown"
  commit = "f9d68613baef187daffcfbd4af09b5eab1005d38"
  path = "tree-sitter-pandoc-markdown-inline"
  ```
- Injection in `languages/quarto/injections.scm`:
  ```scheme
  ((inline) @injection.content
   (#set! injection.language "pandoc_markdown_inline"))
  ```
- Created `languages/pandoc_markdown_inline/` with:
  - `config.toml`: `name = "Pandoc Markdown Inline"\ngrammar = "pandoc_markdown_inline"`
  - `highlights.scm`: Copy of upstream inline grammar highlights

**Hypothesis**: The inline grammar needs its own language configuration directory.

**Result**: ❌ FAILED - No bold highlighting
- Both grammars compiled successfully
- Extension installed successfully
- Still no bold text highlighting
- **Reason**: Unknown - possibly injection not working or language not recognized

---

### Test 3: Changed Injection from @content to @injection.content (FAILED)
**Date**: 2025-10-12 ~09:25
**Configuration**:
- Changed injection query based on Zed docs example:
  ```scheme
  ((inline) @content
   (#set! injection.language "pandoc_markdown_inline"))
  ```
- Everything else same as Test 2

**Hypothesis**: Zed docs show `@content` pattern, so maybe that's correct instead of `@injection.content`.

**Result**: ❌ FAILED - No bold highlighting
- **Reason**: Conflicting documentation - Zed's actual markdown implementation uses `@injection.content`

---

### Test 4: Reverted to @injection.content, Removed Inline Language Dir (CURRENT)
**Date**: 2025-10-12 ~current
**Configuration**:
- Both grammars in `extension.toml` (same as Test 2)
- Injection: `((inline) @injection.content (#set! injection.language "pandoc_markdown_inline"))`
- **Removed** `languages/pandoc_markdown_inline/` directory
- Upstream grammar's block `highlights.scm` already includes `(strong_emphasis) @text.strong`

**Hypothesis**:
- The inline grammar doesn't need a separate language config directory
- Zed should use the inline grammar's own query files automatically
- The block grammar's highlights.scm includes inline node highlights for when injection works
- This matches how the upstream grammar is designed (no separate inline language in their examples)

**Research Evidence**:
- Upstream `tree-sitter-pandoc-markdown/queries/injections.scm` uses: `((inline) @injection.content (#set! injection.language "pandoc_markdown_inline"))`
- Upstream `tree-sitter-pandoc-markdown/queries/highlights.scm` includes both block and inline highlights
- Zed's built-in markdown uses: `((inline) @injection.content (#set! injection.language "markdown-inline"))`
- Zed's built-in markdown does NOT have a separate `languages/markdown_inline/` directory

**Status**: 🔄 PENDING TEST
**Next Steps**:
1. Run `./install-dev.sh`
2. Restart Zed
3. Install extension via "zed: install dev extension"
4. Test bold highlighting in test-bold.qmd

---

### Test 5: Both Grammars + Inline Language Dir (Based on Other Extensions)
**Date**: 2025-10-12 09:50
**Configuration**:
- Both grammars in `extension.toml` (same as Test 2):
  ```toml
  [grammars.pandoc_markdown]
  repository = "https://github.com/ck37/tree-sitter-pandoc-markdown"
  commit = "f9d68613baef187daffcfbd4af09b5eab1005d38"
  path = "tree-sitter-pandoc-markdown"

  [grammars.pandoc_markdown_inline]
  repository = "https://github.com/ck37/tree-sitter-pandoc-markdown"
  commit = "f9d68613baef187daffcfbd4af09b5eab1005d38"
  path = "tree-sitter-pandoc-markdown-inline"
  ```
- Injection in `languages/quarto/injections.scm`:
  ```scheme
  ((inline) @injection.content
   (#set! injection.language "pandoc_markdown_inline"))
  ```
- **Recreated** `languages/pandoc_markdown_inline/` with:
  - `config.toml`: `name = "Pandoc Markdown Inline"\ngrammar = "pandoc_markdown_inline"`
  - `highlights.scm`: Full inline grammar highlights (emphasis, strong, links, etc.)

**Hypothesis**:
Based on Ruby and PHP extensions research:
- Each grammar needs its own language directory
- The inline grammar needs `config.toml` to map the grammar name
- The inline grammar needs `highlights.scm` for proper highlighting
- This is the standard Zed pattern for multi-grammar extensions

**Research Evidence**:
- Ruby extension: 3 grammars → multiple language directories (erb uses `embedded_template` grammar)
- PHP extension: 2 grammars → 2 language directories (phpdoc has own config + highlights)
- Both follow the pattern: grammar declared in extension.toml → language directory with config.toml

**Status**: 🔄 PENDING TEST
**Next Steps**:
1. Run `./install-dev.sh`
2. Restart Zed
3. Install extension via "zed: install dev extension"
4. Test bold highlighting in test-bold.qmd

---

## Comparison with Other Extensions

### Ruby Extension (3 grammars)
- **Grammars in extension.toml**: `ruby`, `embedded_template`, `rbs`
- **Language directories**: `ruby/`, `erb/`, `html-erb/`, `js-erb/`, `rbs/`, `yaml-erb/`
- **Pattern**: Each grammar has at least one corresponding language directory
- **Example**: `embedded_template` grammar → `erb/` language with `config.toml` setting `grammar = "embedded_template"`

### PHP Extension (2 grammars)
- **Grammars in extension.toml**: `php`, `phpdoc`
- **Language directories**: `php/`, `phpdoc/`
- **Pattern**: Each grammar has a corresponding language directory
- **Injection**: PHP injects `phpdoc` grammar into PHPDoc comments
- **Example**: `phpdoc/config.toml` sets `grammar = "phpdoc"` and has its own `highlights.scm`

### Pattern Identified
✅ **When using multiple grammars, EACH grammar needs its own `languages/[name]/` directory with:**
- `config.toml` specifying `grammar = "[grammar_name]"`
- `highlights.scm` (optional - can use grammar's built-in queries)
- Other query files as needed

⚠️ **Test 4 hypothesis was WRONG** - we removed `languages/pandoc_markdown_inline/` but other extensions show we NEED it!

## Key Learnings

1. **Zed Extension Caching**: Zed aggressively caches extension manifests. Must restart Zed AND clear cache for changes to take effect.
   - Cache locations:
     - `~/Library/Application Support/Zed/extensions/installed/quarto` (symlink)
     - `~/Library/Application Support/Zed/extensions/work/quarto` (work dir)

2. **Grammar Compilation**: The `grammars/` directory created by local `cargo build` conflicts with Zed's grammar fetching. Must delete it before installation.

3. **Injection Query Syntax**:
   - When using `(#set! injection.language "...")`, use `@injection.content`
   - The language name must match the grammar name exactly
   - Documentation can be misleading - check actual implementation

4. **Dual Grammar Architecture**:
   - Block grammar creates structure and `(inline)` container nodes
   - Inline grammar parses content within those containers
   - Both grammars must be declared in `extension.toml`
   - Injection query connects them via the `(inline)` node

## Test Template (for future tests)

```markdown
### Test X: [Brief Description] (RESULT)
**Date**: YYYY-MM-DD
**Configuration**:
- [List all relevant config changes]

**Hypothesis**: [What we think will happen and why]

**Result**: ✅ SUCCESS / ❌ FAILED - [Brief outcome]
- [Detailed observations]
- **Reason**: [Why it worked or failed]
```

---

## Files to Check When Debugging

1. `extension.toml` - Grammar declarations
2. `languages/quarto/injections.scm` - Language injection rules
3. `languages/quarto/highlights.scm` - Syntax highlighting rules
4. `~/Library/Logs/Zed/Zed.log` - Installation logs and errors
5. `~/Library/Application Support/Zed/extensions/installed/` - Installed extensions
6. `grammars/` directory - Should NOT exist before Zed installation

### Test 5 Result Update
**Result**: ❌ FAILED - No bold highlighting
- Extension installed successfully at 09:50
- Both grammars compiled without errors
- Configuration exactly matches Test 2
- Still no bold highlighting

---

### Test 6: Add hidden=true to Inline Config
**Date**: 2025-10-12 09:52
**Configuration**:
- Same as Test 5, plus:
- Added `hidden = true` to `languages/pandoc_markdown_inline/config.toml`

**Hypothesis**: PHP's phpdoc uses `hidden = true` for auxiliary grammars only used via injection.

**Result**: Not tested independently (moved to Test 7)

---

### Test 7: Remove Custom Inline highlights.scm (CURRENT HYPOTHESIS)
**Date**: 2025-10-12 ~current

**Configuration**:
- Keep both grammars in `extension.toml`
- Keep `languages/pandoc_markdown_inline/config.toml` with `hidden = true`
- **REMOVE** `languages/pandoc_markdown_inline/highlights.scm`
- Let Zed use grammar's built-in highlights from compiled grammar queries

**Hypothesis**:
Zed's built-in markdown does NOT have a `languages/markdown_inline/` directory with custom highlights.scm. This suggests:
1. Zed automatically loads highlights from the grammar's own `queries/highlights.scm`
2. Our custom `languages/pandoc_markdown_inline/highlights.scm` might be BLOCKING the grammar's built-in highlights
3. For injected grammars, Zed may expect to use the grammar's own query files, not extension-provided ones
4. The `hidden = true` config is sufficient - just tells Zed the grammar exists for injection

**Supporting Evidence**:
- ✅ Zed's markdown: Has `languages/markdown/injections.scm` with inline injection
- ✅ Zed's markdown: NO `languages/markdown_inline/` directory found
- ✅ Grammar provides its own highlights: `grammars/pandoc_markdown_inline/tree-sitter-pandoc-markdown-inline/queries/highlights.scm` exists
- ✅ Grammar's highlights.scm includes: `(strong_emphasis) @text.strong`

**Key Insight - Structure vs Content Injection**:
- PHP/Ruby inject into **content nodes** (comment text, template content)
- Markdown injects into **structure nodes** (`(inline)` containers that get re-parsed)
- Structure-node injections may need grammar's own queries, not custom extension queries

**Status**: 🔄 PENDING TEST

**Test Steps**:
1. `rm languages/pandoc_markdown_inline/highlights.scm`
2. Run `./install-dev.sh`
3. Restart Zed
4. Install extension
5. Test bold highlighting in `test-bold.qmd`


### Test 7 Result
**Result**: ❌ FAILED - No bold highlighting
- Extension installed successfully
- Both grammars compiled
- Removed custom highlights.scm from inline language directory
- Still no bold highlighting
- **This confirms the issue is NOT about custom highlights blocking grammar queries**

---

### Critical Research Discovery

After analyzing popular extensions (Java, C#, Ruby, PHP) and Zed's built-in markdown:

**NO extension found that does custom-to-custom grammar injection like ours**
- Java: Multiple grammars for different file types (not injection)
- C#: Single grammar only
- Ruby: Multiple grammars for different file types (not injection)  
- PHP: Injects `phpdoc` into comment **content** (text-based injection)
- **Markdown**: Injects into `(inline)` **structure** nodes, BUT no `languages/markdown_inline/` directory!

**Key Question**: Can extensions even do grammar-to-grammar injection, or is this a built-in-only feature?

See `EXTENSION_RESEARCH.md` for full analysis.

---

### Next Investigation Steps

**Option 1**: Find ANY extension doing custom grammar injection
- Search zed-extensions for injection patterns
- Look for extensions with multiple grammars where one injects into another

**Option 2**: Try alternative approach
- Merge inline highlights into block grammar's highlights.scm
- Use single grammar without injection
- May lose some Pandoc inline parsing precision

**Option 3**: Ask Zed community
- File issue: "How to inject custom grammar into another custom grammar?"
- Check if this is supported for extensions or built-in only

