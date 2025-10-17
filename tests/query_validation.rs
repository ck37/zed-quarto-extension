use std::path::Path;
/// Test that validates highlights.scm can be parsed by tree-sitter
/// This ensures our query file syntax is valid
use std::process::Command;

#[test]
fn highlights_query_is_valid_syntax() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let highlights_path = manifest_dir.join("grammars/quarto/queries/highlights.scm");
    let grammar_dir = manifest_dir.join("grammars/quarto");

    // Grammar is now vendored in the repo, so just verify it exists
    assert!(
        grammar_dir.exists(),
        "Grammar directory should exist at grammars/quarto (vendored in repo)"
    );

    // Test that tree-sitter can parse our highlights query
    let output = Command::new("tree-sitter")
        .current_dir(&grammar_dir)
        .args(["query", highlights_path.to_str().unwrap()])
        .output();

    match output {
        Ok(result) => {
            if !result.status.success() {
                eprintln!("tree-sitter query failed:");
                eprintln!("stdout: {}", String::from_utf8_lossy(&result.stdout));
                eprintln!("stderr: {}", String::from_utf8_lossy(&result.stderr));
                panic!("highlights.scm contains syntax errors");
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("tree-sitter CLI not found - skipping query validation");
            eprintln!("Install with: npm install -g tree-sitter-cli");
            // Don't fail the test if tree-sitter CLI is not installed
        }
        Err(e) => panic!("Failed to run tree-sitter: {}", e),
    }
}

#[test]
fn highlights_uses_zed_compatible_scopes() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let highlights =
        std::fs::read_to_string(manifest_dir.join("grammars/quarto/queries/highlights.scm"))
            .expect("Failed to read highlights.scm");

    // Check that we're using Zed-compatible scopes, not standard tree-sitter scopes
    // Filter out comment lines before checking
    let non_comment_lines: Vec<&str> = highlights
        .lines()
        .filter(|line| !line.trim_start().starts_with(';'))
        .collect();
    let non_comment_content = non_comment_lines.join("\n");

    let incompatible_scopes = [
        "@markup.heading",
        "@markup.italic",
        "@markup.bold",
        "@markup.raw.inline",
        "@markup.raw.block",
        "@markup.link.text",
        "@markup.link.url",
    ];

    let mut found_incompatible = Vec::new();
    for scope in &incompatible_scopes {
        if non_comment_content.contains(scope) {
            found_incompatible.push(*scope);
        }
    }

    assert!(
        found_incompatible.is_empty(),
        "highlights.scm contains Zed-incompatible scopes: {:?}\n\
         These should be converted to Zed-compatible equivalents:\n\
         @markup.heading -> @text.title\n\
         @markup.italic -> @text.emphasis\n\
         @markup.bold -> @emphasis.strong\n\
         @markup.raw.inline -> @text.literal\n\
         @markup.link.text -> @text.reference\n\
         @markup.link.url -> @text.uri",
        found_incompatible
    );

    // Verify we ARE using Zed-compatible scopes
    let required_scopes = [
        "@text.title",
        "@text.emphasis",
        "@emphasis.strong",
        "@text.literal",
    ];

    for scope in &required_scopes {
        assert!(
            highlights.contains(scope),
            "highlights.scm missing required Zed-compatible scope: {}",
            scope
        );
    }
}
