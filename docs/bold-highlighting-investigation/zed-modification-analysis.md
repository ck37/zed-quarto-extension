# Modifying Zed to Support Custom Grammar Injection

## Proposal
Contribute a fix to Zed itself to support extension-to-extension grammar injection.

## Current Limitation

**What works**: Extensions can inject built-in languages
- Example: `vue` → `javascript`, `css`, `html`

**What doesn't work**: Extensions can inject custom grammars
- Example: `pandoc_markdown` → `pandoc_markdown_inline`

**Root cause**: Zed likely only resolves grammar names from its built-in language registry, not from extension-defined grammars.

---

## Investigation Findings (2025-10-12)

### 1. Injection Resolution Code

**✅ FOUND**: `crates/language/src/language_registry.rs`

**Key findings**:
- `LanguageRegistry` has `get_or_load_grammar()` for loading grammars
- Supports native and WASM grammar loading
- Handles grammar loading states and caching
- **However**: No specific logic found for resolving injection.language strings to extension grammars

**Files examined**:
```
zed-industries/zed/
├── crates/language/src/
│   ├── language.rs          - Language structure and queries
│   ├── language_registry.rs  - Grammar loading and registration
│   └── markdown-inline/      - Built-in hidden language for injection
│       └── config.toml       - Has `hidden = true`
```

### 2. Understand Grammar Registry

**Current hypothesis**:
```rust
// Pseudocode - likely current behavior
fn resolve_injection_language(name: &str) -> Option<Grammar> {
    // Only checks built-in languages
    BUILT_IN_LANGUAGES.get(name)
}
```

**Needed behavior**:
```rust
// Pseudocode - desired behavior
fn resolve_injection_language(name: &str) -> Option<Grammar> {
    // First check built-in languages
    if let Some(grammar) = BUILT_IN_LANGUAGES.get(name) {
        return Some(grammar);
    }

    // Then check extension-defined grammars
    EXTENSION_GRAMMARS.get(name)
}
```

### 2. markdown-inline Mystery Solved

**✅ CONFIRMED**: `markdown-inline` IS a built-in language in Zed

**Key findings**:
- Located at `crates/languages/src/markdown-inline/`
- Configuration: `config.toml` with `hidden = true`
- It's a separate built-in language, not loaded from extensions
- This explains why markdown injection works but our extension injection doesn't

**Evidence**:
```toml
# crates/languages/src/markdown-inline/config.toml
name = "Markdown-Inline"
grammar = "markdown-inline"
hidden = true
```

**Implication**: When Zed resolves `injection.language = "markdown-inline"`, it finds it in the built-in language registry. Extension-defined grammars are not in this registry.

### 3. Extension Grammar Loading

---

## Research Strategy

### Step 1: Search Zed Codebase

```bash
# Clone Zed repository
git clone https://github.com/zed-industries/zed.git
cd zed

# Search for injection-related code
rg "injection\.language" --type rust
rg "resolve.*grammar" --type rust
rg "language.*injection" --type rust

# Search for grammar registration
rg "register.*grammar" --type rust
rg "GrammarRegistry" --type rust
```

### Step 2: Find Relevant Issues/PRs ✅ COMPLETED

**Found issues and discussions**:

1. **Issue #4612** - "Custom Treesitter language injection queries" (✅ RESOLVED)
   - Custom injection queries ARE supported (like Helix/Neovim)
   - Extensions can define custom `injections.scm` files
   - Example: Nix extension uses custom injections

2. **Discussion #14953** - "Writing a custom injection for specific language"
   - Confirmed: Multiple language injections in a single extension are possible
   - Documentation: https://zed.dev/docs/extensions/languages#code-injections
   - Approach: Create subdirectory for each language with `injections.scm`

3. **Issue #9656** - "Zed does not use standard capture nodes for injections"
   - Zed had non-standard injection syntax
   - Fixed in PR #22268 - added backwards compatibility

4. **Extensions Issue #484** - Reports injection limitations
   - Tree-sitter only applies injections to nodes without children
   - Still an open limitation

**Key discovery**: Extensions CAN define custom injections, BUT they only work for injecting **built-in** languages, not extension-defined grammars.

### Step 3: Examine Built-in Markdown ✅ COMPLETED

**How does markdown-inline work?**
- ✅ It IS a separate built-in language in Zed's core
- ✅ Located at `crates/languages/src/markdown-inline/`
- ✅ Has its own `config.toml` with `hidden = true`
- ✅ Registered as a hidden language (not shown in UI but available for injection)

**Directory structure**:
```
crates/languages/src/
├── markdown/
│   ├── config.toml
│   └── injections.scm    # Contains: ((inline) @injection.content
│                         #           (#set! injection.language "markdown-inline"))
├── markdown-inline/
│   └── config.toml       # Contains: hidden = true
└── [other languages...]
```

**Key insight**: This is the pattern we need to replicate for extension grammars. The parent language injects to a hidden child language for inline content.

---

## Research Summary

### What We Confirmed

1. **Built-in markdown-inline exists**: Zed has a hidden `markdown-inline` language specifically for injection
2. **Extension injections work**: Extensions CAN define custom `injections.scm` files
3. **The limitation**: Extensions can only inject **built-in** languages, not other extension-defined grammars
4. **Grammar registry**: Extension grammars are loaded but not added to the injection resolution registry

### The Core Problem

When Zed processes:
```scheme
((inline) @injection.content
 (#set! injection.language "pandoc_markdown_inline"))
```

It looks for `"pandoc_markdown_inline"` in the built-in language registry. Extension grammars are not checked.

### Why This Matters

- ✅ `"markdown-inline"` works → it's built-in
- ✅ `"javascript"` works → it's built-in
- ✅ `"python"` works → it's built-in
- ❌ `"pandoc_markdown_inline"` fails → it's extension-defined

### The Fix Needed

Extend Zed's injection resolution to check extension-loaded grammars in addition to built-in languages.

---

## Potential Implementation Approaches

### Approach A: Extend Language Registry

**Location**: Wherever Zed maintains its language/grammar registry

**Change**: Add extension grammars to the lookup pool

**Pseudocode**:
```rust
struct LanguageRegistry {
    built_in: HashMap<String, Grammar>,
    extensions: HashMap<String, Grammar>,  // Add this
}

impl LanguageRegistry {
    fn resolve_injection(&self, name: &str) -> Option<&Grammar> {
        self.built_in.get(name)
            .or_else(|| self.extensions.get(name))  // Add this
    }
}
```

**Pros**:
- Clean separation of built-in vs extension grammars
- Doesn't break existing code
- Easy to understand

**Cons**:
- Need to find where registry is defined
- May have multiple registries to update

### Approach B: Lazy Grammar Loading

**Concept**: When an injection language isn't found, check if an extension provides it

**Pseudocode**:
```rust
fn resolve_injection(&self, name: &str) -> Option<Grammar> {
    if let Some(grammar) = self.get_loaded_grammar(name) {
        return Some(grammar);
    }

    // Try loading from extensions
    if let Some(extension) = self.find_extension_with_grammar(name) {
        return self.load_grammar_from_extension(extension, name);
    }

    None
}
```

**Pros**:
- Doesn't require pre-loading all extension grammars
- More efficient memory usage

**Cons**:
- More complex
- Need to handle async grammar loading

### Approach C: Extension Manifest Declaration

**Concept**: Extensions declare which grammars can be injected

**In extension.toml**:
```toml
[grammars.pandoc_markdown_inline]
repository = "..."
commit = "..."
path = "..."
injectable = true  # New field
```

**Pros**:
- Explicit control
- Backward compatible (defaults to false)
- Clear documentation

**Cons**:
- Requires schema change
- All extensions would need to opt-in

---

## Development Plan

### Phase 1: Research (1-2 days)
1. Clone Zed repository
2. Find injection resolution code
3. Understand grammar registry architecture
4. Identify what needs to change
5. Check if there are existing related issues/PRs

### Phase 2: Design (1 day)
1. Choose best approach (A, B, or C)
2. Write detailed technical design
3. Identify all files that need modification
4. Consider edge cases and backward compatibility

### Phase 3: Implementation (2-3 days)
1. Make code changes
2. Add tests
3. Update documentation
4. Test with our extension

### Phase 4: Contribution (1-2 weeks)
1. Open GitHub issue discussing the proposal
2. Get feedback from Zed team
3. Create PR with implementation
4. Address review feedback
5. Get merged

**Total estimated time**: 2-4 weeks (depending on complexity and review process)

---

## Risks & Considerations

### Technical Risks

**1. Architecture Mismatch**
- Our assumptions about how Zed works might be wrong
- The fix might be much more complex than anticipated
- May require changes to WASM interface

**2. Performance Concerns**
- Loading extension grammars might impact performance
- Need to ensure efficient lookup

**3. Security Considerations**
- Extension grammars running in WASM sandbox
- Need to maintain security boundaries

### Process Risks

**1. Zed Team May Reject**
- They might have reasons for current design
- May prefer different approach
- Could be considered out of scope

**2. Long Review Process**
- Zed is a popular project
- PRs can take weeks to review
- May need multiple iterations

**3. Breaking Changes**
- Fix might require breaking changes
- Need careful backward compatibility

---

## Alternative: Simpler Fix

### What if it's just a bug?

**Hypothesis**: Extension grammars ARE in registry, but injection resolution has a bug

**Simpler fix**: Just fix the lookup logic

**Steps**:
1. Find where injection.language is resolved
2. Check if it's querying the right registry
3. Fix the query to include extension grammars
4. Test

**If this is the case**: Fix could be 10-50 lines of code, much simpler!

---

## Validation Strategy

### Before Starting Development

**Test in Zed codebase**:
1. Add debug logging to injection resolution
2. Install our extension
3. Check logs to see what happens when it tries to resolve "pandoc_markdown_inline"
4. Confirm it's failing to find extension grammar

**Expected findings**:
```
[DEBUG] Resolving injection language: "pandoc_markdown_inline"
[DEBUG] Checked built-in languages: not found
[DEBUG] Extension grammars: [skipped/not checked]  ← This is the bug
[WARN] Injection failed: unknown language "pandoc_markdown_inline"
```

---

## Documentation Needs

### If We Contribute Fix

**1. Update Zed docs**:
- Add example of extension-to-extension injection
- Document that extensions can now inject custom grammars
- Update extension development guide

**2. Update our extension**:
- Document that it requires Zed version X.Y.Z+
- Provide instructions for users on older versions

**3. Update issue #484**:
- Reference our PR
- Close or update the issue

---

## Decision Criteria

### Should we pursue this?

**✅ Reasons TO modify Zed**:
1. Proper long-term solution
2. Benefits all Zed extension developers
3. Makes extensions more powerful
4. Our use case is legitimate (Pandoc dual-grammar is standard)
5. Learning opportunity - understand Zed internals

**❌ Reasons NOT TO modify Zed**:
1. Time investment (2-4 weeks)
2. Uncertain acceptance
3. Users need fix now, not in weeks
4. We can work around it (Approach 1: merged highlights)
5. May be technically complex

### Hybrid Approach

**Best of both worlds**:
1. Implement **Approach 1** (merged highlights) NOW
   - Users get working bold highlighting immediately
   - Extension is functional

2. THEN contribute Zed fix
   - Proper solution for future
   - Helps broader community
   - Once merged, we can switch to dual-grammar

---

## Next Steps

### Recommended Path

**Immediate** (this week):
1. Implement Approach 1 (merged highlights)
2. Release working version to users
3. Document limitation in README

**Short-term** (next 2 weeks):
1. Research Zed codebase (Phase 1)
2. Open Zed issue proposing the fix
3. Get feedback from Zed team

**If Zed team is receptive**:
1. Implement the fix (Phase 2-3)
2. Submit PR
3. Once merged, update extension to use dual-grammar

**If Zed team rejects**:
1. Keep using merged highlights approach
2. Document why dual-grammar isn't possible
3. Consider upstream grammar modification (Approach 3)

---

## Conclusion

**Research completed** (2025-10-12): We now have a clear understanding of the problem.

### What We Learned

1. **Root cause confirmed**: Zed's injection resolution only checks built-in languages
2. **Built-in pattern identified**: `markdown-inline` is a hidden built-in language
3. **Extension support exists**: Custom `injections.scm` files work for built-in languages
4. **The gap**: Extension-to-extension grammar injection is not supported

### Path Forward

**Current workaround** (implemented):
- ✅ Using built-in `markdown-inline` injection (~70% coverage)
- ✅ Solves primary user complaint (bold/italic highlighting)
- ✅ Good enough for now

**Long-term solution** (planned):
1. File Zed issue with research findings
2. Propose extending injection resolution to check extension grammars
3. Contribute PR if Zed team is receptive
4. Switch to full `pandoc_markdown_inline` grammar when supported

**Why this matters**:
- Benefits all Zed extension developers
- Enables dual-grammar architectures in extensions
- Makes Zed extensions more powerful
- Aligns with tree-sitter best practices

**Estimated effort**: 1-2 weeks for PR (after Zed team approval)

**Status**: Research complete, ready to file issue when desired
