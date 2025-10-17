#!/bin/bash
# Workaround: Copy extension queries over grammar queries
# This is necessary because Zed loads grammar's built-in queries (with @markup.* scopes)
# instead of extension's queries (with @text.* scopes).
# See docs/highlighting-failure-analysis.md for details.

set -e

ZED_WORK_DIR="$HOME/Library/Application Support/Zed/extensions/work/quarto"

if [ ! -d "$ZED_WORK_DIR" ]; then
  echo "❌ Zed work directory not found: $ZED_WORK_DIR"
  echo "   Make sure you've installed the dev extension first:"
  echo "   Cmd+Shift+P → 'zed: install dev extension'"
  exit 1
fi

GRAMMAR_QUERIES="$ZED_WORK_DIR/grammars/quarto/queries"

if [ ! -d "$GRAMMAR_QUERIES" ]; then
  echo "❌ Grammar queries directory not found: $GRAMMAR_QUERIES"
  echo "   Zed may not have built the grammar yet."
  echo "   Try opening a .qmd file first, then run this script again."
  exit 1
fi

echo "📁 Found Zed work directory: $ZED_WORK_DIR"
echo "📝 Copying extension queries to grammar directory..."

# Copy our Zed-compatible queries over the grammar's standard queries
cp "languages/quarto/highlights.scm" "$GRAMMAR_QUERIES/highlights.scm"
cp "languages/quarto/injections.scm" "$GRAMMAR_QUERIES/injections.scm"
cp "languages/quarto/indents.scm" "$GRAMMAR_QUERIES/indents.scm"

echo "✅ Queries copied successfully!"
echo ""
echo "Next steps:"
echo "  1. Restart Zed (to reload queries)"
echo "  2. Open a .qmd file"
echo "  3. Verify syntax highlighting works"
echo ""
echo "Note: You'll need to rerun this script after:"
echo "  - Reinstalling the extension"
echo "  - Zed rebuilding the grammar"
echo "  - Zed updates that clear caches"
