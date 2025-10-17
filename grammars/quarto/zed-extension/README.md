# Zed Extension for tree-sitter-quarto

This directory contains a Zed editor extension for the tree-sitter-quarto parser.

## Installation

1. **Build the WASM parser** (if not already done):
   ```bash
   npx tree-sitter build --wasm
   ```

2. **Install as dev extension in Zed**:
   - Open Zed
   - Open the command palette (Cmd+Shift+P)
   - Type "zed: install dev extension"
   - Select this directory: `/Users/ck432/Partners HealthCare Dropbox/Chris Kennedy/Code/tree-sitter-quarto/zed-extension`

3. **Open a .qmd file** to test the parser

## Testing Inline Attributes

Once installed, open a .qmd file and test these patterns:

```markdown
[simple text]{.class}
[highlighted text]{#important .alert}
[styled span]{style="color: red;"}
```

The attributes should be highlighted and parsed correctly according to the tree-sitter AST.

## Rebuilding

After making changes to `grammar.js`:

1. Regenerate the parser:
   ```bash
   npx tree-sitter generate
   ```

2. Rebuild WASM:
   ```bash
   npx tree-sitter build --wasm
   ```

3. Copy the updated WASM file:
   ```bash
   cp tree-sitter-quarto.wasm zed-extension/languages/quarto/
   ```

4. Reload Zed (Cmd+Q and reopen) to pick up the changes

## Files

- `extension.toml` - Extension metadata
- `languages/quarto/config.toml` - Language configuration
- `languages/quarto/tree-sitter-quarto.wasm` - Compiled parser

## Known Issues

- **Link text parsing**: Link text content shows ERROR nodes with `reference_label` - this is a pre-existing issue from the base grammar
- **Inline attributes work**: Despite the link text issue, the attributes themselves parse correctly

## Current Status

✅ **87/87 tests passing** (100%)
✅ **Inline attributes implemented** via link rule extension
⚠️ **Link text content** has pre-existing ERROR nodes (doesn't affect attributes)
