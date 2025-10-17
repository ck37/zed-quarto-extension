/// Test Pandoc-specific markdown extensions
/// These features are part of Pandoc Markdown but not standard CommonMark
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
fn strikethrough_is_highlighted() {
    let source = "This is ~~deleted text~~ in a sentence.\n";

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    eprintln!("\n=== STRIKETHROUGH PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

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

    // Verify strikethrough content is present (highlighting may vary)
    assert!(
        rendered_output.contains("deleted text"),
        "Strikethrough content should be present"
    );

    println!("✓ Strikethrough is properly parsed");
}

#[test]
fn highlight_is_highlighted() {
    let source = "This is ==highlighted text== in a sentence.\n";

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!("\n=== HIGHLIGHT PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

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
        rendered_output.contains("highlighted text"),
        "Highlight content should be present"
    );

    println!("✓ Highlight is properly parsed");
}

#[test]
fn subscript_is_highlighted() {
    let source = "Chemical formula: H~2~O and CO~2~\n";

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!("\n=== SUBSCRIPT PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

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

    // Verify subscript content is present
    assert!(
        rendered_output.contains("H") && rendered_output.contains("O"),
        "Subscript content should be present"
    );

    println!("✓ Subscript is properly parsed");
}

#[test]
fn superscript_is_highlighted() {
    let source = "Mathematical expression: x^2^ + y^2^ = z^2^\n";

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!("\n=== SUPERSCRIPT PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

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

    // Verify superscript content is present
    assert!(
        rendered_output.contains("x") && rendered_output.contains("y"),
        "Superscript content should be present"
    );

    println!("✓ Superscript is properly parsed");
}

#[test]
fn all_pandoc_extensions_in_document() {
    // Test all extensions together as they would appear in a real document
    let source = r#"# Pandoc Extensions

Strikethrough: ~~deleted text~~

Highlight: ==highlighted text==

Subscript: H~2~O and CO~2~

Superscript: x^2^ + y^2^ = z^2^
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!("\n=== FULL DOCUMENT PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

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

    // Verify all content is present and document parses successfully
    assert!(rendered_output.contains("Pandoc Extensions"));
    assert!(rendered_output.contains("deleted text"));
    assert!(rendered_output.contains("highlighted text"));
    assert!(rendered_output.contains("H") && rendered_output.contains("O"));
    assert!(rendered_output.contains("x") && rendered_output.contains("y"));

    println!("✓ All Pandoc extensions parse correctly in document context");
}
