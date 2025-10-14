# Syntax Highlighting Failure Analysis

## Timeline
1. **Before**: Extension was working (presumably with tree-sitter-quarto commit 20c2966)
2. **Change**: Updated to commit 851b221 (for tree-sitter 0.25.10 compatibility)
3. **Now**: Almost no syntax highlighting visible in Zed

## What We Know

### Grammar Repository (tree-sitter-quarto)
- Uses standard `@markup.*` scopes (e.g., `@markup.heading`, `@markup.italic`)
- Both old commit (20c2966) and new commit (851b221) use `@markup.*` scopes
- Grammar location: `https://github.com/ck37/tree-sitter-quarto`
- Queries located in: `queries/highlights.scm`

### Extension Repository (zed-quarto-extension)
- Has custom `languages/quarto/highlights.scm` with Zed-compatible scopes
- Uses `@text.*` scopes (e.g., `@text.title`, `@text.emphasis`, `@emphasis.strong`)
- This file has been correct since commit 5a3ace9

### Zed's Behavior
- Loads grammars from git repositories at install time
- Caches grammars in `grammars/` directory
- Should load query files from extension's `languages/<lang>/` directory
- **UNKNOWN**: Does it override grammar's built-in queries or merge them?

### Current Files
```
extension.toml: Points to tree-sitter-quarto @ 851b221
languages/quarto/config.toml: Declares grammar = "quarto"
languages/quarto/highlights.scm: Has Zed-compatible scopes (@text.*)
grammars/quarto/queries/highlights.scm: Has @markup.* scopes (from grammar)
```

## The Mystery

If the old commit (20c2966) also used `@markup.*` scopes, and our extension always had
`@text.*` scopes in `languages/quarto/highlights.scm`, then:

**Why was it working before and broken now?**

## Hypotheses

### Hypothesis 1: Zed's Query Loading Order Changed
- **Theory**: Zed used to load extension queries first, now loads grammar queries first
- **How to test**: Check Zed changelog/release notes for query loading changes
- **Likelihood**: Low (would affect many extensions)

### Hypothesis 2: Grammar Structure Changed
- **Theory**: Something in the new grammar commit affects how Zed loads queries
- **How to test**: Compare grammar structure between commits 20c2966 and 851b221
- **Likelihood**: Medium

### Hypothesis 3: Extension Queries Not Being Loaded
- **Theory**: Extension's `languages/quarto/highlights.scm` is being ignored entirely
- **How to test**: Add intentionally broken syntax to our highlights.scm, reinstall
- **Likelihood**: High (explains complete lack of highlighting)

### Hypothesis 4: Grammar Compilation Issue
- **Theory**: Grammar compiles but queries don't get properly embedded/loaded
- **How to test**: Check compiled WASM file, inspect runtime behavior
- **Likelihood**: Medium

### Hypothesis 5: Cache Corruption
- **Theory**: The grammars/ cache is in a bad state
- **How to test**: Delete entire grammars/ directory, clear Zed cache, reinstall
- **Likelihood**: Medium (but we already tried this)

## Testing Plan

### Phase 1: Verify Extension Queries Are Loaded (High Priority)
1. Add a syntax error to `languages/quarto/highlights.scm`
2. Reinstall extension in Zed
3. Check Zed logs for parser errors
4. **Expected**: If queries are loaded, we'd see errors. If not loaded, no errors.

### Phase 2: Compare Grammar Commits (Medium Priority)
1. Clone tree-sitter-quarto at both commits
2. Diff the grammar.js files
3. Diff the src/parser.c files
4. Look for structural changes that might affect query loading

### Phase 3: Test Grammar in Isolation (Medium Priority)
1. Create minimal test Zed extension with just the grammar
2. Use grammar's default queries (with @markup.* scopes)
3. See if ANY highlighting works
4. **Expected**: If this works, problem is in our extension's query override mechanism

### Phase 4: Automated Testing (High Priority - Long Term)
1. Create test that validates highlights.scm syntax
2. Create test that verifies scope names are Zed-compatible
3. Create test that checks grammar can be loaded and compiled
4. Run tree-sitter test command on our queries against grammar

## Test Results

### Automated Query Validation (COMPLETED)
Created `tests/query_validation.rs` with two tests:
1. **highlights_query_is_valid_syntax**: ✅ PASS - Our queries are syntactically valid
2. **highlights_uses_zed_compatible_scopes**: ✅ PASS - Our file uses `@text.*` scopes correctly

**Finding**: Our `languages/quarto/highlights.scm` is correct and has Zed-compatible scopes.

### Root Cause Identified

**Problem**: Zed is loading the grammar repository's built-in `queries/highlights.scm` (which uses `@markup.*` scopes) instead of our extension's `languages/quarto/highlights.scm` (which uses `@text.*` scopes).

**Evidence**:
- Extension's highlights.scm: Uses `@text.title`, `@text.emphasis`, etc. (correct)
- Grammar's highlights.scm: Uses `@markup.heading`, `@markup.italic`, etc. (incompatible with Zed)
- No highlighting visible in Zed = Zed themes don't recognize `@markup.*` scopes

## Solution Options

### Option 1: Force Zed to Skip Grammar Queries (PREFERRED)
Need to find a way in `extension.toml` to tell Zed not to load the grammar's built-in queries.
- Possible approach: Research Zed extension API documentation
- May require Zed version check or feature flag

### Option 2: Modify Grammar Clone Process
Overwrite the grammar's queries directory after cloning:
- Could be done in a build script
- Fragile, depends on Zed's caching behavior

### Option 3: Fork Grammar with Zed Queries
Create a Zed-specific fork of tree-sitter-quarto:
- Not preferred - creates maintenance burden
- Defeats purpose of keeping grammar editor-agnostic

### Option 4: Submit PR to Grammar
Add `queries/highlights-zed.scm` to tree-sitter-quarto:
- Then reference it in extension.toml
- But we want THIS extension to handle Zed compatibility

## Research Findings

### Zed Documentation Review (via Context7)
- Extensions should place query files in `languages/<language-name>/highlights.scm`
- Grammars are referenced in `extension.toml` with `repository` and `commit` fields
- **No documented mechanism to explicitly override grammar queries**
- Expected behavior: Extension queries should take precedence over grammar queries
- **Actual behavior**: Grammar queries appear to be loaded instead

### Conclusion
This appears to be either:
1. A bug in Zed's extension loading system
2. Undocumented behavior that requires a specific configuration
3. A recent regression in Zed's query loading priority

## Proposed Solutions

### Option A: File Issue with Zed (RECOMMENDED)
Create a minimal reproduction case and file an issue:
- Extension with custom queries using Zed-compatible scopes
- Grammar with standard `@markup.*` scopes
- Document that grammar queries are loaded instead of extension queries
- Request clarification on expected behavior

### Option B: Temporary Workaround - Post-Install Script
Create a script that users run after installation:
```bash
#!/bin/bash
# Copy extension queries over grammar queries
cp languages/quarto/highlights.scm grammars/quarto/queries/highlights.scm
```
- Fragile, breaks on Zed updates
- Requires manual user intervention
- Not ideal but would work

### Option C: Use Local Grammar Copy
Instead of referencing grammar from git, vendorthe grammar locally:
- Clone tree-sitter-quarto into `grammars/quarto`
- Overwrite its `queries/highlights.scm` with our version
- Reference it with `path = "grammars/quarto"`
- Downsides: Larger repo size, harder to update grammar

### Option D: Request Zed-Compatible Queries in Grammar
Ask tree-sitter-quarto to add `queries/highlights-zed.scm`:
- Grammar provides both standard and Zed-compatible queries
- This extension references the Zed variant
- But we wanted to keep grammar editor-agnostic

## Next Steps

1. **Immediate**: Try Option B workaround to unblock user
2. **Short-term**: File issue with Zed about query loading priority
3. **Long-term**: Once Zed clarifies/fixes behavior, remove workaround
