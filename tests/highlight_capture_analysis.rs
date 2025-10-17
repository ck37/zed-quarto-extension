/// Deep analysis of what highlight captures are actually being generated
/// This will show us if child text nodes are being captured or not
use tree_sitter::{Language, Parser};
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

#[test]
fn analyze_emphasis_captures() {
    let source = "*italic text*\n";

    let language = unsafe { tree_sitter_quarto() };

    // First, let's see the AST structure
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let tree = parser.parse(source, None).unwrap();

    println!("\n=== AST STRUCTURE ===");
    println!("{}\n", tree.root_node().to_sexp());

    // Now let's see what our queries capture
    let highlights_query = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("grammars/quarto/queries/highlights.scm"),
    )
    .expect("Failed to read highlights.scm");

    let mut config = HighlightConfiguration::new(language, "quarto", &highlights_query, "", "")
        .expect("Failed to create config");

    // Use actual scope names from our queries
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

    println!("=== HIGHLIGHT EVENTS ===");
    let mut depth = 0;
    let mut event_list = vec![];

    for event in events {
        let event = event.expect("Event failed");
        event_list.push(event);

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

    // Analysis: Check what we're actually capturing
    println!("\n=== ANALYSIS ===");

    let has_emphasis_scope = event_list.iter().any(|e| {
        if let HighlightEvent::HighlightStart(scope) = e {
            config.names()[scope.0].contains("emphasis")
        } else {
            false
        }
    });

    let has_text_scope = event_list.iter().any(|e| {
        if let HighlightEvent::HighlightStart(scope) = e {
            config.names()[scope.0] == "text"
        } else {
            false
        }
    });

    println!(
        "Has @text.emphasis or @emphasis.strong scope: {}",
        has_emphasis_scope
    );
    println!("Has @text scope: {}", has_text_scope);

    // Count how many times text content is captured
    let mut text_captures = 0;
    let mut in_source = false;
    for event in &event_list {
        match event {
            HighlightEvent::Source { start, end } => {
                let text = &source[*start..*end];
                if text.contains("italic text") {
                    text_captures += 1;
                    in_source = true;
                }
            }
            HighlightEvent::HighlightStart(_) if in_source => {
                // This means the text got a highlight scope applied
            }
            _ => {}
        }
    }

    println!(
        "Times 'italic text' appears in Source events: {}",
        text_captures
    );

    // The key question: Does the text "italic text" appear INSIDE a HighlightStart..HighlightEnd block?
    let mut inside_highlight = false;
    let mut text_highlighted = false;

    for event in &event_list {
        match event {
            HighlightEvent::HighlightStart(scope) => {
                let name = config.names()[scope.0];
                if name.contains("emphasis") || name.contains("strong") {
                    inside_highlight = true;
                    println!("Started {} scope", name);
                }
            }
            HighlightEvent::Source { start, end } => {
                let text = &source[*start..*end];
                if inside_highlight && text.contains("italic text") {
                    text_highlighted = true;
                    println!("✓ Found 'italic text' inside a highlight scope!");
                }
            }
            HighlightEvent::HighlightEnd => {
                if inside_highlight {
                    inside_highlight = false;
                }
            }
        }
    }

    if !text_highlighted {
        println!("✗ WARNING: 'italic text' is NOT inside any emphasis/strong highlight scope!");
        println!("   This means our queries aren't capturing the text content properly.");
    }

    println!("\n");
}

#[test]
fn analyze_heading_captures() {
    let source = "# Heading Text\n";

    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let tree = parser.parse(source, None).unwrap();

    println!("\n=== HEADING AST STRUCTURE ===");
    println!("{}\n", tree.root_node().to_sexp());

    let highlights_query = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("grammars/quarto/queries/highlights.scm"),
    )
    .expect("Failed to read highlights.scm");

    let mut config = HighlightConfiguration::new(language, "quarto", &highlights_query, "", "")
        .expect("Failed to create config");

    let scope_names: Vec<String> = config.names().iter().map(|s| s.to_string()).collect();
    let scope_refs: Vec<&str> = scope_names.iter().map(|s| s.as_str()).collect();
    config.configure(&scope_refs);

    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(&config, source.as_bytes(), None, |_| None)
        .expect("Highlighting failed");

    println!("=== HEADING HIGHLIGHT EVENTS ===");
    let mut depth = 0;
    let mut event_list = vec![];

    for event in events {
        let event = event.expect("Event failed");
        event_list.push(event);

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

    // Check if "Heading Text" is inside a @text.title scope
    let mut inside_title = false;
    let mut heading_text_highlighted = false;

    for event in &event_list {
        match event {
            HighlightEvent::HighlightStart(scope) => {
                let name = config.names()[scope.0];
                if name.contains("title") {
                    inside_title = true;
                    println!("\nStarted title scope: {}", name);
                }
            }
            HighlightEvent::Source { start, end } => {
                let text = &source[*start..*end];
                if inside_title && text.contains("Heading Text") {
                    heading_text_highlighted = true;
                    println!("✓ Found 'Heading Text' inside @text.title scope!");
                }
            }
            HighlightEvent::HighlightEnd => {
                if inside_title {
                    inside_title = false;
                }
            }
        }
    }

    if !heading_text_highlighted {
        println!("✗ WARNING: 'Heading Text' is NOT inside @text.title scope!");
    }

    println!("\n");
}
