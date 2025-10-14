# Contributing to tree-sitter-quarto

Thank you for your interest in contributing! This guide will help you get started.

## Project Status

**Current Phase:** Planning - Not yet functional

We welcome contributions once the foundation is implemented. For now, feedback on the design and planning documents is appreciated.

## Getting Started

### Prerequisites

- Node.js (v16 or later)
- npm or yarn
- C compiler (for external scanner)
- Git

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/ck37/tree-sitter-quarto.git
   cd tree-sitter-quarto
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Generate the parser:
   ```bash
   npx tree-sitter generate
   ```

4. Run tests:
   ```bash
   npx tree-sitter test
   ```

## Development Workflow

### Making Changes

1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes to:
   - `grammar.js` - Grammar rules
   - `src/scanner.c` - External scanner (if needed)
   - `queries/*.scm` - Syntax highlighting queries
   - `test/corpus/*.txt` - Test cases

3. Generate and test:
   ```bash
   npx tree-sitter generate
   npx tree-sitter test
   ```

4. Parse example files:
   ```bash
   npx tree-sitter parse examples/sample.qmd
   ```

### Writing Tests

Tests follow the tree-sitter corpus format in `test/corpus/*.txt`:

```
==================
Test name
==================

Input markdown here

---

(expected_ast_structure)
```

**Guidelines:**
- One feature per test case
- Include edge cases
- Test feature interactions
- Add descriptive test names

### Code Style

**Grammar Rules:**
- Use descriptive rule names
- Add comments for complex rules
- Group related rules together
- Use `field()` for important nodes

**Scanner Code:**
- Follow existing C style
- Add comments explaining logic
- Handle edge cases
- Test boundary conditions

**Query Files:**
- Use consistent indentation (2 spaces)
- Group related patterns
- Add comments for non-obvious patterns
- Test with real documents

## File Naming Convention

**Use lowercase filenames throughout:**
- ✅ `plan.md`, `todo.md`, `readme.md`
- ❌ `PLAN.md`, `TODO.md`, `README.md`

Exception: `README.md`, `CONTRIBUTING.md`, `LICENSE` (conventional names)

## Project Structure

```
tree-sitter-quarto/
├── grammar.js              # Main grammar file
├── src/
│   └── scanner.c           # External scanner
├── queries/
│   ├── highlights.scm      # Syntax highlighting
│   ├── injections.scm      # Language injection
│   ├── folds.scm           # Code folding
│   └── locals.scm          # Local variables
├── test/
│   └── corpus/             # Test cases
│       ├── executable-cells.txt
│       ├── chunk-options.txt
│       └── ...
├── examples/
│   └── sample.qmd          # Example document
└── docs/                   # Documentation
    ├── plan.md
    ├── todo.md
    └── ...
```

## Architecture Overview

### Base Grammar

This parser extends tree-sitter-pandoc-markdown using the "Copy & Extend" strategy:
- Copy `grammar.js` from tree-sitter-pandoc-markdown
- Extend with Quarto-specific rules
- Document source commit hash

### Key Features

1. **Executable Code Cells** - Parse `{python}`, `{r}`, `{julia}` cells
2. **Chunk Options** - Parse `#| key: value` syntax
3. **Cross-References** - Distinguish `@fig-plot` from `@citation`
4. **Inline Code Cells** - Parse `` `{python} expr` ``
5. **Shortcodes** - Parse `{{< video url >}}`

See [docs/plan.md](./docs/plan.md) for detailed architecture.

## External Scanner

The external scanner handles context-sensitive parsing:

**Required tokens:**
- `CHUNK_OPTION_MARKER` - Detect `#|` at cell start
- `CELL_BOUNDARY` - Context-aware cell delimiters

**Why needed:**
- Distinguish `#| option` from `# comment`
- Requires looking at cell context
- Beyond LR(1) parser capability

## Testing Guidelines

### Test Categories

1. **Unit Tests** - Individual features
2. **Edge Cases** - Boundary conditions
3. **Integration Tests** - Feature combinations
4. **Real-World Tests** - Parse actual Quarto documents

### Running Tests

```bash
# Run all tests
npx tree-sitter test

# Run specific test file
npx tree-sitter test -f executable-cells

# Debug test failures
npx tree-sitter test --debug
```

### Adding Test Cases

1. Add test to appropriate `test/corpus/*.txt` file
2. Run `npx tree-sitter test`
3. If needed, update expected AST
4. Verify parse tree with `npx tree-sitter parse`

## Documentation

### When to Update Docs

- Adding new features → Update `docs/plan.md` and `README.md`
- Changing architecture → Update `docs/plan.md`
- Adding tests → Update `docs/todo.md` checklist
- Making decisions → Update `docs/todo.md` Decision Log

### Documentation Files

- `README.md` - Project overview and quick start
- `docs/plan.md` - Implementation plan and architecture
- `docs/todo.md` - Task checklist and decisions
- `docs/reference-documentation.md` - Technical references
- `CONTRIBUTING.md` - This file

## Pull Request Process

1. **Before submitting:**
   - Run all tests (`npx tree-sitter test`)
   - Update documentation
   - Add test cases for new features
   - Follow code style guidelines

2. **PR description should include:**
   - What: Brief description of changes
   - Why: Rationale for changes
   - Testing: How you tested the changes
   - Docs: What documentation was updated

3. **Review process:**
   - Maintainers will review your PR
   - Address feedback
   - Ensure CI passes
   - Squash commits if requested

## Commit Messages

**Format:**
```
type: brief description

Detailed explanation if needed.

Fixes #issue-number
```

**Types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code restructuring
- `chore:` - Maintenance tasks

**Examples:**
- `feat: add shortcode parsing support`
- `fix: correct chunk option detection in scanner`
- `docs: update architecture decisions in plan.md`
- `test: add edge cases for nested cells`

## Questions or Issues?

- **Bug reports:** Open a GitHub issue with reproduction steps
- **Feature requests:** Open an issue describing the use case
- **Questions:** Start a GitHub discussion
- **Security issues:** Email maintainers directly

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Assume good intentions

## Related Projects

- [tree-sitter-pandoc-markdown](https://github.com/ck37/tree-sitter-pandoc-markdown) - Base grammar
- [Quarto](https://quarto.org/) - Publishing system
- [tree-sitter](https://tree-sitter.github.io/) - Parser framework

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to tree-sitter-quarto!**
