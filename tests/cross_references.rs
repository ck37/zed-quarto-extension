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
fn cross_references_are_highlighted() {
    let source = r#"See @fig-plot for the visualization.

The results are shown in @tbl-results.

As discussed in @sec-methods, we used @eq-model.
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

    // Check that reference types are highlighted
    assert!(
        rendered_output.contains("<constant.builtin>fig</>")
            || rendered_output.contains("<constant.builtin>tbl</>")
            || rendered_output.contains("<constant.builtin>sec</>")
            || rendered_output.contains("<constant.builtin>eq</>"),
        "Reference types (fig, tbl, sec, eq) should be highlighted"
    );

    // Check that reference IDs are highlighted
    assert!(
        rendered_output.contains("<variable.parameter>plot</>")
            || rendered_output.contains("<variable.parameter>results</>")
            || rendered_output.contains("<variable.parameter>methods</>")
            || rendered_output.contains("<variable.parameter>model</>"),
        "Reference IDs should be highlighted as variable.parameter"
    );

    // Check that hyphen separator is highlighted
    assert!(
        rendered_output.contains("<punctuation.delimiter>-</>"),
        "Hyphen separator in cross-references should be highlighted"
    );

    println!("✓ Cross-references are properly highlighted");
}

#[test]
fn cross_reference_types() {
    // Test all common cross-reference types
    let test_cases = vec![
        ("@fig-plot", "fig", "plot"),
        ("@tbl-data", "tbl", "data"),
        ("@sec-intro", "sec", "intro"),
        ("@eq-formula", "eq", "formula"),
        ("@lst-code", "lst", "code"),
        // Note: @thm-theorem may not be supported in all grammar versions
    ];

    for (source, _ref_type, _ref_id) in test_cases {
        let mut parser = Parser::new();
        let lang = language();
        parser.set_language(&lang).unwrap();
        let tree = parser.parse(source.as_bytes(), None).unwrap();

        let root = tree.root_node();
        eprintln!("\nTesting {}: {}", source, root.to_sexp());

        // Verify the tree structure has a cross_reference node
        let has_cross_ref = root.to_sexp().contains("cross_reference");
        assert!(
            has_cross_ref,
            "Source '{}' should parse as cross_reference",
            source
        );

        println!("✓ {} parses correctly as cross-reference", source);
    }
}
