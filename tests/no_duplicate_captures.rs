/// Test that emphasis/strong captures don't appear in both block and inline grammars
///
/// This is a regression test for a critical bug where both grammars captured
/// emphasis/strong_emphasis, creating overlapping scopes that caused triple asterisks
/// to render as plain text (no highlighting at all).
///
/// The fix: Only the inline grammar should capture emphasis/strong. The block grammar
/// should delegate all inline formatting to the injected inline grammar.
use std::fs;

#[test]
fn test_block_grammar_does_not_capture_emphasis() {
    let block_highlights = fs::read_to_string("languages/quarto/highlights.scm")
        .expect("Failed to read block grammar highlights.scm");

    // Block grammar should NOT capture emphasis or strong_emphasis
    // These should only be captured by the inline grammar
    assert!(
        !block_highlights.contains("(emphasis) @"),
        "Block grammar must NOT capture (emphasis) nodes - this creates overlapping scopes with inline grammar.\n\
         Emphasis should only be captured by languages/pandoc_markdown_inline/highlights.scm"
    );

    assert!(
        !block_highlights.contains("(strong_emphasis) @"),
        "Block grammar must NOT capture (strong_emphasis) nodes - this creates overlapping scopes with inline grammar.\n\
         Strong emphasis should only be captured by languages/pandoc_markdown_inline/highlights.scm"
    );

    println!("✓ Block grammar correctly delegates emphasis/strong to inline grammar");
}

#[test]
fn test_inline_grammar_captures_emphasis() {
    let inline_highlights = fs::read_to_string("languages/pandoc_markdown_inline/highlights.scm")
        .expect("Failed to read inline grammar highlights.scm");

    // Inline grammar SHOULD capture emphasis and strong_emphasis
    assert!(
        inline_highlights.contains("@text.emphasis") || inline_highlights.contains("@emphasis"),
        "Inline grammar must capture emphasis nodes with @text.emphasis or similar scope"
    );

    assert!(
        inline_highlights.contains("@emphasis.strong"),
        "Inline grammar must capture strong_emphasis nodes with @emphasis.strong scope"
    );

    println!("✓ Inline grammar correctly captures emphasis and strong emphasis");
}

#[test]
fn test_no_overlapping_inline_captures() {
    // These are inline-level constructs that should ONLY be in the inline grammar
    // If they appear in the block grammar, they'll create scope conflicts
    let inline_only_nodes = vec![
        "(emphasis)",
        "(strong_emphasis)",
        "(code_span)",
        // Note: Some inline nodes like links can appear in both grammars,
        // so we only test the ones that are purely inline formatting
    ];

    let block_highlights = fs::read_to_string("languages/quarto/highlights.scm")
        .expect("Failed to read block grammar highlights.scm");

    for node in inline_only_nodes {
        // Check if the block grammar captures this node
        let pattern = format!("{} @", node);
        if block_highlights.contains(&pattern) {
            panic!(
                "Block grammar should NOT capture {} - this creates overlapping scopes.\n\
                 This node should only be captured by the inline grammar to prevent conflicts.",
                node
            );
        }
    }

    println!("✓ No overlapping inline-level captures detected");
}

#[test]
fn test_block_grammar_has_injection_comment() {
    let block_highlights = fs::read_to_string("languages/quarto/highlights.scm")
        .expect("Failed to read block grammar highlights.scm");

    // The block grammar should have a comment explaining why emphasis/strong are not captured
    assert!(
        block_highlights.contains("inline grammar") &&
        (block_highlights.contains("DO NOT capture") || block_highlights.contains("handled by")),
        "Block grammar should have a comment explaining that emphasis/strong are handled by inline grammar.\n\
         This helps prevent future developers from re-adding these captures."
    );

    println!("✓ Block grammar has documentation about inline grammar delegation");
}

#[test]
fn test_inline_grammar_documents_capture_strategy() {
    let inline_highlights = fs::read_to_string("languages/pandoc_markdown_inline/highlights.scm")
        .expect("Failed to read inline grammar highlights.scm");

    // The inline grammar should have a comment explaining its capture strategy
    assert!(
        inline_highlights.contains("Strategy:") ||
        (inline_highlights.contains("capture") && inline_highlights.contains("nodes")),
        "Inline grammar should document its capture strategy (container nodes vs text nodes)"
    );

    // Verify we're capturing emphasis and strong containers
    assert!(
        inline_highlights.contains("(emphasis) @") &&
        inline_highlights.contains("(strong_emphasis) @"),
        "Inline grammar should capture emphasis and strong_emphasis container nodes"
    );

    println!("✓ Inline grammar correctly documents and implements container-based capture strategy");
}

#[test]
fn test_emphasis_handling_is_documented() {
    // Check that our scope naming decision document mentions this issue
    let scope_doc = fs::read_to_string("docs/scope-naming-decision.md")
        .expect("Failed to read scope-naming-decision.md");

    // The document should mention the inline grammar or injection
    assert!(
        scope_doc.contains("inline") || scope_doc.contains("injection"),
        "docs/scope-naming-decision.md should document the inline grammar approach"
    );

    println!("✓ Emphasis handling is documented in scope-naming-decision.md");
}
