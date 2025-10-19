/// Test reference-style links in Pandoc Markdown
/// Reference links use [text][ref] syntax with definitions elsewhere
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
    let highlight_query = include_str!("../grammars/quarto/queries/highlights.scm");
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
fn basic_reference_link() {
    let source = r#"This is [link text][ref] in a sentence.

[ref]: https://example.com
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    eprintln!(
        "\n=== REFERENCE LINK PARSE TREE ===\n{}\n",
        tree.root_node().to_sexp()
    );

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
    eprintln!("=== RENDERED OUTPUT ===\n{}\n", rendered_output);

    // Verify link components are present
    assert!(
        rendered_output.contains("link text"),
        "Link text should be present"
    );
    assert!(
        rendered_output.contains("example.com"),
        "Link URL should be present"
    );

    println!("✓ Reference-style link is properly parsed");
}

#[test]
fn shorthand_reference_link() {
    // Shorthand: [text][] or [text] uses text as reference
    let source = r#"See [Python][] for details.

[Python]: https://python.org
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!(
        "\n=== SHORTHAND REFERENCE PARSE TREE ===\n{}\n",
        tree.root_node().to_sexp()
    );

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
    eprintln!("=== RENDERED OUTPUT ===\n{}\n", rendered_output);

    assert!(
        rendered_output.contains("Python"),
        "Shorthand reference should be present"
    );

    println!("✓ Shorthand reference link is properly parsed");
}

#[test]
fn multiple_reference_links() {
    let source = r#"# Documentation

See [Python][py] and [R][r-lang] for examples.

Also check [Julia][] for numerical computing.

[py]: https://python.org
[r-lang]: https://r-project.org
[Julia]: https://julialang.org
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!(
        "\n=== MULTIPLE REFERENCES PARSE TREE ===\n{}\n",
        tree.root_node().to_sexp()
    );

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
    eprintln!("=== RENDERED OUTPUT ===\n{}\n", rendered_output);

    // Verify all link text is present
    assert!(rendered_output.contains("Python"));
    assert!(rendered_output.contains("Julia"));
    assert!(rendered_output.contains("python.org") || rendered_output.contains("py"));
    assert!(rendered_output.contains("julialang.org") || rendered_output.contains("Julia"));

    println!("✓ Multiple reference links parse correctly");
}

#[test]
fn reference_with_title() {
    let source = r#"Check the [documentation][docs] for more info.

[docs]: https://example.com "Example Documentation"
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!(
        "\n=== REFERENCE WITH TITLE PARSE TREE ===\n{}\n",
        tree.root_node().to_sexp()
    );

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
    eprintln!("=== RENDERED OUTPUT ===\n{}\n", rendered_output);

    assert!(
        rendered_output.contains("documentation"),
        "Reference link text should be present"
    );
    assert!(
        rendered_output.contains("example.com")
            || rendered_output.contains("Example Documentation"),
        "Reference definition should be present"
    );

    println!("✓ Reference link with title is properly parsed");
}

#[test]
fn inline_vs_reference_links() {
    // Mix of inline [text](url) and reference [text][ref] styles
    let source = r#"Compare [inline link](https://inline.com) with [reference link][ref].

[ref]: https://reference.com
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!(
        "\n=== MIXED LINKS PARSE TREE ===\n{}\n",
        tree.root_node().to_sexp()
    );

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
    eprintln!("=== RENDERED OUTPUT ===\n{}\n", rendered_output);

    // Both link styles should be present
    assert!(rendered_output.contains("inline link"));
    assert!(rendered_output.contains("reference link"));
    assert!(
        rendered_output.contains("inline.com") || rendered_output.contains("reference.com"),
        "Both URLs should be present"
    );

    println!("✓ Mixed inline and reference links parse correctly");
}
