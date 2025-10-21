#!/usr/bin/env python3
"""
Claude Code hook to prevent unauthorized GitHub issue creation.

This hook blocks attempts to create issues in restricted repositories
without explicit user approval.
"""
import json
import sys

# Read hook input from stdin
input_data = json.load(sys.stdin)
command = input_data.get("tool_input", {}).get("command", "")

# Repositories that require explicit approval for issue creation
RESTRICTED_REPOS = [
    "zed-industries/extensions",
    "zed-industries/zed"
]

# Check if this is a gh issue create command
if "gh issue create" in command:
    for repo in RESTRICTED_REPOS:
        # Check both --repo and -R flags
        if f"--repo {repo}" in command or f"-R {repo}" in command:
            output = {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": (
                        f"❌ Creating issues in '{repo}' requires explicit user approval.\n\n"
                        f"This repository is restricted per CLAUDE.md guidelines. Please:\n"
                        f"1. Ask the user for permission to create an issue in {repo}\n"
                        f"2. Use the correct repository instead:\n"
                        f"   - ck37/zed-quarto-extension (extension-specific issues)\n"
                        f"   - ck37/tree-sitter-quarto (grammar issues)\n\n"
                        f"See CLAUDE.md for complete repository guidelines."
                    )
                }
            }
            print(json.dumps(output))
            sys.exit(0)

# Allow all other commands
sys.exit(0)
