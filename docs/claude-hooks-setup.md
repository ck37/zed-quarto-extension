# Claude Code Hooks Setup

This document explains how to configure Claude Code hooks to enforce repository policies.

## Project-Specific Configuration

This project uses **project-level hooks** configured in `.claude/settings.json`. These hooks are version-controlled and apply automatically when working in this repository.

### Configuration Files

- **`.claude/settings.json`**: Project-level hook configuration (version controlled, shared with all contributors)
- **`.claude/hooks/validate-gh-command.py`**: Validation script (version controlled)
- **`.claude/settings.local.json`**: Machine-specific overrides (gitignored, optional)

## Blocking Unauthorized Issue Creation

The project enforces a PreToolUse hook that prevents Claude from creating issues in `zed-industries/extensions` without explicit approval.

### Current Setup

The hook is already configured in this project. The files are:

**`.claude/settings.json`**:
```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/validate-gh-command.py"
          }
        ]
      },
      {
        "matcher": "mcp__acp__Bash",
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/validate-gh-command.py"
          }
        ]
      }
    ]
  }
}
```

**`.claude/hooks/validate-gh-command.py`** (see file for full content):
- Blocks `gh issue create` commands to `zed-industries/extensions` and `zed-industries/zed`
- Allows issue creation in `ck37/zed-quarto-extension`, `ck37/tree-sitter-quarto`, and other repos
- Provides clear error messages explaining why the action was blocked

### No Setup Required

The hooks are already configured and version-controlled in this repository. They will automatically apply when you work in this project.

### Testing the Hook

Try creating an issue in a restricted repo:

```bash
gh issue create --repo zed-industries/extensions --title "Test" --body "Test"
```

The hook should block it with a clear error message.

## How It Works

1. **PreToolUse Event**: Runs before any Bash tool is executed
2. **Command Inspection**: Script checks if the command is `gh issue create` for a restricted repo
3. **Permission Decision**: Returns `"deny"` to block the action
4. **Feedback to Claude**: The `permissionDecisionReason` explains why it was blocked
5. **Claude's Response**: Claude sees the block and can request explicit permission or use the correct repo

## Adding More Restrictions

You can extend the script to block other operations:

```python
# Block pushes to protected branches
if "git push" in command and "origin main" in command:
    # Block and require approval
    
# Block dangerous operations
if "rm -rf" in command or "sudo" in command:
    # Block and warn
```

## Allowed Repositories

The hook allows issue creation in:
- `ck37/zed-quarto-extension` (extension-specific issues)
- `ck37/tree-sitter-quarto` (grammar issues)
- Any other repositories not in the restricted list

## References

- **Claude Code Hooks Documentation**: https://docs.claude.com/en/docs/claude-code/hooks
- **Repository Guidelines**: See `CLAUDE.md` in this repo
