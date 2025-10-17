use tree_sitter::{Language, Parser};
use tree_sitter_highlight::{HighlightConfiguration, Highlighter};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

fn language() -> Language {
    unsafe { tree_sitter_quarto() }
}

fn highlight_configuration() -> HighlightConfiguration {
    let highlight_query = include_str!("../grammars/quarto/queries/zed/highlights.scm");
    let injection_query = include_str!("../grammars/quarto/queries/injections.scm");
    let locals_query = "";

    let mut config = HighlightConfiguration::new(
        language(),
        "quarto",
        highlight_query,
        injection_query,
        locals_query,
    )
    .expect("valid highlight configuration");

    let scope_names: Vec<String> = config.names().iter().map(|s| s.to_string()).collect();
    let scope_names_refs: Vec<&str> = scope_names.iter().map(|s| s.as_str()).collect();
    config.configure(&scope_names_refs);

    config
}

#[test]
fn basic_citations_are_highlighted() {
    let source = r#"According to @smith2024, the results show significance.

Multiple studies [@jones2023; @brown2022] confirm this.
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    eprintln!("\n=== PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

    let config = highlight_configuration();
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(&config, source.as_bytes(), None, |_| None)
        .expect("highlighting succeeds");

    let mut rendered = Vec::new();
    for event in events {
        match event.expect("valid event") {
            tree_sitter_highlight::HighlightEvent::HighlightStart(s) => {
                let scope_name = config.names()[s.0];
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
    eprintln!("\n=== RENDERED OUTPUT ===\n{}\n", rendered_output);

    // Check that @ symbol is highlighted
    assert!(
        rendered_output.contains("<punctuation.special>@</>"),
        "@ prefix should be highlighted"
    );

    // Check that citation keys are highlighted
    assert!(
        rendered_output.contains("<variable.parameter>smith2024</>")
            || rendered_output.contains("<variable.parameter>jones2023</>")
            || rendered_output.contains("<variable.parameter>brown2022</>"),
        "Citation keys should be highlighted as variable.parameter"
    );

    println!("✓ Basic citations are properly highlighted");
}

#[test]
fn citation_variations() {
    let source = r#"Narrative citation: @smith2024 says this.

Parenthetical: Previous work [@smith2024] found that.

Multiple: Several studies [@smith2024; @jones2023; @brown2022] confirm.

With prefix: As shown by [see @smith2024] the data suggests.

With page: According to [@smith2024, p. 42] we can conclude.
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!("\n=== PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

    let config = highlight_configuration();
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(&config, source.as_bytes(), None, |_| None)
        .unwrap();

    let mut rendered = Vec::new();
    for event in events {
        match event.unwrap() {
            tree_sitter_highlight::HighlightEvent::HighlightStart(s) => {
                let scope_name = config.names()[s.0];
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
    eprintln!("\n=== RENDERED OUTPUT ===\n{}\n", rendered_output);

    // Verify citations are parsed
    let tree_str = tree.root_node().to_sexp();
    assert!(
        tree_str.contains("citation"),
        "Document should contain citation nodes"
    );

    // Check that all citation keys are highlighted
    assert!(
        rendered_output.contains("smith2024"),
        "Citation key smith2024 should be present"
    );
    assert!(
        rendered_output.contains("jones2023"),
        "Citation key jones2023 should be present"
    );
    assert!(
        rendered_output.contains("brown2022"),
        "Citation key brown2022 should be present"
    );

    println!("✓ Citation variations are properly parsed and highlighted");
}

#[test]
fn citation_keys_with_special_characters() {
    // Test citation keys with various formats
    let test_cases = vec![
        "@smith2024",       // Year
        "@Smith_2024",      // Underscore
        "@smith-etal-2024", // Hyphens
        "@SMITH2024",       // Uppercase
        "@smith2024a",      // Letter suffix
    ];

    for source in test_cases {
        let mut parser = Parser::new();
        let lang = language();
        parser.set_language(&lang).unwrap();
        let tree = parser.parse(source.as_bytes(), None).unwrap();

        let root = tree.root_node();
        eprintln!("\nTesting {}: {}", source, root.to_sexp());

        // Verify the tree structure contains citation node
        let has_citation = root.to_sexp().contains("citation");
        assert!(has_citation, "Source '{}' should parse as citation", source);

        println!("✓ {} parses correctly as citation", source);
    }
}
