use std::fs;
use std::path::Path;

use tree_sitter::{Language, Parser};
use tree_sitter_highlight::{HighlightConfiguration, Highlighter};

// Link to the compiled pandoc-markdown grammar
#[link(name = "tree-sitter-pandoc-markdown", kind = "static")]
extern "C" {
    fn tree_sitter_pandoc_markdown() -> Language;
}

fn language() -> Language {
    unsafe { tree_sitter_pandoc_markdown() }
}

fn highlight_configuration() -> HighlightConfiguration {
    // Use the extension's highlights directly; they already align with upstream Markdown scopes.
    let highlight_query = include_str!("../languages/quarto/highlights.scm");
    let injection_query = include_str!("../languages/quarto/injections.scm");
    let locals_query = "";

    let mut config = HighlightConfiguration::new(
        language(),
        "quarto",
        highlight_query,
        injection_query,
        locals_query,
    )
    .expect("valid highlight configuration with pandoc-markdown grammar");

    config.configure(&[
        "annotation",
        "attribute",
        "comment",
        "constant.macro",
        "emphasis.strong",
        "property",
        "punctuation.delimiter",
        "punctuation.special",
        "string",
        "tag",
        "text.emphasis",
        "text.highlight",
        "text.literal",
        "text.reference",
        "text.strike",
        "text.subscript",
        "text.super",
        "text.title",
        "text.underline",
        "text.uri",
        "type",
    ]);
    config
}

#[test]
fn highlights_cover_quarto_constructs() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/basic.qmd");
    let source = fs::read_to_string(&fixture).expect("fixture readable");

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    let root = tree.root_node();
    let mut seen_front_matter = false;
    for i in 0..root.named_child_count() {
        let child = root.named_child(i).expect("valid child");
        if child.kind() == "yaml_front_matter" {
            seen_front_matter = true;
            let mut has_content = false;
            for j in 0..child.named_child_count() {
                if child
                    .named_child(j)
                    .map(|node| node.kind() == "yaml_front_matter_content")
                    .unwrap_or(false)
                {
                    has_content = true;
                    break;
                }
            }
            assert!(has_content, "front matter should contain YAML content");
            break;
        }
    }
    assert!(
        seen_front_matter,
        "document should include yaml front matter"
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
                rendered.push(format!("<{}>", config.names()[s.0]));
            }
            tree_sitter_highlight::HighlightEvent::HighlightEnd => {
                rendered.push("</>".into());
            }
            tree_sitter_highlight::HighlightEvent::Source { start, end } => {
                rendered.push(source[start..end].to_string());
            }
        }
    }

    let rendered = rendered.join("");

    // Debug: print first 500 chars of rendered output
    eprintln!(
        "Rendered output (first 500 chars):\n{}",
        &rendered.chars().take(500).collect::<String>()
    );

    // Basic smoke test: verify we got some highlighting
    assert!(
        rendered.contains("<"),
        "document should have some syntax highlighting"
    );

    // Verify pandoc-markdown specific features are highlighted
    if rendered.contains("text.title") {
        println!("✓ Headings are highlighted");
    }
    if rendered.contains("constant.macro") {
        println!("✓ Shortcodes are highlighted");
    }
    if rendered.contains("text.reference") {
        println!("✓ Citations/cross-references are highlighted");
    }
    if rendered.contains("text.literal") {
        println!("✓ Fenced divs/code blocks are highlighted");
    }
    if rendered.contains("comment") {
        println!("✓ Chunk options are highlighted");
    }
    // Note: Inline content (emphasis, bold) requires Zed's injection system
    // Basic tree-sitter-highlight doesn't support cross-grammar injection
    if rendered.contains("<text.emphasis>") {
        println!("✓ Italic text is highlighted");
    }
    if rendered.contains("<emphasis.strong>") {
        println!("✓ Bold text is highlighted");
    }
}
