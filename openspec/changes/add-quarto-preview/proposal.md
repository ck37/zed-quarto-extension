# Proposal: Add Quarto Preview Command

## Change ID
`add-quarto-preview`

## Summary
Add a slash command `/quarto-preview` that allows users to preview rendered Quarto documents in their default browser, similar to the markdown preview functionality in Zed.

## Motivation
Users working with Quarto (`.qmd`) files need a way to preview the rendered output of their documents without leaving Zed or manually running `quarto preview` in a terminal. The markdown extension in Zed provides an in-editor preview via `markdown::OpenPreview`, and Quarto users would benefit from similar functionality.

### User Need
- **Who**: Data scientists, researchers, and technical writers using Quarto documents
- **What**: Quick preview of rendered `.qmd` files to see final output
- **Why**: Streamline the write-render-review cycle without context switching
- **Current Workaround**: Manually run `quarto preview file.qmd` in terminal, which requires switching contexts and managing terminal windows

## Background

### Current State
The extension provides:
- Syntax highlighting for `.qmd` files via tree-sitter-quarto grammar
- Language injection for embedded code (Python, R, Julia, etc.)
- No preview or rendering capabilities

### Related Work
- **Zed Markdown Extension**: Provides `markdown::OpenPreview` action that opens rendered markdown in a split pane within Zed
- **Quarto CLI**: Provides `quarto preview` command that renders documents and opens them in a browser with live reload
- **Zed Extension API**: Currently supports slash commands but does not expose APIs for opening custom preview panes or registering editor actions

### Technical Constraints
**Zed Extension API Limitations (v0.7.0):**
- ❌ Cannot register custom editor actions (like `quarto::OpenPreview`)
- ❌ Cannot open UI panes or windows programmatically
- ❌ Cannot integrate with Zed's preview pane system
- ✅ Can implement slash commands for the Assistant panel
- ✅ Can execute external processes (like `quarto` CLI)
- ✅ Can open URLs in default browser

**Why Not In-Editor Preview?**
The markdown preview in Zed (`crates/markdown_preview`) is implemented as a **core Zed feature**, not via the extension API. Extensions currently cannot replicate this functionality because the API doesn't expose:
- Methods to register custom actions
- Methods to open preview panes
- UI rendering capabilities beyond slash command text output

## Proposed Solution

### Implementation Approach: External Browser Preview via Slash Command
Add `/quarto-preview` slash command that:
1. Detects if `quarto` CLI is available in PATH
2. Executes `quarto preview <file> --no-browser --render html`
3. Opens the rendered HTML in the user's default browser
4. Returns status message to Assistant panel

### Why This Approach?
- **Feasible Now**: Works within current extension API constraints
- **Low Complexity**: Leverages existing Quarto CLI functionality
- **Familiar UX**: Matches how Quarto preview normally works
- **Future-Proof**: Can be replaced with in-editor preview if/when API supports it

### Alternative Considered: Wait for API Expansion
We could wait for Zed to expose extension APIs for:
- Registering custom actions
- Opening preview panes
- UI rendering

**Decision**: Implement slash command now, document future enhancement path. Users get value immediately rather than waiting for uncertain API changes.

## Impact

### User-Facing Changes
- **New**: `/quarto-preview` slash command in Assistant panel
- **New**: Automatic browser-based preview of Quarto documents
- **New**: Error messages if Quarto CLI not installed

### Developer-Facing Changes
- **Modified**: `extension.toml` - Add slash command registration
- **Modified**: `src/lib.rs` - Implement `run_slash_command` method
- **Added**: Quarto CLI detection logic
- **Added**: Process execution for `quarto preview`
- **Added**: Tests for command execution

### Documentation Changes
- **Modified**: `README.md` - Add usage instructions for preview command
- **Added**: Document Quarto CLI installation requirement
- **Added**: Document future enhancement path (in-editor preview when API supports it)

## Dependencies
- **Quarto CLI**: Required at runtime for preview functionality
  - Installation: https://quarto.org/docs/get-started/
  - Version: Any recent version (>= 1.0.0)
  - Detection: Check `quarto` in PATH
- **No new Rust dependencies**: Uses existing `zed_extension_api` capabilities

## Risks & Mitigations

### Risk: Quarto CLI Not Installed
**Mitigation**: Slash command checks for `quarto` availability and returns helpful error message with installation link if not found.

### Risk: File Not Saved
**Mitigation**: Document that users should save file before previewing. Quarto CLI will render the saved version.

### Risk: Preview Server Port Conflicts
**Mitigation**: Quarto CLI handles port selection automatically. User can configure port via Quarto project settings if needed.

## Success Criteria
- [ ] Users can execute `/quarto-preview` in Assistant panel
- [ ] Command detects Quarto CLI availability
- [ ] Command opens rendered document in browser
- [ ] Command returns clear status messages
- [ ] Command handles errors gracefully (file not found, CLI not installed, render errors)
- [ ] Documentation clearly explains usage and requirements
- [ ] Tests validate command execution logic

## Future Enhancements
When Zed Extension API supports it:
- In-editor preview pane (matching markdown preview UX)
- Live reload as file changes
- Custom action registration (`quarto::OpenPreview`)
- Toolbar button for quick preview

### Related Zed Issues
The following issues track extension API expansion for preview/UI capabilities:
- **#17325** - Custom Views in Extension API (closed as "unactionable", no timeline)
- **#21208** - Webview via Extensions (open, "significant work to be done", no near-term timeline)
- **#10043** - Extension API to render information (closed as "not planned")
- **Discussion #18880** - Generic Panel Extension Support (no near-term plans per maintainers)

**Status as of Oct 2025**: Zed team has "no near-term plans" for visual component APIs in extensions. The current API focuses on language servers, slash commands, and debuggers. In-editor preview would require significant platform-agnostic work (different webview tech per OS).

**Implication**: The slash command approach in this proposal is likely the appropriate solution for the foreseeable future.

## Open Questions
None - implementation approach is well-defined within current API constraints and aligned with Zed roadmap.
