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

## Investigation Needed

### 1. Find the Injection Resolution Code

**Location**: Likely in `crates/language/src/` or `crates/extension/src/`

**Key questions**:
- Where does Zed resolve `injection.language` strings to actual grammars?
- How does it look up grammar names?
- Does it check extension-defined grammars?

**Files to examine**:
```
zed-industries/zed/
├── crates/language/
│   ├── src/
│   │   ├── language.rs
│   │   ├── grammar.rs
│   │   ├── injection.rs (if exists)
├── crates/extension/
│   ├── src/
│   │   ├── extension_builder.rs
│   │   ├── extension_host.rs
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

### 3. Check Extension Grammar Loading

**Questions**:
- Are extension grammars loaded into a registry?
- Can they be queried by name?
- What's the lifecycle of extension grammar loading?

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

### Step 2: Find Relevant Issues/PRs

Search Zed repository for:
- Closed PRs about injections
- Issues mentioning extension grammars
- Comments about injection resolution

**Already known**:
- Issue #484 - Reports injection problems
- PR #9654 - Updated injection syntax
- Issue #4612 - Custom injection queries (resolved)

### Step 3: Examine Built-in Markdown

**How does markdown-inline work?**
- Is it a separate language in Zed's core?
- Or is it part of the markdown grammar package?
- Where is it registered?

```bash
# Search for markdown-inline in Zed codebase
rg "markdown-inline" --type rust
rg "markdown_inline" --type rust

# Check language definitions
ls crates/languages/src/
```

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

**Modifying Zed is feasible and valuable**, but:
- Requires 2-4 weeks
- Success depends on Zed team acceptance
- Users need fix now

**Recommendation**:
1. Ship quick fix (merged highlights) first
2. Then pursue Zed modification as proper long-term solution
3. This way users aren't blocked, and we still contribute to Zed

**This approach**:
✅ Delivers working extension immediately
✅ Contributes to Zed ecosystem
✅ Provides fallback if PR is rejected
✅ Best for all stakeholders
