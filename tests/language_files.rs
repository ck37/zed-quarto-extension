/// Verify that required language configuration files exist
///
/// # Why Language Query Files Are Required
///
/// Even when using a grammar from a GitHub repository, Zed extensions MUST provide
/// their own language configuration files in the `languages/<lang>/` directory.
///
/// ## Architecture: Grammar vs Extension
///
/// In extension.toml, we specify a grammar from GitHub:
/// ```toml
/// [grammars.quarto]
/// repository = "https://github.com/ck37/tree-sitter-quarto"
/// rev = "9f7e5d2..."
/// ```
///
/// But Zed still requires the extension to provide language files:
///
/// 1. **Grammar (from GitHub)** → Provides the **parser** (how to build the AST)
/// 2. **Extension (local files)** → Provides **editor integration** (how to use the AST)
///
/// ## Division of Responsibilities
///
/// - **Grammar repository**: Defines syntax rules, creates parse tree from source code
/// - **Extension's languages/ dir**: Maps parse tree to editor features (colors, folding, navigation)
///
/// ## Required Files
///
/// These files MUST exist or the extension won't work:
/// - `config.toml`: Language metadata (file extensions, comment syntax, tab settings)
/// - `highlights.scm`: Syntax highlighting queries (maps AST nodes → semantic scopes for themes)
/// - `injections.scm`: Language injection rules (embedded Python, R, YAML in Quarto documents)
///
/// ## Optional but Recommended Files
///
/// These enhance the editing experience:
/// - `indents.scm`: Smart indentation rules
/// - `folds.scm`: Code folding support (collapse sections)
/// - `outline.scm`: Document outline/structure for navigation panel
/// - `tags.scm`: Symbol navigation (go to definition, find references)
/// - `locals.scm`: Local scope support for accurate symbol resolution
/// - `textobjects.scm`: Text object selection (e.g., select inside function)
///
/// ## What Happens Without These Files
///
/// Without language query files, the extension will:
/// - ✅ Install successfully in Zed
/// - ✅ Load the grammar from GitHub
/// - ✅ Parse .qmd files correctly
/// - ❌ Provide NO syntax highlighting (everything appears as plain text)
/// - ❌ Provide NO code folding, outline, or other features
///
/// ## This Test Prevents That
///
/// This test validates all required files exist and will fail if they're missing,
/// preventing the "extension installs but doesn't work" issue from occurring.
use std::path::Path;

#[test]
fn language_config_exists() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("languages/quarto/config.toml");

    assert!(
        config_path.exists(),
        "languages/quarto/config.toml is required for language registration"
    );

    let config_content = std::fs::read_to_string(&config_path).expect("Failed to read config.toml");

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

    let highlights =
        std::fs::read_to_string(&highlights_path).expect("Failed to read highlights.scm");

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

    let injections =
        std::fs::read_to_string(&injections_path).expect("Failed to read injections.scm");

    // Verify it contains language injections
    assert!(
        injections.contains("#set! injection.language")
            || injections.contains("@injection.content"),
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

    let highlights =
        std::fs::read_to_string(&highlights_path).expect("Failed to read highlights.scm");

    // Check that we're NOT using nvim-treesitter @markup.* scopes in actual queries
    // (tree-sitter-quarto grammar has Zed scopes in queries/highlights.scm,
    //  but we also need them in the extension's languages/quarto/highlights.scm)
    //
    // Filter out comments before checking for nvim scopes
    let query_lines: Vec<_> = highlights
        .lines()
        .filter(|line| !line.trim().starts_with(';'))
        .collect();
    let queries_only = query_lines.join("\n");

    let nvim_scopes = [
        "@markup.heading",
        "@markup.bold",
        "@markup.italic",
        "@markup.raw",
    ];

    for scope in nvim_scopes {
        assert!(
            !queries_only.contains(scope),
            "highlights.scm should NOT use nvim-treesitter scope in queries: {}\n\
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
