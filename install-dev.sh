#!/bin/bash
set -e

echo "🧹 Cleaning build artifacts..."
cargo clean
rm -rf grammars/

echo "🧹 Cleaning Zed extension caches..."
# Remove symlink if it exists
if [ -L ~/Library/Application\ Support/Zed/extensions/installed/quarto ]; then
    rm ~/Library/Application\ Support/Zed/extensions/installed/quarto
    echo "  ✓ Removed symlink"
fi

# Remove work directory if it exists
if [ -d ~/Library/Application\ Support/Zed/extensions/work/quarto ]; then
    rm -rf ~/Library/Application\ Support/Zed/extensions/work/quarto
    echo "  ✓ Removed work directory"
fi

echo "🔨 Building extension..."
cargo build --release

echo "🧹 Cleaning grammars directory (Zed will rebuild it)..."
rm -rf grammars/

echo "✨ Ready to install in Zed!"
echo ""
echo "Next steps:"
echo "  1. Restart Zed (to clear cached extension data)"
echo "  2. Cmd+Shift+P → 'zed: install dev extension'"
echo "  3. Select this directory"
