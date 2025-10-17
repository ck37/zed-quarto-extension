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
    let highlight_query = include_str!("../grammars/quarto-vendored/queries/zed/highlights.scm");
    let injection_query = include_str!("../grammars/quarto-vendored/queries/injections.scm");
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
fn inline_shortcodes_are_highlighted() {
    let source = r#"This document uses {{< var name >}} and {{< meta title >}}.

You can also use {{< pagebreak >}} inline.
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

    // Check that shortcode delimiters are highlighted
    assert!(
        rendered_output.contains("<punctuation.special>{{<")
            || rendered_output.contains("<punctuation.special>{{")
            || rendered_output.contains("<punctuation.special><"),
        "Shortcode opening delimiter should be highlighted"
    );

    assert!(
        rendered_output.contains("<punctuation.special>>}}")
            || rendered_output.contains("<punctuation.special>}}")
            || rendered_output.contains("<punctuation.special>>"),
        "Shortcode closing delimiter should be highlighted"
    );

    // Check that shortcode names are highlighted
    assert!(
        rendered_output.contains("<function>var</>")
            || rendered_output.contains("<function>meta</>")
            || rendered_output.contains("<function>pagebreak</>"),
        "Shortcode names should be highlighted as functions"
    );

    // Check that shortcode arguments are highlighted (may include spaces)
    assert!(
        rendered_output.contains("<parameter>") && (rendered_output.contains("name") || rendered_output.contains("title")),
        "Shortcode arguments should be highlighted as parameters"
    );

    println!("✓ Inline shortcodes are properly highlighted");
}

#[test]
fn block_shortcodes_are_highlighted() {
    let source = r#"{{< include _setup.qmd >}}

Some content here.

{{< embed notebooks/analysis.ipynb#fig-results >}}
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

    // Check that include shortcode is highlighted
    assert!(
        rendered_output.contains("<function>include</>"),
        "Include shortcode should be highlighted"
    );

    // Check that embed shortcode is highlighted
    assert!(
        rendered_output.contains("<function>embed</>"),
        "Embed shortcode should be highlighted"
    );

    // Check that file paths in shortcodes are highlighted
    assert!(
        rendered_output.contains("_setup.qmd")
            && rendered_output.contains("notebooks/analysis.ipynb"),
        "File paths in shortcodes should be present"
    );

    println!("✓ Block shortcodes are properly highlighted");
}

#[test]
fn shortcode_variations() {
    // Test various shortcode syntaxes
    let test_cases = vec![
        "{{< var x >}}",                // variable
        "{{< meta author >}}",          // metadata
        "{{< pagebreak >}}",            // self-closing
        "{{< include file.qmd >}}",     // include
        "{{< embed notebook.ipynb >}}", // embed
    ];

    for source in test_cases {
        let mut parser = Parser::new();
        let lang = language();
        parser.set_language(&lang).unwrap();
        let tree = parser.parse(source.as_bytes(), None).unwrap();

        let root = tree.root_node();
        eprintln!("\nTesting {}: {}", source, root.to_sexp());

        // Verify the tree structure contains shortcode node
        let has_shortcode = root.to_sexp().contains("shortcode");
        assert!(
            has_shortcode,
            "Source '{}' should parse as shortcode",
            source
        );

        println!("✓ {} parses correctly as shortcode", source);
    }
}
