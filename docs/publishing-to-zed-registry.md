# Publishing to Zed Extension Registry

This guide explains how to publish the Quarto extension to the official Zed extensions registry so it appears under "Languages" in Zed's extension list.

## Prerequisites

✅ **Already Complete:**
- [x] Extension has `extension.toml` with metadata
- [x] Extension is in a public GitHub repository
- [x] Extension has an accepted open source license (MIT)
- [x] Extension builds successfully as WASM
- [x] All tests pass

⚠️ **To Check:**
- [ ] LICENSE file is at repository root (not in a subdirectory)
- [ ] Extension ID doesn't contain "zed" in the name (✅ ours is "quarto")

## Publishing Process

### 1. Verify License File Location

Check that `LICENSE` is at the repository root:

```bash
ls -la LICENSE
```

If it's not there, move or create it at the root.

### 2. Fork the Extensions Repository

1. Go to https://github.com/zed-industries/extensions
2. Click "Fork" to create your own fork
3. Clone your fork locally:

```bash
cd ~/Code  # or wherever you want to work
git clone https://github.com/YOUR-USERNAME/extensions.git
cd extensions
git submodule update --init --recursive
```

### 3. Add Your Extension as a Submodule

⚠️ **Important:** Use HTTPS URL, not SSH!

```bash
cd extensions  # the directory within the repo
git submodule add https://github.com/ck37/zed-quarto-extension.git quarto
```

This will:
- Create `extensions/quarto/` directory as a git submodule
- Point to your extension repository
- Use the current commit from main branch

### 4. Update extensions.toml

Install pnpm if needed:
```bash
npm install -g pnpm
```

Edit `extensions.toml` and add your extension entry:

```toml
[quarto]
submodule = "extensions/quarto"
version = "0.2.0"
```

**Note:** The version must match the `version` field in your `extension.toml`

### 5. Sort the Configuration

Run the sorting script to ensure proper formatting:

```bash
pnpm install  # if first time
pnpm sort-extensions
```

### 6. Commit and Push to Your Fork

```bash
git add .gitmodules extensions/quarto extensions.toml
git commit -m "Add Quarto language extension

Adds support for Quarto (.qmd) documents with:
- Syntax highlighting via tree-sitter-quarto
- Support for Pandoc Markdown extensions
- Code chunk highlighting (Python, R, Julia, SQL, etc.)
- YAML front matter injection
- Citations, cross-references, shortcodes
- Fenced divs and callouts
"

git push origin main
```

### 7. Open a Pull Request

1. Go to your fork on GitHub
2. Click "Contribute" → "Open pull request"
3. Write a clear PR description:

```markdown
# Add Quarto Language Extension

This PR adds support for Quarto (`.qmd`) authoring in Zed.

## What is Quarto?

Quarto is an open-source scientific and technical publishing system built on Pandoc that enables authoring documents, presentations, websites, and books using Markdown with embedded code execution (Python, R, Julia, etc.).

## Extension Features

- **Syntax highlighting** via tree-sitter-quarto grammar
- **Quarto-specific features**: executable code cells, chunk options, inline code cells, cross-references
- **Pandoc Markdown extensions**: fenced divs, attribute lists, citations, shortcodes, subscript, superscript, strikethrough, etc.
- **Language injections**: Python, R, Julia, SQL, JavaScript, TypeScript, Bash, YAML, and more
- **Math support**: inline and display math with LaTeX syntax
- **Tables**: pipe tables with proper cell parsing

## Testing

- Repository: https://github.com/ck37/zed-quarto-extension
- 73 automated tests passing
- Grammar repository: https://github.com/ck37/tree-sitter-quarto (203 tests passing)
- Builds successfully to WASM (211KB)

## License

MIT License (compatible with Zed)

## Links

- Extension repo: https://github.com/ck37/zed-quarto-extension
- Grammar repo: https://github.com/ck37/tree-sitter-quarto
- Quarto website: https://quarto.org
```

4. Submit the PR

### 8. Wait for Review

The Zed team will review your PR. They may ask for changes or improvements. Common feedback:
- Code quality issues
- Missing documentation
- License compatibility
- Extension naming conventions

### 9. After Merge

Once merged, the extension will be:
- ✅ Automatically packaged and published to the Zed extension registry
- ✅ Available under "Languages" in Zed's extension browser
- ✅ Installable by all Zed users

## Updating Your Extension

After the initial publish, to release updates:

1. Make changes in your extension repository
2. Commit and push to your extension repo
3. Update the submodule in the extensions repo:
   ```bash
   cd extensions/quarto
   git pull origin main
   cd ../..
   git add extensions/quarto
   git commit -m "Update Quarto extension to vX.Y.Z"
   git push
   ```
4. Open a new PR to update the version

## Current Status

- [ ] License at repository root verified
- [ ] Extensions repository forked
- [ ] Extension added as submodule
- [ ] extensions.toml updated
- [ ] PR opened
- [ ] PR merged
- [ ] Extension live in registry

## Resources

- [Zed Extensions Documentation](https://zed.dev/docs/extensions/developing-extensions)
- [Extensions Repository](https://github.com/zed-industries/extensions)
- [Extension API Docs](https://github.com/zed-industries/zed/blob/main/docs/src/extensions.md)

## Notes

- **Extension ID:** "quarto" (no "zed" in the name ✅)
- **Current Version:** 0.2.0
- **Repository:** https://github.com/ck37/zed-quarto-extension
- **License:** MIT
