#!/bin/bash
# Temporary workaround for Zed loading grammar queries instead of extension queries
# This script copies our Zed-compatible highlights.scm over the grammar's version
#
# Run this after installing the extension in Zed if highlighting is broken

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXTENSION_QUERIES="$SCRIPT_DIR/languages/quarto/highlights.scm"
GRAMMAR_QUERIES="$SCRIPT_DIR/grammars/quarto/queries/highlights.scm"

if [ ! -f "$EXTENSION_QUERIES" ]; then
    echo "Error: Extension queries not found at $EXTENSION_QUERIES"
    exit 1
fi

if [ ! -d "$SCRIPT_DIR/grammars/quarto/queries" ]; then
    echo "Error: Grammar not installed. Please install the extension in Zed first."
    exit 1
fi

echo "Copying Zed-compatible highlights.scm to grammar directory..."
cp "$EXTENSION_QUERIES" "$GRAMMAR_QUERIES"

echo "Done! Please restart Zed or reload the window for changes to take effect."
echo ""
echo "Note: This is a temporary workaround. If you reinstall the extension,"
echo "you'll need to run this script again."
