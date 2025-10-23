# Design: Quarto Preview Command

## Overview
This document outlines the technical design for implementing Quarto document preview functionality in the Zed extension.

## Problem Statement
Users need to preview rendered Quarto documents without leaving Zed or manually running CLI commands. The ideal solution would mirror Zed's markdown preview (in-editor split pane), but current extension API limitations prevent this approach.

## Design Constraints

### Zed Extension API Limitations
The `zed_extension_api` (v0.7.0) provides:
- ✅ `Extension` trait with methods for language servers, workspace configuration
- ✅ Slash command registration and execution
- ✅ Process execution (`zed::Command`)
- ✅ HTTP client for downloads
- ✅ Settings access

**NOT Available:**
- ❌ Custom action registration (e.g., `quarto::OpenPreview`)
- ❌ UI pane/window APIs
- ❌ Integration with preview system
- ❌ Custom toolbar buttons
- ❌ Editor state modification

### Quarto CLI Behavior
`quarto preview <file>` command:
- Renders document to output format (HTML by default)
- Starts local preview server
- Opens browser automatically (unless `--no-browser` flag used)
- Watches file for changes and live-reloads
- Requires saved file on disk

## Architecture Options

### Option 1: Slash Command with External Browser (SELECTED)

**Implementation:**
```rust
impl zed::Extension for QuartoExtension {
    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "quarto-preview" => {
                // 1. Detect quarto CLI
                // 2. Get current file path from args or worktree
                // 3. Execute: quarto preview <file>
                // 4. Return status message
            }
            _ => Err(format!("unknown command: {}", command.name))
        }
    }
}
```

**Pros:**
- Works within current API constraints
- Leverages existing Quarto CLI functionality
- Matches user expectations (Quarto preview normally opens in browser)
- Simple implementation (~50 lines of code)
- No new dependencies

**Cons:**
- External to Zed (opens browser, not in-editor)
- User must install Quarto CLI separately
- Different UX than markdown preview

**Decision**: This is the only viable option given current API constraints.

### Option 2: In-Editor Preview Pane (FUTURE)

**Requirements:**
- Zed Extension API must add:
  - Action registration API
  - Preview pane API
  - HTML rendering capability

**Conceptual Implementation:**
```rust
// Future API (hypothetical)
impl zed::Extension for QuartoExtension {
    fn register_actions(&mut self, registry: &mut ActionRegistry) {
        registry.register("quarto::OpenPreview", |context| {
            // Render document
            // Open preview pane
            // Set up live reload
        });
    }
}
```

**Decision**: Document as future enhancement. Note: Zed team has stated "no near-term plans" for visual component APIs in extensions (per Discussion #18880, Oct 2024). Related issues (#17325, #21208, #10043) show this is a frequently requested feature but not currently on the roadmap. In-editor preview would require significant platform-agnostic work for webviews. User should not expect this capability in extensions for the foreseeable future.

### Option 3: Slash Command with HTML Output (REJECTED)

**Approach**: Render to HTML, return HTML as text in Assistant panel

**Why Rejected:**
- HTML source code not useful to users
- Assistant panel doesn't render HTML
- Defeats purpose of preview (users want visual output)

## Selected Design: Slash Command Implementation

### Component Architecture

```
┌─────────────────────────────────────────────┐
│          Zed Editor (User)                  │
│  ┌───────────────────────────────────────┐  │
│  │   Assistant Panel                     │  │
│  │   > /quarto-preview                   │  │
│  │   ✓ Opening preview...                │  │
│  └───────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
                    ↓ (1) Invoke command
┌─────────────────────────────────────────────┐
│     QuartoExtension::run_slash_command      │
│  ┌───────────────────────────────────────┐  │
│  │ 1. Detect quarto CLI                  │  │
│  │ 2. Get current file path              │  │
│  │ 3. Validate file exists and is .qmd   │  │
│  │ 4. Execute: quarto preview <file>     │  │
│  │ 5. Return status message              │  │
│  └───────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
                    ↓ (2) Execute command
┌─────────────────────────────────────────────┐
│           Quarto CLI (External)             │
│  ┌───────────────────────────────────────┐  │
│  │ - Render .qmd → HTML                  │  │
│  │ - Start preview server                │  │
│  │ - Open browser                        │  │
│  │ - Watch file for changes              │  │
│  └───────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
                    ↓ (3) Open browser
┌─────────────────────────────────────────────┐
│          Default Browser                    │
│  ┌───────────────────────────────────────┐  │
│  │   Rendered Quarto Document            │  │
│  │   (HTML output with live reload)      │  │
│  └───────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
```

### Data Flow

**Input**: User types `/quarto-preview` in Assistant panel

**Processing**:
1. Zed calls `QuartoExtension::run_slash_command()`
2. Extension checks for `quarto` in PATH
3. Extension determines current file (from worktree or args)
4. Extension validates file is `.qmd`
5. Extension executes `quarto preview <file>`
6. Quarto CLI renders and opens browser
7. Extension returns status message

**Output**: 
- Browser opens with rendered document
- Assistant panel shows success message
- Quarto preview server runs in background

### Error Handling

| Error Condition | Detection | Response |
|----------------|-----------|----------|
| Quarto CLI not installed | `which quarto` fails | Return error with installation link |
| No file in context | Worktree is None and args empty | Return error: "No file to preview" |
| File not `.qmd` | Path extension check | Return error: "Only .qmd files supported" |
| File doesn't exist | File system check | Return error: "File not found" |
| Render fails | `quarto preview` exit code | Return error with Quarto error message |

### Implementation Details

#### 1. Slash Command Registration
**File**: `extension.toml`
```toml
[slash_commands.quarto-preview]
description = "Preview the current Quarto document in browser"
requires_argument = false
```

#### 2. Extension Implementation
**File**: `src/lib.rs`
```rust
use zed_extension_api as zed;
use std::fs;

struct QuartoExtension;

impl zed::Extension for QuartoExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        match command.name.as_str() {
            "quarto-preview" => self.preview_quarto(worktree),
            _ => Err(format!("Unknown command: {}", command.name)),
        }
    }
}

impl QuartoExtension {
    fn preview_quarto(
        &self,
        worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        // 1. Check quarto CLI availability
        if !self.is_quarto_available() {
            return Err(
                "Quarto CLI not found. Install from https://quarto.org/docs/get-started/".to_string()
            );
        }

        // 2. Get current file path
        let file_path = self.get_current_file(worktree)?;

        // 3. Validate file extension
        if !file_path.ends_with(".qmd") {
            return Err("Preview only works with .qmd files".to_string());
        }

        // 4. Validate file exists
        if !fs::metadata(&file_path).is_ok() {
            return Err(format!("File not found: {}", file_path));
        }

        // 5. Execute quarto preview
        let output = std::process::Command::new("quarto")
            .arg("preview")
            .arg(&file_path)
            .output()
            .map_err(|e| format!("Failed to run quarto: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Quarto preview failed: {}", stderr));
        }

        // 6. Return success message
        let text = format!("✓ Opening preview for {}", file_path);
        Ok(zed::SlashCommandOutput {
            text: text.clone(),
            sections: vec![zed::SlashCommandOutputSection {
                range: (0..text.len()).into(),
                label: "Quarto Preview".to_string(),
            }],
        })
    }

    fn is_quarto_available(&self) -> bool {
        std::process::Command::new("which")
            .arg("quarto")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn get_current_file(&self, worktree: Option<&zed::Worktree>) -> Result<String, String> {
        // Implementation depends on how to get current file from worktree
        // May need to be passed as argument to slash command
        worktree
            .and_then(|wt| wt.root_path())
            .ok_or_else(|| "No file in context".to_string())
    }
}
```

### Testing Strategy

**Unit Tests**: `tests/quarto_preview.rs`
- Mock Quarto CLI detection
- Test error messages
- Test file validation logic

**Integration Tests**:
- Test with real `.qmd` file (if Quarto CLI available)
- Test error handling (CLI not found, invalid file)

**Manual Testing**:
1. Install dev extension: `./install-dev.sh`
2. Open `.qmd` file in Zed
3. Open Assistant panel
4. Type `/quarto-preview`
5. Verify browser opens with rendered output

## Performance Considerations

**Command Execution Time**:
- Quarto CLI detection: ~10ms (`which quarto`)
- File validation: ~1ms (file system check)
- Quarto preview startup: ~1-3 seconds (rendering + server start)

**Async Behavior**:
- Slash command execution is blocking within extension
- Quarto preview runs as background process
- Browser opens asynchronously

**Resource Usage**:
- Extension: Negligible (just CLI invocation)
- Quarto preview server: ~50-100MB RAM
- Browser: Standard webpage memory usage

## Security Considerations

**Command Injection**:
- File paths are validated (must be `.qmd`)
- No user input directly passed to shell
- Uses `std::process::Command` (not shell execution)

**File Access**:
- Only previews files in current worktree
- Quarto CLI runs with user permissions
- No file writing from extension

## Maintenance & Evolution

**Dependency Updates**:
- No new Rust dependencies
- Relies on Quarto CLI (user-managed)
- Zed Extension API changes may require updates

**Future Enhancements**:
1. Configuration options (port, format, theme)
2. Multiple format support (PDF, DOCX, RevealJS)
3. In-editor preview when API supports it
4. Keybinding support when API supports it

## Decision Log

| Decision | Rationale | Date |
|----------|-----------|------|
| Use slash command approach | Only viable option with current API | 2025-10-22 |
| External browser preview | Matches Quarto CLI UX, simple implementation | 2025-10-22 |
| No format argument | Start simple, add later if requested | 2025-10-22 |
| Require Quarto CLI | Simpler than bundling renderer, matches user expectations | 2025-10-22 |

## References
- Zed Extension API: https://docs.rs/zed_extension_api/latest/zed_extension_api/
- Quarto CLI: https://quarto.org/docs/reference/cli.html
- Zed Slash Commands: https://zed.dev/docs/extensions/slash-commands
