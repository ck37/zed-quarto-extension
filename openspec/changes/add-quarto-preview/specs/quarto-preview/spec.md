# Spec: Quarto Preview

## Overview
Capability to preview rendered Quarto documents in a browser via slash command in the Zed Assistant panel.

## ADDED Requirements

### Requirement: Slash Command Registration
The extension SHALL register a `/quarto-preview` slash command that users can invoke from the Assistant panel.

**Rationale**: Slash commands are the only mechanism available in the Zed Extension API for adding custom user-invocable functionality.

#### Scenario: User discovers preview command
**Given** the extension is installed and active  
**When** the user types `/` in the Assistant panel  
**Then** the command `/quarto-preview` appears in the autocomplete list  
**And** the description reads "Preview the current Quarto document in browser"

#### Scenario: Command invoked without file context
**Given** no Quarto file is open or in context  
**When** the user executes `/quarto-preview`  
**Then** the command returns an error message "No file to preview"  
**And** the error includes guidance on how to provide file context

---

### Requirement: Quarto CLI Detection
The extension SHALL detect whether the Quarto CLI is available on the system before attempting to preview.

**Rationale**: Preview functionality requires the Quarto CLI. Graceful failure with helpful error message improves user experience.

#### Scenario: Quarto CLI is available
**Given** `quarto` is in the system PATH  
**When** the extension checks for Quarto availability  
**Then** the check succeeds  
**And** preview command execution proceeds

#### Scenario: Quarto CLI is not available
**Given** `quarto` is NOT in the system PATH  
**When** the user executes `/quarto-preview`  
**Then** the command returns error "Quarto CLI not found. Install from https://quarto.org/docs/get-started/"  
**And** the command does not attempt to execute `quarto`

---

### Requirement: File Type Validation
The extension SHALL validate that the target file has a `.qmd` extension before attempting preview.

**Rationale**: Quarto preview only works with Quarto markdown files. Attempting to preview other file types would fail or produce unexpected results.

#### Scenario: Valid Quarto file
**Given** the current file is "document.qmd"  
**When** the user executes `/quarto-preview`  
**Then** the extension proceeds with preview execution

#### Scenario: Invalid file type
**Given** the current file is "document.md" (not .qmd)  
**When** the user executes `/quarto-preview`  
**Then** the command returns error "Preview only works with .qmd files"  
**And** the command does not execute `quarto`

---

### Requirement: File Existence Validation
The extension SHALL verify that the target file exists on the file system before attempting preview.

**Rationale**: Prevents cryptic errors from Quarto CLI when file doesn't exist.

#### Scenario: File exists
**Given** the file "document.qmd" exists on disk  
**When** the user executes `/quarto-preview`  
**Then** the extension proceeds with preview execution

#### Scenario: File does not exist
**Given** the file "document.qmd" does NOT exist on disk  
**When** the user executes `/quarto-preview`  
**Then** the command returns error "File not found: document.qmd"  
**And** the command does not execute `quarto`

---

### Requirement: Quarto Preview Execution
The extension SHALL execute `quarto preview <file>` when all validations pass.

**Rationale**: This leverages the Quarto CLI's built-in preview functionality, which handles rendering, server startup, and browser opening.

#### Scenario: Successful preview launch
**Given** the file "document.qmd" exists and is valid  
**And** Quarto CLI is available  
**When** the user executes `/quarto-preview`  
**Then** the extension executes `quarto preview document.qmd`  
**And** the Quarto preview server starts  
**And** the rendered document opens in the default browser  
**And** the command returns success message "✓ Opening preview for document.qmd"

#### Scenario: Quarto render fails
**Given** the file "document.qmd" contains syntax errors  
**When** the user executes `/quarto-preview`  
**And** Quarto CLI fails with exit code 1  
**Then** the command returns error "Quarto preview failed: <stderr output>"  
**And** the browser does not open

---

### Requirement: Success Feedback
The extension SHALL return a success message when preview is launched successfully.

**Rationale**: Users need confirmation that the command worked, especially since the browser may take a moment to open.

#### Scenario: Success message format
**Given** preview launched successfully for "analysis.qmd"  
**When** the command completes  
**Then** the Assistant panel displays "✓ Opening preview for analysis.qmd"  
**And** the message is highlighted with label "Quarto Preview"

---

### Requirement: Error Feedback
The extension SHALL return clear, actionable error messages for all failure scenarios.

**Rationale**: Users need to understand what went wrong and how to fix it.

#### Scenario: Error messages are actionable
**Given** any error condition (CLI missing, wrong file type, file not found, render error)  
**When** the command fails  
**Then** the error message clearly states what went wrong  
**And** the error message includes actionable next steps (install CLI, check file type, verify file exists, fix syntax errors)

---

### Requirement: Documentation
The extension SHALL document the preview command, its requirements, and usage in the README.

**Rationale**: Users need to know the feature exists and how to use it.

#### Scenario: README documents preview command
**Given** the extension README  
**Then** it includes a section on "Preview Command"  
**And** it explains the `/quarto-preview` slash command  
**And** it lists the Quarto CLI as a requirement  
**And** it links to Quarto installation instructions  
**And** it provides example usage

---

### Requirement: Future Enhancement Path
The extension documentation SHALL note that in-editor preview is a future enhancement pending Zed Extension API improvements.

**Rationale**: Sets user expectations and provides context for the current design choice.

#### Scenario: Documentation mentions future enhancements
**Given** the extension README or documentation  
**Then** it includes a "Future Enhancements" or "Roadmap" section  
**And** it mentions in-editor preview as a planned feature  
**And** it explains the current API limitation  
**And** it notes that the implementation will be updated when the API supports it
