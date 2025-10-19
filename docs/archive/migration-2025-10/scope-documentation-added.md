# Documentation References Added to highlights.scm

## Summary

Added comprehensive documentation comments to the top of `languages/quarto/highlights.scm` to help future maintainers understand and validate Zed scope usage.

## What Was Added

### 1. Documentation Section (Lines 6-30)

References to key documentation files:

```scheme
; Complete list of Zed-supported scopes:
;   docs/zed-syntax-scopes.md

; Scope validation tests:
;   tests/zed_scope_validation.rs
;   Run: cargo test --test zed_scope_validation

; Scope usage summary:
;   docs/scope-validation-summary.md

; Original scope naming decision:
;   docs/scope-naming-decision.md
```

### 2. Key Scope Conventions (Lines 32-49)

Side-by-side comparison table showing Zed vs nvim-treesitter scope names:

```scheme
; Zed uses different scope names than nvim-treesitter:
;   Zed                    nvim-treesitter
;   ----                   ---------------
;   @title                 @markup.heading
;   @emphasis              @markup.italic
;   @emphasis.strong       @markup.bold
;   @text.literal          @markup.raw.inline / @markup.raw.block
;   @link_text.markup      @markup.link.label
;   @link_uri.markup       @markup.link.url
;   ...
```

This table:
- Shows at-a-glance what scopes to use
- Prevents accidentally using nvim-treesitter conventions
- Documents the most commonly confused scope mappings

### 3. Grammar Information (Lines 51-57)

Links to the grammar source and specification:

```scheme
; Syntax highlighting queries for tree-sitter-quarto
; Grammar: https://github.com/ck37/tree-sitter-quarto
; Based on: openspec/specs/language-injection/spec.md
```

## Benefits

### For Future Maintainers

1. **Quick reference**: See scope mappings without leaving the file
2. **Testing guidance**: Know how to validate changes (`cargo test --test zed_scope_validation`)
3. **Documentation discovery**: Find detailed docs in `/docs` directory
4. **Convention awareness**: Understand Zed vs nvim-treesitter differences

### For Contributors

1. **Clear guidelines**: Know what scope names to use
2. **Validation process**: Understand how scopes are tested
3. **Historical context**: See why certain decisions were made
4. **Error prevention**: Warning against common mistakes (e.g., using `@markup.*`)

## Documentation Structure

The comments create a clear documentation hierarchy:

```
highlights.scm (this file)
├─> docs/zed-syntax-scopes.md         # Complete scope reference
├─> tests/zed_scope_validation.rs     # Automated validation
├─> docs/scope-validation-summary.md  # Usage analysis
└─> docs/scope-naming-decision.md     # Original rationale
```

## Example Use Cases

### Scenario 1: Adding New Syntax Element

A contributor wants to add highlighting for a new Pandoc feature:

1. **Check scope conventions** in the header comments
2. **Find appropriate scope** from the Zed column in the table
3. **Add query** using validated scope name
4. **Run validation**: `cargo test --test zed_scope_validation`
5. **Reference docs** if validation fails

### Scenario 2: Investigating Highlighting Issue

A user reports that something isn't highlighting correctly:

1. **Check the scope mapping table** to ensure correct Zed scope is used
2. **Review docs/zed-syntax-scopes.md** for scope details
3. **Check docs/scope-validation-summary.md** for known issues
4. **Run `cargo test list_all_used_scopes`** to see current usage

### Scenario 3: Migrating from Another Editor

Someone familiar with nvim-treesitter conventions:

1. **See the comparison table** immediately in comments
2. **Understand Zed uses different conventions**
3. **Know where to find complete mapping** (docs/zed-syntax-scopes.md)
4. **Avoid common mistakes** highlighted in IMPORTANT note

## Testing

All tests pass with the new comments:

```bash
$ cargo test --test zed_scope_validation
running 5 tests
test no_nvim_treesitter_scopes ... ok
test uses_recommended_markdown_scopes ... ok
test documents_all_used_scopes ... ok
test all_scopes_are_zed_compatible ... ok
test list_all_used_scopes ... ok
```

The `no_nvim_treesitter_scopes` test correctly filters out comment lines, so the comparison table doesn't trigger false positives.

## Future Maintenance

When updating the file:

1. **Keep comments in sync** with actual documentation files
2. **Update scope table** if new conventions emerge
3. **Add notes** for any non-obvious scope choices
4. **Run tests** before committing changes

## Commit Message Suggestion

```
docs: add comprehensive scope documentation to highlights.scm

- Add references to docs/zed-syntax-scopes.md and validation tests
- Include Zed vs nvim-treesitter scope comparison table
- Link to grammar source and specification
- Provide quick validation command for contributors

This makes it easier for future maintainers to understand scope
conventions and validate their changes without searching through
multiple documentation files.
```

## Related Files

- `languages/quarto/highlights.scm` - Updated with documentation comments
- `docs/zed-syntax-scopes.md` - Complete scope reference
- `tests/zed_scope_validation.rs` - Validation tests
- `docs/scope-validation-summary.md` - Comprehensive analysis
- `docs/scope-naming-decision.md` - Original decision rationale
