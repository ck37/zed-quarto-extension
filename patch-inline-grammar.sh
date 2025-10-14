#!/bin/bash
# Patch the inline grammar with Zed-compatible scopes
# Run this AFTER installing the extension in Zed

set -e

GRAMMAR_HIGHLIGHTS="grammars/pandoc_markdown_inline/tree-sitter-pandoc-markdown-inline/queries/highlights.scm"
OVERRIDE_HIGHLIGHTS="languages/pandoc_markdown_inline/highlights.scm"

if [ ! -f "$OVERRIDE_HIGHLIGHTS" ]; then
    echo "❌ Override file not found: $OVERRIDE_HIGHLIGHTS"
    exit 1
fi

if [ ! -f "$GRAMMAR_HIGHLIGHTS" ]; then
    echo "❌ Grammar highlights file not found: $GRAMMAR_HIGHLIGHTS"
    echo "   Please install the extension in Zed first"
    exit 1
fi

echo "🔧 Patching inline grammar with Zed-compatible scopes..."
cp "$OVERRIDE_HIGHLIGHTS" "$GRAMMAR_HIGHLIGHTS"

# Verify the patch worked
if grep -q "@text.emphasis" "$GRAMMAR_HIGHLIGHTS"; then
    echo "✅ Successfully patched! Highlights now use @text.emphasis and @emphasis.strong"
    echo ""
    echo "Next steps:"
    echo "  1. In Zed, run: Cmd+Shift+P → 'zed: reload extensions'"
    echo "  2. Triple asterisks should now work!"
else
    echo "❌ Patch failed - grammar still has @markup.* scopes"
    exit 1
fi
