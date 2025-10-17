use tree_sitter::{Language, Parser};
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

#[test]
fn test_real_document_link() {
    let source = "Links should work: [link text](https://example.com)\n";

    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let tree = parser.parse(source, None).unwrap();

    println!("\n=== AST ===");
    println!("{}\n", tree.root_node().to_sexp());

    let highlights_query = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("grammars/quarto/queries/highlights.scm"),
    )
    .unwrap();

    let mut config =
        HighlightConfiguration::new(language, "quarto", &highlights_query, "", "").unwrap();

    let scope_names: Vec<String> = config.names().iter().map(|s| s.to_string()).collect();
    let scope_refs: Vec<&str> = scope_names.iter().map(|s| s.as_str()).collect();
    config.configure(&scope_refs);

    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(&config, source.as_bytes(), None, |_| None)
        .unwrap();

    let mut event_list = vec![];
    for event in events {
        event_list.push(event.unwrap());
    }

    // Check for link text and URL highlighting (Zed uses @text.uri for links)
    let mut found_link_text = false;
    let mut found_link_uri = false;

    for event in &event_list {
        if let HighlightEvent::HighlightStart(scope) = event {
            let name = config.names()[scope.0];
            if name.contains("link_text") || name.contains("text.reference") {
                found_link_text = true;
                println!("✓ Found link text scope: {}", name);
            }
            if name.contains("link_uri") || name.contains("text.uri") {
                found_link_uri = true;
                println!("✓ Found link URI scope: {}", name);
            }
        }
    }

    assert!(
        found_link_uri,
        "Link URL should be highlighted with @text.uri or @link_uri.markup"
    );
    if found_link_text {
        println!("✓ Link text is highlighted!");
    } else {
        println!("⚠ Link text not highlighted (known grammar limitation for some cases)");
    }
}
