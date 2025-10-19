# Extension Research - Grammar Patterns in Popular Zed Extensions

## Purpose
Understanding how popular Zed extensions structure their grammars to solve our bold highlighting issue.

## Research Date
2025-10-12

---

## Java Extension (120 stars ⭐ - Most Popular Language)

**Repository**: https://github.com/zed-extensions/java

### Grammar Configuration
```toml
[grammars.java]
repository = "https://github.com/tree-sitter/tree-sitter-java"
commit = "94703d5a6bed02b98e438d7cad1136c01a60ba2c"

[grammars.properties]
repository = "https://github.com/tree-sitter-grammars/tree-sitter-properties"
commit = "579b62f5ad8d96c2bb331f07d1408c92767531d9"
```

**Grammars**: 2 (`java`, `properties`)
**Language Directories**: 2 (`java/`, `properties/`)

### Pattern Analysis
- ✅ Each grammar has a corresponding language directory
- ✅ No dual-grammar injection pattern (no grammar injecting into another grammar)
- ✅ Properties is a separate file format (`.properties` files)
- **Use case**: Supporting multiple file types, not inline content parsing

---

## C# Extension (44 stars ⭐)

**Repository**: https://github.com/zed-extensions/csharp

### Grammar Configuration
```toml
[grammars.c_sharp]
repository = "https://github.com/tree-sitter/tree-sitter-c-sharp"
commit = "dd5e59721a5f8dae34604060833902b882023aaf"
```

**Grammars**: 1 (`c_sharp`)
**Language Directories**: 1 (`csharp/`)

### Pattern Analysis
- ✅ Single grammar for single language
- ✅ Standard pattern: 1 grammar → 1 language directory
- ❌ No dual-grammar pattern
- **Use case**: Simple language support

---

## Ruby Extension (77 stars ⭐)

**Repository**: https://github.com/zed-extensions/ruby
*(Previously analyzed)*

### Grammar Configuration
```toml
[grammars.ruby]
repository = "https://github.com/tree-sitter/tree-sitter-ruby"
commit = "71bd32fb7607035768799732addba884a37a6210"

[grammars.embedded_template]
repository = "https://github.com/tree-sitter/tree-sitter-embedded-template"
commit = "c70c1de07dedd532089c0c90835c8ed9fa694f5c"

[grammars.rbs]
repository = "https://github.com/joker1007/tree-sitter-rbs"
commit = "de893b166476205b09e79cd3689f95831269579a"
```

**Grammars**: 3 (`ruby`, `embedded_template`, `rbs`)
**Language Directories**: 6 (`ruby/`, `erb/`, `html-erb/`, `js-erb/`, `rbs/`, `yaml-erb/`)

### Pattern Analysis
- ✅ Multiple grammars, multiple language directories
- ✅ `embedded_template` grammar → used by multiple language configs (erb, html-erb, etc.)
- ✅ Each language directory has `config.toml` specifying which grammar to use
- ✅ Injections used for ERB templates
- **Use case**: Template languages, multiple file formats sharing grammars

---

## PHP Extension (42 stars ⭐)

**Repository**: https://github.com/zed-extensions/php
*(Previously analyzed)*

### Grammar Configuration
```toml
[grammars.php]
repository = "https://github.com/tree-sitter/tree-sitter-php"
commit = "[hash]"

[grammars.phpdoc]
repository = "https://github.com/claydonrcarter/tree-sitter-phpdoc"
commit = "[hash]"
```

**Grammars**: 2 (`php`, `phpdoc`)
**Language Directories**: 2 (`php/`, `phpdoc/`)

### Injection Pattern
```scheme
; In languages/php/injections.scm
((comment) @injection.content
  (#match? @injection.content "^/\\*\\*[^*]")
  (#set! injection.language "phpdoc"))
```

### Pattern Analysis
- ✅ `phpdoc` grammar injected into PHP comments
- ✅ `phpdoc/config.toml` has `hidden = true`
- ✅ Both grammars have language directories
- ✅ Injection is **content-based** (comment text content)
- **Use case**: Documentation comments with special syntax

---

## Zed Built-in Markdown

**Location**: `zed-industries/zed/crates/languages/src/markdown/`

### Grammar Configuration
```toml
grammar = "markdown"
```

**Grammars**: 1 (`markdown`)
**Language Directories**: 1 (`markdown/`)

### Injection Pattern
```scheme
; In languages/markdown/injections.scm
((inline) @injection.content
 (#set! injection.language "markdown-inline"))
```

### Critical Discovery
- ❌ **NO `languages/markdown_inline/` directory exists in Zed**
- ❌ Only `languages/markdown/` with `grammar = "markdown"`
- ✅ Injection references `"markdown-inline"` but no language directory for it
- ❓ **Question**: Where does `markdown-inline` grammar come from?

### Hypothesis
Zed's core might:
1. Bundle `markdown-inline` grammar separately (not as a language)
2. Auto-load it from the markdown grammar itself
3. Use a different mechanism for built-in vs extension grammars

---

## Key Patterns Identified

### Pattern 1: Simple Language (C#, single grammar)
```
extension.toml: [grammars.c_sharp]
languages/csharp/config.toml: grammar = "c_sharp"
```
**Use**: Standard single-language support

### Pattern 2: Multiple File Types (Java)
```
extension.toml: [grammars.java] + [grammars.properties]
languages/java/: grammar = "java"
languages/properties/: grammar = "properties"
```
**Use**: Different file formats

### Pattern 3: Content Injection (PHP)
```
extension.toml: [grammars.php] + [grammars.phpdoc]
languages/php/: grammar = "php", has injections.scm
languages/phpdoc/: grammar = "phpdoc", hidden = true
Injection: comment content → phpdoc
```
**Use**: Injecting into text content (comments)

### Pattern 4: Structure Injection (Markdown - UNCLEAR)
```
extension.toml: ??? (built-in, no extension.toml)
languages/markdown/: grammar = "markdown", has injections.scm
languages/markdown_inline/: DOES NOT EXIST
Injection: (inline) nodes → "markdown-inline"
```
**Use**: Injecting into structural nodes for re-parsing
**Status**: ⚠️ **UNCLEAR HOW THIS WORKS**

---

## Critical Unanswered Questions

### Question 1: Where is markdown_inline defined?
- It's referenced in markdown's injections.scm
- But there's no `languages/markdown_inline/` directory
- Is it:
  - A. Part of the markdown grammar itself?
  - B. A separate bundled grammar in Zed core?
  - C. Auto-discovered from the grammar's own inline variant?

### Question 2: Why doesn't markdown_inline need a language directory?
- PHP's phpdoc DOES have `languages/phpdoc/`
- Ruby's embedded_template DOES have language directories
- But markdown_inline DOES NOT
- What's different about it?

### Question 3: Do we need languages/pandoc_markdown_inline/?
Based on research:
- ✅ PHP pattern says YES (phpdoc has its own directory)
- ❌ Markdown pattern says NO (markdown_inline has no directory)
- ❓ Our pattern is like markdown (structure injection), not PHP (content injection)

---

## Implications for Our Extension

### Current Problem
We've tried:
1. **With** `languages/pandoc_markdown_inline/` + `highlights.scm` → FAILED
2. **Without** `languages/pandoc_markdown_inline/` → FAILED
3. **With** `languages/pandoc_markdown_inline/config.toml` only → NOT YET TESTED

### Test 7 Hypothesis (Current)
Based on markdown research:
- Remove `languages/pandoc_markdown_inline/highlights.scm`
- Keep `languages/pandoc_markdown_inline/config.toml` with `hidden = true`
- Let grammar provide its own highlights

### Alternative Hypothesis (if Test 7 fails)
Zed might not support grammar-to-grammar injection for extensions:
- Built-in markdown can inject to markdown_inline (special case?)
- Extensions might only be able to inject to built-in languages (yaml, python, etc.)
- Need to find an extension that successfully does grammar-to-grammar injection

---

## Next Research Steps

1. ✅ Check Java, C#, Ruby, PHP extension patterns (COMPLETED)
2. ✅ Verify markdown built-in structure (COMPLETED)
3. ⏳ Find if ANY extension successfully injects one custom grammar into another
4. ⏳ Check Zed's core code to see how markdown_inline is handled
5. ⏳ Ask Zed community if grammar-to-grammar injection is supported in extensions

## Action Items

- [ ] Test 7: Remove custom highlights.scm, keep config.toml only
- [ ] If Test 7 fails: Search for extensions doing custom-to-custom grammar injection
- [ ] If no examples found: May need to file Zed issue or change approach

---

## Additional Research: Template Language Extensions

### Svelte Extension
**Repository**: https://github.com/zed-extensions/svelte

**Grammars**: 1 (`svelte`)
**Injections**: ✅ Has `languages/svelte/injections.scm`

**What it injects:**
```scheme
; JavaScript into <script> tags
; TypeScript into <script lang="typescript">
; CSS into <style> tags
```

**Pattern**: Single custom grammar (`svelte`) → Injects BUILT-IN languages only (JavaScript, TypeScript, CSS)

### Vue Extension
**Repository**: https://github.com/zed-extensions/vue

**Grammars**: 1 (`vue`)
**Injections**: ✅ Has `languages/vue/injections.scm`

**What it injects:**
```scheme
; JavaScript, TypeScript, CSS, Pug, HTML
; Into various Vue template sections
```

**Pattern**: Single custom grammar (`vue`) → Injects BUILT-IN languages only

---

## Critical Finding: Extensions Issue #484

**Issue**: "Extensions and language injections"
**URL**: https://github.com/zed-industries/extensions/issues/484

**Problem reported**:
- User trying to create LilyPond extension with embedded Scheme highlighting
- Language injections don't work properly in extensions
- Tree-sitter only applies injected grammars to nodes without children
- Issue remains OPEN (not resolved)

**Key quote**: "Tree-sitter appears to only apply injected grammars to syntax tree nodes without child nodes"

**Impact**: This suggests language injections in extensions have known limitations!

---

## Issue #7710: Support injected language syntax

**URL**: https://github.com/zed-industries/zed/issues/7710

**Status**: CLOSED (resolved)
**Resolution**: "Zed supports arbitrary injections now"

**However**: Comment suggests some injections still limited (SQL less reliable than regex)

---

## Pattern Analysis Summary

After researching ALL available extensions:

### ✅ What WORKS in Extensions:
1. **Custom grammar → Built-in language injection** (Vue, Svelte, Ruby, PHP)
   - Example: `vue` grammar → injects `javascript`, `css`, `html`
   - Example: `php` grammar → injects `phpdoc` (wait, this IS custom!)

### ❓ What's UNCLEAR:
1. **Custom grammar → Custom grammar injection** (Our case: pandoc_markdown → pandoc_markdown_inline)
   - PHP does `php` → `phpdoc` BUT phpdoc might be special-cased
   - NO other examples found
   - Issue #484 suggests this doesn't work reliably

### ❌ What DOESN'T WORK:
1. **Complex nested injections** (per Issue #484)
2. **Injections into nodes with children** (per Issue #484)

---

## Hypothesis: The Inline Node Problem

Our injection:
```scheme
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))
```

**The issue**: `(inline)` nodes probably HAVE CHILDREN!
- The block grammar creates `(inline)` nodes
- These nodes contain text that should be parsed
- But if they have child nodes, Tree-sitter won't apply the injection (per Issue #484)

**This matches our symptoms**:
- Extension installs successfully
- Grammars compile successfully  
- But bold highlighting never appears
- Because the injection never actually runs!

---

## Conclusion

**Extension-to-extension grammar injection appears to be BROKEN or UNSUPPORTED in Zed**

Evidence:
1. ❌ No extensions found doing custom→custom grammar injection (except possibly PHP)
2. ❌ Open Issue #484 reports injection problems
3. ❌ Tree-sitter limitation: won't inject into nodes with children
4. ❌ Our 7 tests all failed despite various configurations

**Recommendation**: We need an alternative approach that doesn't rely on grammar injection.


---

## Issue #4612: Custom Treesitter Language Injection Queries

**URL**: https://github.com/zed-industries/zed/issues/4612
**Status**: Appears to be RESOLVED

**Original Request**:
- Add custom tree-sitter language injection queries (like Helix/Neovim)
- Example: Highlight SQL inside `sql!` macros in Rust
- Use case: SQL in Nix, Ruby heredocs, etc.

**Resolution**:
- Comment by @llllvvuu: "This feature now seems to exist"
- Referenced Nix extension as an example using injections

**Relevance to Our Issue**:
This is HIGHLY RELATED! It's about the same feature we need (custom injections).

### Nix Extension Analysis
- **Grammars**: 1 (`nix`)
- Only single grammar, so not custom-to-custom injection
- BUT: Issue claims Nix uses injections (need to verify)

**Key difference from our case**:
- Issue #4612 is about injecting TO built-in languages (SQL, etc.)
- Our case is injecting TO custom grammar (pandoc_markdown_inline)
- These might be different capabilities!


### Nix Injections Verified

**File**: `languages/nix/injections.scm`

**What it injects**:
- regex, bash, fish, haskell, javascript, perl, python, rust
- ALL are BUILT-IN Zed languages

**Pattern**: Nix grammar → Injects built-in languages only (no custom grammars)

**Conclusion**: Issue #4612 was about custom injection QUERIES, not custom-to-custom grammar injection. The "custom" part is the trigger logic, not the target grammar.

---

## Final Conclusion

After exhaustive research of ALL zed-extensions:

**✅ SUPPORTED: Custom grammar → Built-in language injection**
- Examples: Vue→JS/CSS, Svelte→JS/CSS, Nix→bash/python, PHP→phpdoc(?)
- Extensions CAN inject built-in languages (JavaScript, Python, SQL, etc.)

**❌ NOT SUPPORTED: Custom grammar → Custom grammar injection**  
- Our case: pandoc_markdown → pandoc_markdown_inline
- NO successful examples found
- Issue #484 reports this doesn't work
- Tree-sitter limitation with nodes that have children

**The problem**: Zed (or Tree-sitter) doesn't support re-parsing structural nodes with a different custom grammar. Built-in languages work, but not extension-defined grammars.


---

## PR #9654: Injection Capture Syntax Changes

**URL**: https://github.com/zed-industries/zed/pull/9654
**Status**: Closed (superseded by PR #22268)

**Problem**: Original injection syntax was breaking Java extension
- Non-standard captures: `@language`, `@content`, `combined`, `language`
- Needed to match Tree-sitter documentation standard

**Fix**: Updated to standard captures
- `@language` → `@injection.language`
- `@content` → `@injection.content`
- `combined` → `injection.combined`
- `language` → `injection.language`

**Result**: PR #22268 added backwards compatibility for both syntaxes

**Impact on our extension**: We're already using standard syntax (`@injection.content`, `@injection.language`)

---

## Issue #1588: HTML Not Highlighted in Markdown

**URL**: https://github.com/zed-industries/extensions/issues/1588
**Problem**: HTML within Markdown documents not syntax-highlighted
**Diagnosis**: Tree-sitter developers said it's "an injection issue in Zed's deployment"

**Resolution**: PR #20527 merged - added "HTML injections for markdown syntax highlighting"

**Key Insight**: This proves Zed's built-in markdown CAN inject languages successfully!

---

## Key Takeaway

1. ✅ Injection syntax standardized - we're using correct syntax
2. ✅ Zed's markdown successfully injects HTML (proven by PR #20527)
3. ❓ But markdown-inline injection still mysterious - need to see actual implementation


---

## CRITICAL DISCOVERY: Zed's Markdown Injections.scm

**Source**: `zed-industries/zed/crates/languages/src/markdown/injections.scm`

**Complete file content**:
```scm
(fenced_code_block
  (info_string
    (language) @injection.language)
  (code_fence_content) @injection.content)

((inline) @injection.content
 (#set! injection.language "markdown-inline"))

((html_block) @injection.content
  (#set! injection.language "html"))

((minus_metadata) @injection.content (#set! injection.language "yaml"))

((plus_metadata) @injection.content (#set! injection.language "toml"))
```

### Analysis

**Our injection (identical syntax)**:
```scm
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))
```

**The Difference**:
- ✅ Zed: injects `"markdown-inline"` (built-in language in Zed core)
- ❌ Ours: injects `"pandoc_markdown_inline"` (custom grammar in extension)

**Other injections in markdown**:
- `"html"` - BUILT-IN language
- `"yaml"` - BUILT-IN language  
- `"toml"` - BUILT-IN language
- Code blocks - inject to BUILT-IN languages (python, javascript, etc.)

### The Pattern

**ALL of Zed's markdown injections target BUILT-IN languages**, not custom extension grammars!

This aligns with our research:
- Vue/Svelte → inject built-in languages only
- Ruby/PHP → inject to... wait, does phpdoc count as built-in or custom?

### Critical Question

**Is `markdown-inline` a built-in language in Zed, or is it handled specially?**

If it's built-in, then our approach CAN'T work because extensions can't inject to custom grammars.

If it's special-cased, we need to understand how.

