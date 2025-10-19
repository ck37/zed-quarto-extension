# Testing Zed's Query Loading Mechanism

## Goal
Determine if Zed's `extension.toml` supports specifying custom query file names (like `highlights-zed.scm` instead of `highlights.scm`).

## What We Know

### From Source Code Investigation

1. **GrammarManifestEntry fields** (`extension_manifest.rs`):
   - `repository`: Grammar repository URL
   - `rev` (or `commit`): Git commit SHA
   - `path`: Optional subdirectory within grammar repo for parser source files

2. **No explicit query file name configuration**:
   - No field for specifying custom query file names found in schema
   - `path` is for grammar source files (parser.c, scanner.c), not queries

3. **Query loading** (`language.rs`):
   - Queries loaded via `LanguageQueries` struct
   - Methods: `with_highlights_query()`, `with_brackets_query()`, etc.
   - Takes query source strings, not file paths

### Standard Zed Behavior

Based on documentation and conventions:
- Extension queries go in `languages/<language-name>/` directory
- File names are standard: `highlights.scm`, `injections.scm`, `indents.scm`, etc.
- Zed should prefer extension queries over grammar's built-in queries

### The Problem

**Zed is loading grammar's built-in queries** instead of extension's queries:
- Grammar has `queries/highlights.scm` with `@markup.*` scopes
- Extension has `languages/quarto/highlights.scm` with `@text.*` scopes
- Result: No highlighting (Zed themes don't support `@markup.*`)

## Experiments to Try

### Experiment 1: Try undocumented fields in extension.toml

Test if any of these work:

```toml
[grammars.quarto]
repository = "https://github.com/ck37/tree-sitter-quarto"
commit = "b1b4cbd88fc6f787c660bf52b0e23879a8fc66c2"

# Try various potential field names:
queries_path = "queries-zed"           # Maybe custom queries directory?
highlights_query = "highlights-zed"     # Maybe custom query file name?
query_prefix = "zed"                    # Maybe adds prefix to query files?
use_grammar_queries = false             # Maybe disables grammar queries?
```

**Status:** Not tested yet - need to try each and see if Zed accepts it

### Experiment 2: Create highlights-zed.scm in grammar repo

1. Fork tree-sitter-quarto
2. Add `queries/highlights-zed.scm` with Zed-compatible scopes
3. Update extension to reference the fork
4. See if Zed automatically detects `-zed.scm` variant

**Status:** Possible but requires grammar repo changes

### Experiment 3: Use local grammar with file:// URL

```toml
[grammars.quarto]
repository = "file:///tmp/tree-sitter-quarto-test"
commit = "HEAD"
```

Then manually modify the grammar's `queries/highlights.scm` to use Zed scopes.

**Status:** This would work but is not a sustainable solution

### Experiment 4: Check if extension queries actually override

Current theory: Zed SHOULD load extension queries, but might have a bug.

Test:
1. Add intentional syntax error to `languages/quarto/highlights.scm`
2. Reinstall extension
3. See if Zed reports query syntax error (proves it's loading extension queries)
4. If no error, confirms Zed is loading grammar queries instead

**Status:** Should try this next

## Expected Behavior vs Actual Behavior

### Expected (per Zed documentation)
- Extension provides `languages/<lang>/highlights.scm`
- Zed uses extension's queries for highlighting
- Grammar's built-in queries are ignored/overridden

### Actual (what we're seeing)
- Extension provides `languages/quarto/highlights.scm` with Zed scopes
- Zed loads grammar's `queries/highlights.scm` with standard scopes
- Result: Wrong scopes used, no highlighting

## Conclusion

**No built-in support for custom query file names found.**

The `extension.toml` schema only supports:
- `repository`: Where to get the grammar
- `commit`/`rev`: Which commit to use
- `path`: Subdirectory for parser source (not queries)

**Next steps:**
1. Test Experiment 4 to confirm Zed is loading grammar queries
2. File issue with Zed about extension query override not working
3. Use workaround script (`fix-zed-queries.sh`) until fixed
4. Consider PR to tree-sitter-quarto with Zed-compatible queries

## References

- [Zed extension manifest schema](https://github.com/zed-industries/zed/blob/main/crates/extension/src/extension_manifest.rs)
- [Zed language queries](https://github.com/zed-industries/zed/blob/main/crates/language/src/language.rs)
- [Extension documentation](https://zed.dev/docs/extensions/languages)
