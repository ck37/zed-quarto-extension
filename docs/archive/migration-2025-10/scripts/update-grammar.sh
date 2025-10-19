#!/bin/bash
# Update vendored tree-sitter-quarto grammar and apply Zed-compatibility patches
#
# Usage: ./scripts/update-grammar.sh [COMMIT_HASH]
#
# If COMMIT_HASH is not provided, uses the latest commit from main branch.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
GRAMMAR_DIR="$REPO_ROOT/grammars/quarto"
GRAMMAR_REPO="https://github.com/ck37/tree-sitter-quarto"
COMMIT="${1:-main}"

echo "Updating tree-sitter-quarto grammar..."
echo "Repository: $GRAMMAR_REPO"
echo "Target commit/branch: $COMMIT"
echo ""

# Remove old vendored grammar
if [ -d "$GRAMMAR_DIR" ]; then
    echo "Removing old grammar at $GRAMMAR_DIR"
    rm -rf "$GRAMMAR_DIR"
fi

# Clone grammar at specific commit
echo "Cloning grammar..."
git clone "$GRAMMAR_REPO" "$GRAMMAR_DIR"
cd "$GRAMMAR_DIR"

if [ "$COMMIT" != "main" ]; then
    echo "Checking out commit $COMMIT..."
    git checkout "$COMMIT"
fi

# Get the actual commit hash for documentation
ACTUAL_COMMIT=$(git rev-parse HEAD)
echo "Using commit: $ACTUAL_COMMIT"

# Remove git metadata
echo "Removing git metadata..."
rm -rf .git .github

# Apply Zed-compatibility patches
echo ""
echo "Applying Zed-compatibility patches to highlights.scm..."

cd "$REPO_ROOT"

# Patch highlights.scm to use Zed-compatible scopes
sed -i '' \
  -e 's/@markup\.heading/@text.title/g' \
  -e 's/@markup\.italic/@text.emphasis/g' \
  -e 's/@markup\.bold/@emphasis.strong/g' \
  -e 's/@markup\.raw\.inline/@text.literal/g' \
  -e 's/@markup\.raw\.block/@text.literal/g' \
  -e 's/@markup\.link\.text/@text.reference/g' \
  -e 's/@markup\.link\.url/@text.uri/g' \
  -e 's/@markup\.quote/@comment/g' \
  -e 's/@markup\.math\.inline/@string/g' \
  -e 's/@markup\.math\.block/@string/g' \
  "$GRAMMAR_DIR/queries/highlights.scm"

# Add patch documentation header
cat > /tmp/highlights_header.scm << 'EOF'
; Syntax highlighting queries for tree-sitter-quarto
; Based on openspec/specs/language-injection/spec.md
;
; PATCHED FOR ZED COMPATIBILITY
; This file has been modified from the upstream tree-sitter-quarto grammar to use
; Zed-compatible scope names. The following substitutions have been made:
;   @markup.heading -> @text.title
;   @markup.italic -> @text.emphasis
;   @markup.bold -> @emphasis.strong
;   @markup.raw.* -> @text.literal
;   @markup.link.text -> @text.reference
;   @markup.link.url -> @text.uri
;   @markup.quote -> @comment
;   @markup.math.* -> @string
;
; Original source: https://github.com/ck37/tree-sitter-quarto
; Commit: COMMIT_PLACEHOLDER

EOF

# Replace COMMIT_PLACEHOLDER with actual commit
sed -i '' "s/COMMIT_PLACEHOLDER/$ACTUAL_COMMIT/" /tmp/highlights_header.scm

# Replace first two lines with our header
tail -n +3 "$GRAMMAR_DIR/queries/highlights.scm" > /tmp/highlights_body.scm
cat /tmp/highlights_header.scm /tmp/highlights_body.scm > "$GRAMMAR_DIR/queries/highlights.scm"
rm /tmp/highlights_header.scm /tmp/highlights_body.scm

echo "  ✓ Applied scope mappings"
echo "  ✓ Added patch documentation header"

# Remove unsupported language injections
echo ""
echo "Removing unsupported language injections (mermaid, dot)..."

# Create temp file with mermaid and dot sections removed
awk '
/; Mermaid Diagrams/,/^$/ { next }
/; Dot\/Graphviz/,/^$/ { next }
/\(\(executable_code_cell/,/\)\)/ {
    if (/mermaid/ || /dot/) {
        skip = 1
    }
    if (!skip) print
    if (/\)\)/) skip = 0
    next
}
{ print }
' "$GRAMMAR_DIR/queries/injections.scm" > /tmp/injections_filtered.scm

mv /tmp/injections_filtered.scm "$GRAMMAR_DIR/queries/injections.scm"

echo "  ✓ Removed mermaid and dot injections"

# Copy extension-specific query files if they don't exist in grammar
echo ""
echo "Adding extension-specific query files..."

if [ ! -f "$GRAMMAR_DIR/queries/outline.scm" ] && [ -f "$REPO_ROOT/languages/quarto/outline.scm" ]; then
    cp "$REPO_ROOT/languages/quarto/outline.scm" "$GRAMMAR_DIR/queries/"
    echo "  ✓ Added outline.scm"
fi

if [ ! -f "$GRAMMAR_DIR/queries/tags.scm" ] && [ -f "$REPO_ROOT/languages/quarto/tags.scm" ]; then
    cp "$REPO_ROOT/languages/quarto/tags.scm" "$GRAMMAR_DIR/queries/"
    echo "  ✓ Added tags.scm"
fi

echo ""
echo "Grammar update complete!"
echo ""
echo "Next steps:"
echo "  1. Run tests: cargo test --workspace --all-features"
echo "  2. Review changes: git diff grammars/quarto"
echo "  3. Commit changes if tests pass"
echo ""
echo "Note: The grammar is now vendored with Zed-compatible patches applied."
echo "      Future updates should use this script to maintain compatibility."
