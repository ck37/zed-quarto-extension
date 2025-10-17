/// Analyze link highlighting to understand why links aren't working
use tree_sitter::{Language, Parser};
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

#[test]
fn analyze_link_structure_and_captures() {
    let source = "[link text](https://example.com)\n";

    let language = unsafe { tree_sitter_quarto() };

    // First, see the AST structure
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let tree = parser.parse(source, None).unwrap();

    println!("\n=== LINK AST STRUCTURE ===");
    println!("{}\n", tree.root_node().to_sexp());

    // Check what our queries capture
    let highlights_query = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("grammars/quarto-vendored/queries/zed/highlights.scm"),
    )
    .expect("Failed to read highlights.scm");

    let mut config = HighlightConfiguration::new(language, "quarto", &highlights_query, "", "")
        .expect("Failed to create config");

    let scope_names: Vec<String> = config.names().iter().map(|s| s.to_string()).collect();
    let scope_refs: Vec<&str> = scope_names.iter().map(|s| s.as_str()).collect();
    config.configure(&scope_refs);

    println!("=== AVAILABLE SCOPES ===");
    for (i, name) in config.names().iter().enumerate() {
        println!("[{}] {}", i, name);
    }
    println!();

    // Run highlighting
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(&config, source.as_bytes(), None, |_| None)
        .expect("Highlighting failed");

    println!("=== LINK HIGHLIGHT EVENTS ===");
    let mut depth = 0;
    let mut event_list = vec![];

    for event in events {
        let event = event.expect("Event failed");
        event_list.push(event.clone());

        match &event {
            HighlightEvent::Source { start, end } => {
                let text = &source[*start..*end];
                println!(
                    "{:indent$}Source[{}..{}]: {:?}",
                    "",
                    start,
                    end,
                    text,
                    indent = depth * 2
                );
            }
            HighlightEvent::HighlightStart(scope) => {
                let scope_name = config.names()[scope.0];
                println!(
                    "{:indent$}▼ HighlightStart: @{}",
                    "",
                    scope_name,
                    indent = depth * 2
                );
                depth += 1;
            }
            HighlightEvent::HighlightEnd => {
                depth -= 1;
                println!("{:indent$}▲ HighlightEnd", "", indent = depth * 2);
            }
        }
    }

    // Analysis: Check what's being captured
    println!("\n=== ANALYSIS ===");

    let has_reference_scope = event_list.iter().any(|e| {
        if let HighlightEvent::HighlightStart(scope) = e {
            config.names()[scope.0].contains("reference")
        } else {
            false
        }
    });

    let has_uri_scope = event_list.iter().any(|e| {
        if let HighlightEvent::HighlightStart(scope) = e {
            config.names()[scope.0].contains("uri")
        } else {
            false
        }
    });

    println!("Has @text.reference scope: {}", has_reference_scope);
    println!("Has @text.uri scope: {}", has_uri_scope);

    // Check if "link text" is inside a scope
    let mut link_text_highlighted = false;
    let mut inside_scope = false;

    for event in &event_list {
        match event {
            HighlightEvent::HighlightStart(_) => {
                inside_scope = true;
            }
            HighlightEvent::Source { start, end } => {
                let text = &source[*start..*end];
                if inside_scope && text.contains("link text") {
                    link_text_highlighted = true;
                    println!("✓ Found 'link text' inside a highlight scope!");
                }
            }
            HighlightEvent::HighlightEnd => {
                inside_scope = false;
            }
        }
    }

    if !link_text_highlighted {
        println!("✗ WARNING: 'link text' is NOT inside any highlight scope!");
    }

    // Check if URL is inside a scope
    let mut url_highlighted = false;
    inside_scope = false;

    for event in &event_list {
        match event {
            HighlightEvent::HighlightStart(_) => {
                inside_scope = true;
            }
            HighlightEvent::Source { start, end } => {
                let text = &source[*start..*end];
                if inside_scope && text.contains("example.com") {
                    url_highlighted = true;
                    println!("✓ Found 'example.com' inside a highlight scope!");
                }
            }
            HighlightEvent::HighlightEnd => {
                inside_scope = false;
            }
        }
    }

    if !url_highlighted {
        println!("✗ WARNING: 'example.com' is NOT inside any highlight scope!");
    }

    println!("\n");
}
