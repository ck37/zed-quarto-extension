/// Automated validation that highlights.scm only uses Zed-supported syntax scopes
///
/// This test extracts all @scope captures from highlights.scm and validates them against
/// the canonical list of scopes supported by Zed's theme system.
///
/// Reference: docs/zed-syntax-scopes.md
use std::collections::HashSet;
use std::path::Path;

/// All syntax highlight scopes supported by Zed's theme system
///
/// Extracted from zed-industries/zed:
/// - crates/theme_importer/src/vscode/syntax.rs (canonical list)
/// - crates/languages/src/markdown/highlights.scm (markdown usage patterns)
///
/// Last updated: 2025-10-17
const ZED_SUPPORTED_SCOPES: &[&str] = &[
    // Core token types
    "attribute",
    "boolean",
    "comment",
    "comment.doc",
    "constant",
    "constructor",
    "embedded",
    "emphasis",
    "emphasis.strong",
    "enum",
    "function",
    "hint",
    "keyword",
    "label",
    "link_text",
    "link_uri",
    "number",
    "operator",
    "predictive",
    "preproc",
    "primary",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.list_marker",
    "punctuation.special",
    "string",
    "string.escape",
    "string.regex",
    "string.special",
    "string.special.symbol",
    "tag",
    "text.literal",
    "title",
    "type",
    "variable",
    "variable.special",
    "variant",
    // Language-specific conventions (used by markdown)
    // These extend base scopes with .markup, .rust, etc. suffixes
    "title.markup",
    "punctuation.markup",
    "punctuation.list_marker.markup",
    "punctuation.embedded.markup",
    "link_text.markup",
    "link_uri.markup",
    // Special scopes
    "text", // Plain text
    "none", // No highlighting
    "parameter", // Function parameters

            // Allow language-specific sub-scopes
            // These follow the pattern: base_scope.language_suffix
            // Examples: variable.builtin, function.method, variable.builtin.self.rust
];

/// Additional scope patterns that should be allowed
/// These are regex patterns for valid sub-scope extensions
const ALLOWED_SCOPE_PATTERNS: &[&str] = &[
    r"^(text|emphasis|function|variable|constant|string|punctuation)\..*$", // Sub-scopes
    r"^.*\.builtin$",                                                       // Built-in variants
    r"^.*\.method$",                                                        // Method variants
    r"^.*\.parameter$",                                                     // Parameter variants
];

#[test]
fn all_scopes_are_zed_compatible() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let highlights_path = manifest_dir.join("grammars/quarto-vendored/queries/zed/highlights.scm");

    let highlights =
        std::fs::read_to_string(&highlights_path).expect("Failed to read highlights.scm");

    // Filter out comment lines before extracting scopes
    let non_comment_lines: Vec<&str> = highlights
        .lines()
        .filter(|line| !line.trim_start().starts_with(';'))
        .collect();
    let non_comment_content = non_comment_lines.join("\n");

    // Extract all @ captures from the query file
    let scopes = extract_scopes(&non_comment_content);

    let mut unsupported_scopes = Vec::new();
    let mut scope_usage = std::collections::HashMap::new();

    for scope in &scopes {
        *scope_usage.entry(scope.as_str()).or_insert(0) += 1;

        if !is_scope_supported(scope) {
            unsupported_scopes.push(scope.clone());
        }
    }

    if !unsupported_scopes.is_empty() {
        eprintln!("\n❌ Found unsupported scopes in highlights.scm:\n");

        let mut unique_unsupported: Vec<_> = unsupported_scopes
            .iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        unique_unsupported.sort();
        let unsupported_count = unique_unsupported.len();

        for scope in &unique_unsupported {
            let count = scope_usage.get(scope.as_str()).unwrap_or(&0);
            eprintln!("  @{} (used {} times)", scope, count);
        }

        eprintln!("\nThese scopes are not in Zed's supported scope list.");
        eprintln!("See docs/zed-syntax-scopes.md for the complete list of supported scopes.");
        eprintln!("\nCommon mistakes:");
        eprintln!("  @text.title -> use @title or @title.markup");
        eprintln!("  @text.emphasis -> use @emphasis");
        eprintln!("  @text.reference -> use @link_text or @link_text.markup");
        eprintln!("  @text.uri -> use @link_uri or @link_uri.markup");
        eprintln!("  @markup.* -> Zed doesn't use nvim-treesitter @markup.* scopes");

        panic!(
            "highlights.scm contains {} unsupported scopes",
            unsupported_count
        );
    }
}

#[test]
fn no_nvim_treesitter_scopes() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let highlights_path = manifest_dir.join("grammars/quarto-vendored/queries/zed/highlights.scm");

    let highlights =
        std::fs::read_to_string(&highlights_path).expect("Failed to read highlights.scm");

    // Filter out comment lines
    let non_comment_lines: Vec<&str> = highlights
        .lines()
        .filter(|line| !line.trim_start().starts_with(';'))
        .collect();
    let non_comment_content = non_comment_lines.join("\n");

    let nvim_scopes = [
        "@markup.heading",
        "@markup.italic",
        "@markup.bold",
        "@markup.raw",
        "@markup.link",
        "@markup.list",
        "@markup.quote",
        "@markup.math",
    ];

    let mut found_nvim = Vec::new();
    for scope in &nvim_scopes {
        if non_comment_content.contains(scope) {
            found_nvim.push(*scope);
        }
    }

    assert!(
        found_nvim.is_empty(),
        "highlights.scm contains nvim-treesitter @markup.* scopes: {:?}\n\
         Zed doesn't support these. See docs/zed-syntax-scopes.md for Zed-compatible scopes.",
        found_nvim
    );
}

#[test]
fn uses_recommended_markdown_scopes() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let highlights_path = manifest_dir.join("grammars/quarto-vendored/queries/zed/highlights.scm");

    let highlights =
        std::fs::read_to_string(&highlights_path).expect("Failed to read highlights.scm");

    // Check that we're using the recommended scopes for markdown-like content
    let scopes = extract_scopes(&highlights);
    let scope_set: HashSet<_> = scopes.iter().map(|s| s.as_str()).collect();

    // These should be present for a Quarto/Markdown extension
    let recommended = [
        "emphasis",            // Italic
        "emphasis.strong",     // Bold
        "title",               // Headings
        "text.literal",        // Code spans
        "link_uri",            // URLs
        "link_text",           // Link labels
        "punctuation.special", // Markers
    ];

    let mut missing = Vec::new();
    for scope in &recommended {
        // Check both with and without .markup suffix
        if !scope_set.contains(scope) && !scope_set.contains(&format!("{}.markup", scope).as_str())
        {
            missing.push(*scope);
        }
    }

    if !missing.is_empty() {
        eprintln!("\n⚠️  Warning: Missing recommended scopes for markdown content:");
        for scope in missing {
            eprintln!("  @{}", scope);
        }
        eprintln!("\nThese scopes are commonly used in markdown/Quarto documents.");
        eprintln!("Consider adding them to improve syntax highlighting.");
    }
}

#[test]
fn documents_all_used_scopes() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let highlights_path = manifest_dir.join("grammars/quarto-vendored/queries/zed/highlights.scm");
    let docs_path = manifest_dir.join("docs/zed-syntax-scopes.md");

    let highlights =
        std::fs::read_to_string(&highlights_path).expect("Failed to read highlights.scm");
    let docs = std::fs::read_to_string(&docs_path).expect("Failed to read zed-syntax-scopes.md");

    let scopes = extract_scopes(&highlights);
    let unique_scopes: HashSet<_> = scopes.iter().map(|s| s.as_str()).collect();

    let mut undocumented = Vec::new();
    for scope in &unique_scopes {
        // Check if scope appears in docs (either as `scope` or @scope)
        if !docs.contains(&format!("`{}`", scope)) && !docs.contains(&format!("@{}", scope)) {
            undocumented.push(*scope);
        }
    }

    if !undocumented.is_empty() {
        let mut sorted: Vec<_> = undocumented.into_iter().collect();
        sorted.sort();

        eprintln!(
            "\n⚠️  Warning: These scopes are used but not documented in docs/zed-syntax-scopes.md:"
        );
        for scope in sorted {
            eprintln!("  @{}", scope);
        }
        eprintln!("\nConsider adding them to the documentation.");
    }
}

// Helper functions

fn extract_scopes(query: &str) -> Vec<String> {
    let mut scopes = Vec::new();

    // Match @ followed by word characters, dots, underscores
    let re = regex::Regex::new(r"@([\w\._]+)").unwrap();

    for cap in re.captures_iter(query) {
        if let Some(scope) = cap.get(1) {
            scopes.push(scope.as_str().to_string());
        }
    }

    scopes
}

fn is_scope_supported(scope: &str) -> bool {
    // Check if it's in the explicit list
    if ZED_SUPPORTED_SCOPES.contains(&scope) {
        return true;
    }

    // Check if it matches allowed patterns
    for pattern in ALLOWED_SCOPE_PATTERNS {
        let re = regex::Regex::new(pattern).unwrap();
        if re.is_match(scope) {
            return true;
        }
    }

    // Special cases that should be allowed
    match scope {
        "text" | "none" | "parameter" => true,
        _ => false,
    }
}

// Additional helper test to list all scopes we use
#[test]
fn list_all_used_scopes() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let highlights_path = manifest_dir.join("grammars/quarto-vendored/queries/zed/highlights.scm");

    let highlights =
        std::fs::read_to_string(&highlights_path).expect("Failed to read highlights.scm");

    let scopes = extract_scopes(&highlights);
    let mut scope_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for scope in scopes {
        *scope_counts.entry(scope).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = scope_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    eprintln!("\n📋 All scopes used in highlights.scm (by frequency):\n");
    for (scope, count) in sorted {
        let supported = if is_scope_supported(&scope) {
            "✓"
        } else {
            "✗"
        };
        eprintln!("  {} @{:30} (used {} times)", supported, scope, count);
    }
}
