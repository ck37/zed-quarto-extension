use tree_sitter::{Language, Parser};
use tree_sitter_highlight::{HighlightConfiguration, Highlighter};

// External linkage to tree-sitter-quarto grammar
#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

fn language() -> Language {
    unsafe { tree_sitter_quarto() }
}

fn highlight_configuration() -> HighlightConfiguration {
    let highlight_query = include_str!("../grammars/quarto/queries/highlights.scm");
    let injection_query = include_str!("../grammars/quarto/queries/injections.scm");
    let locals_query = "";

    eprintln!("\n=== QUERIES BEING USED ===");
    eprintln!(
        "Highlight query (first 500 chars):\n{}",
        &highlight_query[..500.min(highlight_query.len())]
    );
    eprintln!(
        "\nLooking for @text.title: {}",
        highlight_query.contains("@text.title")
    );
    eprintln!(
        "Looking for @punctuation.special: {}",
        highlight_query.contains("@punctuation.special")
    );
    eprintln!(
        "Looking for @markup.heading: {}",
        highlight_query.contains("@markup.heading")
    );
    eprintln!();

    // In tree-sitter 0.25, HighlightConfiguration may automatically include
    // the grammar's built-in queries. Let's create the config and see what happens.
    let mut config = HighlightConfiguration::new(
        language(),
        "quarto",
        highlight_query,
        injection_query,
        locals_query,
    )
    .expect("valid highlight configuration with tree-sitter-quarto grammar");

    // Get the list of all capture names used in the query
    eprintln!("=== QUERY CAPTURE NAMES (from HighlightConfiguration) ===");
    eprintln!("Total captures before configure: {}", config.names().len());
    for (idx, name) in config.names().iter().enumerate() {
        eprintln!("  [{}] {}", idx, name);
    }
    eprintln!();

    // In tree-sitter 0.25, we should only configure scope names that are actually
    // present in the query. HighlightConfiguration.names() gives us the list of
    // capture names used in our queries. We need to map these to theme scope names.
    //
    // The configure() method remaps query capture names to theme-compatible names.
    // For example, if our query has "@text.title", we can map it to whatever theme
    // scope name we want (e.g., "heading" or "title" or keep it as "text.title").
    //
    // Since we're already using theme-compatible names in our query (@text.title,
    // @emphasis.strong, etc.), we can just pass the same names to configure().
    let scope_names: Vec<String> = config.names().iter().map(|s| s.to_string()).collect();
    let scope_names_refs: Vec<&str> = scope_names.iter().map(|s| s.as_str()).collect();

    eprintln!("=== SCOPE NAMES TO CONFIGURE (from query captures) ===");
    for (idx, name) in scope_names_refs.iter().enumerate() {
        eprintln!("  [{}] {}", idx, name);
    }
    eprintln!();

    config.configure(&scope_names_refs);

    eprintln!("=== ACTUAL NAMES AFTER CONFIGURE ===");
    for (idx, name) in config.names().iter().enumerate() {
        eprintln!("  [{}] {}", idx, name);
    }
    eprintln!();

    config
}

#[test]
fn headers_are_highlighted() {
    let source = r#"## Simple Header

## Header-With-Hyphens

### Multi-Word-Header-Example

## data-driven analysis
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    // Debug: print the tree structure
    eprintln!("\n=== PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

    let config = highlight_configuration();
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(&config, source.as_bytes(), None, |_| None)
        .expect("highlighting succeeds");

    let mut rendered = Vec::new();
    let mut scopes_used = std::collections::HashSet::new();

    for event in events {
        match event.expect("valid event") {
            tree_sitter_highlight::HighlightEvent::HighlightStart(s) => {
                // In tree-sitter 0.25, we must only configure scope names that exist in the query.
                // The Highlight index maps directly to config.names() after calling configure().
                let scope_name = config.names()[s.0];
                scopes_used.insert(scope_name.to_string());
                rendered.push(format!("<{}>", scope_name));
            }
            tree_sitter_highlight::HighlightEvent::HighlightEnd => {
                rendered.push("</>".into());
            }
            tree_sitter_highlight::HighlightEvent::Source { start, end } => {
                rendered.push(source[start..end].to_string());
            }
        }
    }

    let rendered_output = rendered.join("");

    // Debug: print full rendered output
    eprintln!(
        "\n=== HEADING HIGHLIGHTING OUTPUT ===\n{}\n",
        rendered_output
    );

    eprintln!("\n=== SCOPES USED ===");
    let mut scopes_vec: Vec<_> = scopes_used.iter().collect();
    scopes_vec.sort();
    for scope in scopes_vec {
        eprintln!("  - {}", scope);
    }
    eprintln!();

    // Check what scopes are actually being used
    let has_expected_scopes = rendered_output.contains("<punctuation.special>")
        && rendered_output.contains("<text.title>");
    let has_grammar_scopes =
        rendered_output.contains("<constant.builtin>") || rendered_output.contains("<comment>");

    if has_grammar_scopes && !has_expected_scopes {
        eprintln!("⚠️  WARNING: Using grammar's built-in queries (with @markup.* scopes)");
        eprintln!("   Expected: @punctuation.special and @text.title");
        eprintln!("   Actual:   @constant.builtin and @comment");
        eprintln!();
        eprintln!("   This means Zed is loading tree-sitter-quarto's queries/highlights.scm");
        eprintln!("   instead of the extension's grammars/quarto/queries/highlights.scm");
        eprintln!();
        eprintln!("   See docs/highlighting-failure-analysis.md for details.");
        eprintln!();

        // Verify content is present even with wrong scopes
        assert!(
            rendered_output.contains("Simple Header"),
            "Content should be present"
        );
        assert!(
            rendered_output.contains("Header-With-Hyphens"),
            "Hyphenated headers should be present"
        );
        assert!(
            rendered_output.contains("Multi-Word-Header-Example"),
            "Multi-word headers should be present"
        );
        assert!(
            rendered_output.contains("data-driven"),
            "Headers with hyphens should be present"
        );

        eprintln!("✓ Headers with hyphens ARE parsed correctly");
        eprintln!("✓ Grammar structure is correct");
        eprintln!("✗ But queries are using wrong scope names (@markup.* instead of @text.*)");
        eprintln!();
        panic!("Extension queries not being loaded - Zed is using grammar's built-in queries");
    }

    // Test that heading markers are highlighted with correct scopes
    // Note: In tree-sitter-quarto, the heading marker includes the trailing space
    assert!(
        rendered_output.contains("<punctuation.special>## </>")
            || rendered_output.contains("<punctuation.special>##</>"),
        "Heading markers should be highlighted with punctuation.special"
    );

    assert!(
        rendered_output.contains("<punctuation.special>### </>")
            || rendered_output.contains("<punctuation.special>###</>"),
        "Level 3 heading markers should be highlighted"
    );

    // Test that heading content is highlighted
    assert!(
        rendered_output.contains("<text.title>"),
        "Heading content should be highlighted with text.title"
    );

    // Test specific headers with hyphens
    let has_simple_header = rendered_output.contains("Simple Header");
    let has_hyphenated_header = rendered_output.contains("Header-With-Hyphens");
    let has_multi_word_header = rendered_output.contains("Multi-Word-Header-Example");
    let has_data_driven = rendered_output.contains("data-driven");

    assert!(has_simple_header, "Should contain 'Simple Header'");
    assert!(
        has_hyphenated_header,
        "Should contain 'Header-With-Hyphens'"
    );
    assert!(
        has_multi_word_header,
        "Should contain 'Multi-Word-Header-Example'"
    );
    assert!(has_data_driven, "Should contain 'data-driven'");

    // Verify that hyphenated headers are also wrapped in text.title
    let title_sections: Vec<&str> = rendered_output.split("<text.title>").skip(1).collect();

    let mut found_hyphenated = false;
    for section in title_sections {
        if section.contains("Header-With-Hyphens") {
            found_hyphenated = true;
            eprintln!("✓ Found hyphenated header with text.title highlighting");
        }
    }

    assert!(
        found_hyphenated,
        "Hyphenated headers should be highlighted with text.title"
    );

    println!("\n✓ All heading tests passed - extension queries are being loaded correctly!");
}

#[test]
fn heading_content_uses_inline_capture() {
    // This test verifies our fix: content: (inline) @text.title
    let source = "## Test Heading\n";

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    // Verify the tree structure matches our expectations
    let root = tree.root_node();
    let heading = root.child(0).expect("should have heading node");

    assert_eq!(
        heading.kind(),
        "atx_heading",
        "First node should be atx_heading"
    );

    // Check for marker child
    let marker = heading
        .child_by_field_name("marker")
        .expect("heading should have marker");
    assert_eq!(marker.kind(), "atx_heading_marker");

    // Check for content child (inline node)
    let content = heading
        .child_by_field_name("content")
        .expect("heading should have content");
    assert_eq!(content.kind(), "inline", "content should be an inline node");

    // The inline node should contain the text
    let text = content.child(0).expect("inline should have text child");
    assert_eq!(text.kind(), "text");

    println!("✓ Tree structure matches expected pattern for heading queries");
}
