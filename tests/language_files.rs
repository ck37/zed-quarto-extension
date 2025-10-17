/// Verify that required language configuration files exist
///
/// Even when using a grammar from a GitHub repository, Zed extensions still need
/// their own language configuration files in languages/<lang>/ directory.
///
/// Why this is needed:
/// - Zed loads grammars from the repository specified in extension.toml
/// - BUT language-specific configuration (queries, settings) come from the extension
/// - The grammar provides the parser; the extension provides the editor integration
///
/// Required files:
/// - config.toml: Language metadata (file extensions, comment syntax, etc.)
/// - highlights.scm: Syntax highlighting queries (maps AST nodes to semantic scopes)
/// - injections.scm: Language injection rules (embedded code blocks, YAML, etc.)
///
/// Optional but recommended:
/// - indents.scm: Indentation rules
/// - folds.scm: Code folding rules
/// - outline.scm: Document outline/structure
/// - tags.scm: Symbol navigation
/// - locals.scm: Local scope support
/// - textobjects.scm: Text object selection
use std::path::Path;

#[test]
fn language_config_exists() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("languages/quarto/config.toml");

    assert!(
        config_path.exists(),
        "languages/quarto/config.toml is required for language registration"
    );

    let config_content = std::fs::read_to_string(&config_path)
        .expect("Failed to read config.toml");

    // Verify it contains essential configuration
    assert!(
        config_content.contains("name = \"Quarto\""),
        "config.toml should define language name"
    );
    assert!(
        config_content.contains("path_suffixes = [\"qmd\"]"),
        "config.toml should define .qmd file extension"
    );
}

#[test]
fn highlights_query_exists() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let highlights_path = manifest_dir.join("languages/quarto/highlights.scm");

    assert!(
        highlights_path.exists(),
        "languages/quarto/highlights.scm is REQUIRED for syntax highlighting.\n\
         Even though the grammar is loaded from GitHub, Zed needs the extension\n\
         to provide highlight queries that map grammar nodes to semantic scopes."
    );

    let highlights = std::fs::read_to_string(&highlights_path)
        .expect("Failed to read highlights.scm");

    // Verify it's not empty and contains at least some queries
    assert!(
        highlights.len() > 100,
        "highlights.scm should contain actual query rules"
    );
    assert!(
        highlights.contains('@'),
        "highlights.scm should contain tree-sitter captures (marked with @)"
    );
}

#[test]
fn injections_query_exists() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let injections_path = manifest_dir.join("languages/quarto/injections.scm");

    assert!(
        injections_path.exists(),
        "languages/quarto/injections.scm is REQUIRED for embedded language support.\n\
         This file defines how Zed should parse embedded code (Python, R, YAML, etc.)\n\
         inside Quarto documents."
    );

    let injections = std::fs::read_to_string(&injections_path)
        .expect("Failed to read injections.scm");

    // Verify it contains language injections
    assert!(
        injections.contains("#set! injection.language") || injections.contains("@injection.content"),
        "injections.scm should contain injection directives"
    );
}

#[test]
fn optional_query_files_documented() {
    // This test documents which optional files we have
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let quarto_dir = manifest_dir.join("languages/quarto");

    let optional_files = [
        ("indents.scm", "Indentation rules"),
        ("folds.scm", "Code folding support"),
        ("outline.scm", "Document outline"),
        ("tags.scm", "Symbol navigation"),
        ("locals.scm", "Local scope support"),
        ("textobjects.scm", "Text object selection"),
    ];

    println!("\n📋 Optional language query files:");
    for (filename, description) in optional_files {
        let path = quarto_dir.join(filename);
        let status = if path.exists() { "✅" } else { "❌" };
        println!("  {} {} - {}", status, filename, description);
    }
}

#[test]
fn queries_compatible_with_grammar() {
    // Verify that query files use Zed-compatible scopes
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let highlights_path = manifest_dir.join("languages/quarto/highlights.scm");

    let highlights = std::fs::read_to_string(&highlights_path)
        .expect("Failed to read highlights.scm");

    // Check that we're NOT using nvim-treesitter @markup.* scopes
    // (tree-sitter-quarto grammar has Zed scopes in queries/highlights.scm,
    //  but we also need them in the extension's languages/quarto/highlights.scm)
    let nvim_scopes = [
        "@markup.heading",
        "@markup.bold",
        "@markup.italic",
        "@markup.raw",
    ];

    for scope in nvim_scopes {
        assert!(
            !highlights.contains(scope),
            "highlights.scm should NOT use nvim-treesitter scope: {}\n\
             Use Zed-compatible scopes instead (@title, @emphasis.strong, @text.emphasis, @text.literal)",
            scope
        );
    }

    // Verify we're using Zed-compatible scopes
    let zed_scopes = [
        "@title",           // Headings
        "@emphasis.strong", // Bold
        "@text.emphasis",   // Italic
        "@text.literal",    // Code
    ];

    let has_any_zed_scope = zed_scopes.iter().any(|scope| highlights.contains(scope));
    assert!(
        has_any_zed_scope,
        "highlights.scm should use Zed-compatible scopes like @title, @emphasis.strong, etc."
    );
}
