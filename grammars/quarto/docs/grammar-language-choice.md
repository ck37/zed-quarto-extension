# Grammar Language Choice: JavaScript vs Rust

## Summary

tree-sitter-quarto uses **JavaScript** to define its grammar (`grammar.js`), while the official quarto-markdown project uses **Rust** (via rust-sitter). This document explains why both approaches are valid and why we chose JavaScript.

**Key Insight:** Both approaches generate the same C parser, so **runtime performance is identical**. The choice is about developer experience and ecosystem fit.

## How Tree-sitter Works

Regardless of grammar language, the compilation pipeline is:

```
Grammar Definition → Tree-sitter CLI → C Parser Code → WebAssembly/Native Binary
```

- **JavaScript approach:** `grammar.js` → C code → fast parser
- **Rust approach:** Rust annotations → C code → fast parser
- **Final result:** Same C parser, same performance

## Comparison

| Aspect | JavaScript (our choice) | Rust (rust-sitter) |
|--------|------------------------|-------------------|
| **Runtime performance** | ✅ Fast (C parser) | ✅ Fast (C parser) |
| **Compile-time checking** | ⚠️ JavaScript validation | ✅ Rust type system |
| **Developer tooling** | ✅ Excellent (standard approach) | ⚠️ Good but newer |
| **Learning curve** | ✅ Lower (more examples) | ⚠️ Steeper (fewer examples) |
| **Documentation** | ✅ Comprehensive | ⚠️ Growing |
| **Community examples** | ✅ 99% of grammars | ⚠️ Small but growing |
| **Dependencies** | ✅ Just Node.js | ⚠️ Requires Rust toolchain |
| **Debugging** | ✅ Well-documented | ⚠️ Can be harder |
| **Type safety** | ❌ JavaScript (dynamic) | ✅ Rust (static) |
| **Auto-generated bindings** | ❌ Manual if needed | ✅ Automatic to Rust types |
| **Editor integration** | ✅ Works everywhere | ✅ Works everywhere |

## Why We Chose JavaScript

### 1. Industry Standard
- 99% of existing tree-sitter grammars use JavaScript
- Examples: tree-sitter-python, tree-sitter-typescript, tree-sitter-rust (ironically!)
- tree-sitter-pandoc-markdown (our base) uses JavaScript
- Extensive examples and tutorials available

### 2. Lower Barrier to Entry
- Most developers know JavaScript
- Simpler toolchain (just Node.js and npm)
- Easier for community contributors
- Better IDE support for grammar development

### 3. Proven Ecosystem
- Battle-tested in production editors (Neovim, Zed, VSCode, Helix)
- Robust error messages and debugging tools
- Well-understood patterns and best practices
- Active community support

### 4. Project Independence
- No Rust compilation required
- Works on any platform with Node.js
- Faster CI/CD (no Rust build step)
- Easier for contributors to get started

### 5. Alignment with Base Grammar
- tree-sitter-pandoc-markdown uses JavaScript
- Copy & Extend strategy is simpler when languages match
- Easier to track changes and merge updates

## When Rust Makes Sense

The official quarto-markdown project chose Rust for good reasons:

### 1. Ecosystem Integration
- The quarto-markdown project is already Rust-based
- Tight integration with quarto-cli (Rust)
- Natural fit for their Rust toolchain

### 2. Type Safety
- Compile-time grammar validation
- Rust's type system catches errors early
- Better refactoring support

### 3. Auto-Generated Bindings
- rust-sitter automatically generates Rust bindings
- Reduces boilerplate for Rust consumers
- Type-safe AST traversal in Rust

### 4. Advanced Rust Features
- Can leverage Rust macros for meta-programming
- Procedural macros for compile-time code generation
- Better integration if building Rust-based language servers

## Real-World Examples

### JavaScript Grammars (Standard)
```
tree-sitter-javascript
tree-sitter-python
tree-sitter-typescript
tree-sitter-rust (!)
tree-sitter-go
tree-sitter-c
tree-sitter-cpp
tree-sitter-pandoc-markdown
tree-sitter-quarto (this project)
```

### Rust Grammars (Growing)
```
quarto-markdown (official Quarto)
Some experimental grammars using rust-sitter
```

## Performance Reality

**Myth:** Rust grammars are faster
**Reality:** Both produce identical C code

The tree-sitter CLI (written in Rust) processes the grammar and generates C code. The final parser is C in both cases, compiled to:
- Native code (for direct embedding)
- WebAssembly (for safe, cross-platform distribution)

Performance is determined by:
1. The grammar structure (not the language it's written in)
2. The generated C code (identical regardless of source)
3. The tree-sitter runtime (same for both)

## What About the Official Grammars?

The official quarto-markdown project uses Rust because:
1. They're building a **Rust-based toolchain** (quarto-cli uses Rust)
2. They need **tight Rust integration** for their rendering pipeline
3. They have **Rust expertise** on the team
4. They benefit from **auto-generated Rust bindings**

For **editor integration** (our use case), the language choice doesn't matter. Both approaches work equally well with Zed, Neovim, VSCode, and Helix.

## Migration Considerations

If we wanted to switch to Rust later:
- ✅ Would work fine (editors don't care)
- ⚠️ Requires rewriting grammar in Rust
- ⚠️ Adds Rust toolchain dependency
- ⚠️ Smaller contributor pool
- ✅ Same runtime performance

Current recommendation: **Stay with JavaScript** unless:
1. We build Rust-based tooling on top of the grammar
2. We need compile-time grammar validation
3. Community demands Rust for contributions

## Conclusion

**For tree-sitter-quarto, JavaScript is the right choice.**

We prioritize:
- ✅ Wide accessibility for contributors
- ✅ Proven, stable tooling
- ✅ Alignment with the tree-sitter ecosystem
- ✅ Simpler development workflow

The official quarto-markdown Rust grammars serve a different use case (Rust toolchain integration) and make sense for their project. Both approaches are valid, produce equally fast parsers, and can coexist in the Quarto ecosystem.

## References

- **Traditional approach:** https://tree-sitter.github.io/tree-sitter/creating-parsers
- **rust-sitter:** https://www.shadaj.me/writing/introducing-rust-sitter
- **quarto-markdown grammars:** https://github.com/quarto-dev/quarto-markdown
- **tree-sitter-pandoc-markdown (our base):** https://github.com/ck37/tree-sitter-pandoc-markdown
- **Performance discussion:** https://blog.jez.io/tree-sitter-limitations/

## See Also

- [docs/comparison.md](./comparison.md) - Full comparison with other Quarto parsers
- [CLAUDE.md](../CLAUDE.md) - Project overview and architecture decisions
