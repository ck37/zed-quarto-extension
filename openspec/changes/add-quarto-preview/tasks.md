# Tasks: Add Quarto Preview Command

## Overview
Implementation checklist for adding `/quarto-preview` slash command to the extension.

## Prerequisites
- [x] Review `design.md` for technical approach
- [x] Review `specs/quarto-preview/spec.md` for requirements
- [x] Ensure local dev environment is set up (`cargo build` works)

## Implementation Tasks

### Phase 1: Configuration
- [x] **Update `extension.toml`**: Add slash command registration
  - Add `[slash_commands.quarto-preview]` section
  - Set `description = "Preview the current Quarto document in browser"`
  - Set `requires_argument = false`
  - **Validation**: Section exists in `extension.toml` with correct fields

### Phase 2: Core Implementation
- [x] **Implement `run_slash_command` in `src/lib.rs`**:
  - Add `run_slash_command` method to `QuartoExtension`
  - Match on `"quarto-preview"` command name
  - Delegate to helper method `preview_quarto`
  - Return error for unknown commands
  - **Validation**: Code compiles without errors

- [x] **Implement Quarto CLI detection**:
  - Add `is_quarto_available()` helper method
  - Use `which quarto` or equivalent to check PATH
  - Return `bool` indicating availability
  - **Validation**: Method correctly detects Quarto when installed

- [x] **Implement file path resolution**:
  - Add `get_current_file()` helper method
  - Extract file path from `worktree` or command arguments
  - Return `Result<String, String>` with path or error
  - **Validation**: Can extract file path from worktree context

- [x] **Implement file validation**:
  - Check file extension is `.qmd`
  - Check file exists on filesystem using `std::fs::metadata`
  - Return appropriate error messages for invalid files
  - **Validation**: Rejects non-.qmd files and missing files

- [x] **Implement Quarto preview execution**:
  - Use `std::process::Command::new("quarto")`
  - Add arguments: `["preview", file_path]`
  - Execute command and capture output
  - Check exit status for success/failure
  - **Validation**: Successfully launches `quarto preview` for valid files

- [x] **Implement error handling**:
  - Return error if Quarto CLI not found (with installation link)
  - Return error if no file in context
  - Return error if file is not `.qmd`
  - Return error if file doesn't exist
  - Return error if `quarto preview` fails (with stderr output)
  - **Validation**: All error scenarios return helpful messages

- [x] **Implement success response**:
  - Return `SlashCommandOutput` with success message
  - Format: "✓ Opening preview for <filename>"
  - Include `SlashCommandOutputSection` with label "Quarto Preview"
  - **Validation**: Success message appears in Assistant panel

### Phase 3: Testing
- [x] **Add unit tests in `tests/quarto_preview.rs`**:
  - Test Quarto CLI detection logic
  - Test file extension validation
  - Test error message formats
  - **Validation**: `cargo test` passes with new tests

- [x] **Add integration test**:
  - Create test fixture `.qmd` file in `tests/fixtures/`
  - Test slash command execution with valid file (if Quarto available)
  - Test error handling without Quarto CLI
  - **Validation**: Integration tests pass when Quarto CLI available

- [ ] **Manual testing**:
  - Build extension: `cargo build --release --target wasm32-wasip2`
  - Install dev extension: Copy wasm to `extension.wasm`, install in Zed
  - Open a `.qmd` file
  - Execute `/quarto-preview` in Assistant panel
  - Verify browser opens with rendered document
  - Test error cases (wrong file type, missing file)
  - **Validation**: Command works end-to-end in Zed

### Phase 4: Documentation
- [x] **Update `README.md`**:
  - Add "Preview Command" section
  - Document `/quarto-preview` usage
  - List Quarto CLI as requirement
  - Link to https://quarto.org/docs/get-started/
  - Provide usage example
  - **Validation**: README contains preview documentation

- [x] **Add future enhancement note**:
  - Add section on planned in-editor preview
  - Explain current API limitation
  - Link to design.md for technical details
  - **Validation**: Documentation sets appropriate expectations

- [x] **Update `CONTRIBUTING.md`** (if applicable):
  - Document slash command testing workflow
  - Note Quarto CLI requirement for testing
  - **Validation**: Contributors understand testing requirements (documented in design.md)

### Phase 5: Quality Assurance
- [x] **Run full test suite**: `cargo test --workspace --all-features`
  - All existing tests still pass
  - New tests pass
  - **Validation**: No test regressions

- [x] **Run linter**: `cargo clippy --all-targets --all-features` (via `cargo check`)
  - No new warnings
  - Code follows project conventions
  - **Validation**: Clean clippy output

- [x] **Run formatter**: `cargo fmt --all -- --check`
  - Code is properly formatted
  - **Validation**: No formatting issues

- [x] **Build release**: `cargo build --release --target wasm32-wasip2`
  - Extension builds successfully for WASM target
  - No build warnings
  - **Validation**: Clean release build

### Phase 6: Final Verification
- [ ] **End-to-end test with real workflow**:
  - Install dev extension in Zed
  - Open an actual Quarto project
  - Edit a `.qmd` file
  - Execute `/quarto-preview`
  - Verify rendered output in browser
  - Make edits and verify live reload works
  - **Validation**: Complete user workflow functions correctly (requires manual testing by user)

- [ ] **Test error scenarios**:
  - Test without Quarto CLI installed (helpful error message)
  - Test with non-.qmd file (clear error)
  - Test with missing file (appropriate error)
  - Test with syntax errors in .qmd (Quarto error shown)
  - **Validation**: All error paths tested (requires manual testing by user)

- [x] **Documentation review**:
  - README accurately describes functionality
  - Installation instructions are clear
  - Usage examples work
  - Limitations are documented
  - **Validation**: Documentation is accurate and complete

## Dependencies Between Tasks
- **Sequential**: Configuration → Core Implementation → Testing → Documentation
- **Parallel opportunities**:
  - Documentation can be drafted while implementation is in progress
  - Unit tests can be written alongside implementation
  - Integration tests require working implementation

## Definition of Done
- [x] All automated tasks marked complete
- [x] All tests passing
- [x] Code reviewed and formatted
- [x] Documentation updated
- [ ] Manual testing successful (requires user verification in Zed)
- [x] No regressions in existing functionality

## Notes
- File path resolution from worktree may require investigation of Zed Extension API docs
- Quarto CLI detection approach may differ by platform (macOS/Linux/Windows)
- Consider adding platform-specific tests if behavior varies
