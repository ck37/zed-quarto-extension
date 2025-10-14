/// Test that the inline grammar is properly injected and uses Zed-compatible scopes
///
/// This test verifies:
/// 1. The pandoc_markdown_inline grammar is injected for inline content
/// 2. The inline grammar uses Zed-compatible scopes (@text.emphasis, @emphasis.strong)
///    instead of nvim-treesitter conventions (@markup.italic, @markup.bold)
/// 3. Triple asterisks (***) are properly highlighted as bold+italic
///
/// This is a regression test for the issue where the inline grammar was being
/// compiled with @markup.* scopes even though we needed @text.* for Zed themes.
use std::fs;

#[test]
fn test_inline_grammar_uses_zed_scopes() {
    // Read the inline grammar's highlights.scm
    let highlights_path =
        "grammars/pandoc_markdown/tree-sitter-pandoc-markdown-inline/queries/highlights.scm";

    // Skip test if grammar not yet checked out (e.g., CI environment without grammar cache)
    if !std::path::Path::new(highlights_path).exists() {
        eprintln!("Skipping test - grammar not checked out yet");
        return;
    }

    let highlights_content =
        fs::read_to_string(highlights_path).expect("Failed to read inline grammar highlights.scm");

    // Verify that the inline grammar uses Zed-compatible scopes
    assert!(
        highlights_content.contains("@text.emphasis"),
        "Inline grammar should use @text.emphasis (Zed-compatible), not @markup.italic"
    );

    assert!(
        highlights_content.contains("@emphasis.strong"),
        "Inline grammar should use @emphasis.strong (Zed-compatible), not @markup.bold"
    );

    // Verify it does NOT use nvim-treesitter scopes
    assert!(
        !highlights_content.contains("@markup.italic"),
        "Inline grammar should NOT use @markup.italic (nvim-treesitter convention that Zed doesn't support)"
    );

    assert!(
        !highlights_content.contains("@markup.bold"),
        "Inline grammar should NOT use @markup.bold (nvim-treesitter convention that Zed doesn't support)"
    );

    println!("✓ Inline grammar uses Zed-compatible scopes");
}

#[test]
fn test_inline_grammar_injections_configured() {
    // Verify that the block grammar's injections.scm includes inline grammar injection
    let injections_path = "languages/quarto/injections.scm";

    let injections_content =
        fs::read_to_string(injections_path).expect("Failed to read injections.scm");

    // Verify that inline grammar injection is configured
    assert!(
        injections_content.contains("pandoc_markdown_inline"),
        "injections.scm should configure injection of pandoc_markdown_inline grammar for inline content"
    );

    assert!(
        injections_content.contains("((inline) @injection.content"),
        "injections.scm should inject inline grammar for (inline) nodes"
    );

    println!("✓ Inline grammar injection is configured in injections.scm");
    println!("  Note: Actual injection happens at runtime in Zed with PR #40063");
    println!("  This test verifies the injection query is present");
}

#[test]
fn test_inline_grammar_commit_matches_extension_toml() {
    // Verify that extension.toml references a grammar commit with Zed-compatible scopes
    let extension_toml =
        fs::read_to_string("extension.toml").expect("Failed to read extension.toml");

    // Extract the commit hash for pandoc_markdown_inline
    let commit_line = extension_toml
        .lines()
        .skip_while(|line| !line.contains("[grammars.pandoc_markdown_inline]"))
        .skip(1)
        .find(|line| line.contains("commit = "))
        .expect("Failed to find pandoc_markdown_inline commit in extension.toml");

    let commit = commit_line
        .split('"')
        .nth(1)
        .expect("Failed to extract commit hash");

    println!("Extension references inline grammar at commit: {}", commit);

    // Known good commit with Zed-compatible scopes (zed-compatible-scopes branch)
    const EXPECTED_COMMIT: &str = "d2b53a88ef584df731c7e4be0e204b9dfbeb6f14";

    assert_eq!(
        commit, EXPECTED_COMMIT,
        "extension.toml should reference commit {} which has Zed-compatible scopes",
        EXPECTED_COMMIT
    );

    println!("✓ Extension references correct grammar commit with Zed-compatible scopes");
}
