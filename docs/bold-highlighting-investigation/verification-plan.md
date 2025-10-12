# Verification Plan for Extension-to-Extension Injection Hypothesis

Before filing a Zed issue, we need to verify our hypothesis about why extension-to-extension grammar injection doesn't work.

## Current Hypothesis

**Most likely cause:** Extension grammars may not be loaded when injection resolution occurs.

**Specific hypothesis:**
- Built-in grammars: Compiled into Zed binary → immediately available in memory → injection works ✅
- Extension grammars: Loaded as WASM modules at runtime → may not be loaded when injection resolution runs → injection fails ❌

## Verification Procedure

### Phase 1: Code Investigation (Quickest - Start Here)

**Goal:** Examine Zed source code to understand actual injection resolution behavior.

**Steps:**

1. **Clone Zed repository:**
   ```bash
   git clone https://github.com/zed-industries/zed.git
   cd zed
   git checkout main  # or stable version
   ```

2. **Examine injection resolution code:**

   Key files to read (in order of importance):

   **a) `crates/language/src/syntax_map.rs`**
   - Function: `get_injections()` - Where injection resolution happens
   - Questions to answer:
     - Does it call any grammar loading functions?
     - How does it resolve language names to grammars?
     - Is there error handling for missing languages?

   ```bash
   # Find injection resolution code
   cat crates/language/src/syntax_map.rs | grep -A 50 "fn get_injections"
   ```

   **b) `crates/language/src/language_registry.rs`**
   - Function: `language_for_name_or_extension()` - Language lookup
   - Function: `get_or_load_grammar()` - Grammar loading
   - Questions to answer:
     - Does lookup trigger grammar loading?
     - Is there a difference between built-in and extension grammar lookup?
     - Are extension grammars loaded eagerly or lazily?

   ```bash
   # Find language lookup code
   cat crates/language/src/language_registry.rs | grep -A 30 "fn language_for_name_or_extension"

   # Find grammar loading code
   cat crates/language/src/language_registry.rs | grep -A 30 "fn get_or_load_grammar"
   ```

   **c) `crates/extension/src/extension_lsp.rs`**
   - Function: How extensions register grammars
   - Questions to answer:
     - When are extension grammars loaded?
     - Are they loaded eagerly at registration or lazily on demand?

   ```bash
   # Find extension grammar registration
   cat crates/extension/src/extension_lsp.rs | grep -A 20 "register_language"
   ```

3. **Key questions to answer from code reading:**
   - [ ] Does `language_for_name_or_extension()` call `get_or_load_grammar()`?
   - [ ] Does `get_injections()` trigger any grammar loading?
   - [ ] Is there a separate code path for built-in vs extension grammars?
   - [ ] Are extension grammars loaded eagerly (at registration) or lazily (on first use)?
   - [ ] Is there a check for "grammar loaded in memory" before injection?
   - [ ] How do built-in languages differ in their registration/availability?

**Expected outcome:**
- If we can see the exact code path, we'll know definitively whether our hypothesis is correct
- We may discover the actual reason (could be something we didn't think of)

---

### Phase 2: Build Zed with Debug Logging (If Phase 1 is inconclusive)

**Goal:** Add debug logging to see runtime behavior during injection resolution.

**Steps:**

1. **Add debug logging to injection resolution:**

   Edit `crates/language/src/syntax_map.rs`:
   ```rust
   pub fn get_injections(&self, ...) {
       // Add debug logging
       eprintln!("DEBUG: Resolving injection language: {}", language_name);

       if let Some(language) = self.language_registry.language_for_name(language_name) {
           eprintln!("DEBUG: Found language in registry: {}", language.name());
           eprintln!("DEBUG: Is extension grammar: {}", language.is_extension());
           eprintln!("DEBUG: Grammar loaded: {}", /* check if loaded */);
           // ... rest of code
       } else {
           eprintln!("DEBUG: Language NOT found in registry: {}", language_name);
       }
   }
   ```

2. **Add debug logging to language lookup:**

   Edit `crates/language/src/language_registry.rs`:
   ```rust
   pub fn language_for_name_or_extension(&self, name: &str) -> Option<Arc<Language>> {
       eprintln!("DEBUG: Looking up language: {}", name);
       eprintln!("DEBUG: Available languages: {:?}",
                 self.available_languages.iter().map(|l| l.name()).collect::<Vec<_>>());

       // ... existing code
   }
   ```

3. **Build Zed from source:**
   ```bash
   cargo build --release
   # Takes ~10-15 minutes first time
   ```

4. **Run Zed and test with Quarto extension:**
   ```bash
   ./target/release/zed
   ```

5. **Open a .qmd file and check terminal output:**
   - Look for: "Resolving injection language: pandoc_markdown_inline"
   - Check: Is it found in registry?
   - Check: Is grammar loaded?
   - Compare with built-in injection (markdown-inline)

**Expected outcome:**
- See exactly what happens at runtime when injection is attempted
- Confirm whether extension grammar is in registry but not loaded
- Or discover a different issue (not found in registry, wrong name, etc.)

---

### Phase 3: Minimal Test Case (If we need clear reproduction)

**Goal:** Create minimal extension to isolate the issue.

**Steps:**

1. **Create test extension structure:**
   ```
   test-injection-extension/
   ├── extension.toml
   ├── languages/
   │   ├── test_outer/
   │   │   ├── config.toml
   │   │   ├── highlights.scm
   │   │   └── injections.scm
   │   └── test_inner/
   │       ├── config.toml
   │       └── highlights.scm
   └── grammars/
       ├── test_outer/
       └── test_inner/
   ```

2. **Define minimal grammars:**
   - `test_outer`: Simple grammar with `(content)` nodes
   - `test_inner`: Simple grammar that highlights text
   - Both grammars as minimal as possible (easier to debug)

3. **Test injection:**
   ```scheme
   ; languages/test_outer/injections.scm
   ((content) @injection.content
    (#set! injection.language "test_inner"))
   ```

4. **Compare behaviors:**
   - Test 1: Inject built-in language (e.g., "javascript") → Should work ✅
   - Test 2: Inject extension language ("test_inner") → Hypothesis: fails ❌
   - Same extension, same node structure, only difference is target grammar type

**Expected outcome:**
- Clear minimal reproduction case
- Demonstrates issue is not specific to Pandoc/Quarto
- Easier for Zed team to reproduce and debug

---

### Phase 4: Engage with Zed Community (Optional)

**Goal:** Get feedback before filing formal issue.

**Steps:**

1. **Search existing resources:**
   - Check Zed Discord #extensions channel
   - Search Zed GitHub issues/discussions
   - Look for similar problems or solutions

2. **Ask informal question:**
   - "Has anyone successfully injected a custom extension grammar into another extension grammar?"
   - Share our findings from Phase 1-3
   - Get feedback from Zed developers

3. **Refine understanding:**
   - May discover this is known limitation
   - May learn about workarounds
   - May get guidance on where to look in code

**Expected outcome:**
- Validation of our findings
- Potentially avoid filing duplicate issue
- Better understanding before formal issue

---

## Decision Tree

```
Start: Read Zed source code (Phase 1)
  ↓
  ├─ Code is clear → Hypothesis confirmed/refuted → File issue with code references
  │
  └─ Code is unclear → Build with debug logging (Phase 2)
      ↓
      ├─ Logging shows clear cause → File issue with logs
      │
      └─ Still unclear → Create minimal test case (Phase 3)
          ↓
          └─ Minimal case reproduces issue → File issue with test case
```

## Recommended Starting Point

**Start with Phase 1 (Code Investigation)** because:

1. **Fastest** - No building required, just read code
2. **Definitive** - We'll see exactly what the code does
3. **Informs next steps** - Tells us if we need Phase 2/3

## Next Steps After Verification

Once hypothesis is verified:

1. **Update `zed-modification-analysis.md`** with confirmed findings
2. **File Zed issue** with:
   - Confirmed root cause (not hypothesis)
   - Evidence from code/logs/test cases
   - Proposed solution backed by understanding of actual code
3. **Optionally:** Prepare PR with fix if Zed team is receptive

## Related Documentation

- [zed-modification-analysis.md](./zed-modification-analysis.md) - Current hypothesis and proposed solutions
- [extension-research.md](./extension-research.md) - Testing that confirmed limitation exists
- [alternative-approaches.md](./alternative-approaches.md) - Other approaches considered
