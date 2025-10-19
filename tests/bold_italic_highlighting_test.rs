/// Test that bold and italic text highlighting actually works
///
/// This test validates that our highlights.scm queries successfully capture
/// bold and italic content with the tree-sitter-quarto grammar.
use tree_sitter::{Language, Parser, Query, QueryCursor, StreamingIteratorMut};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

#[test]
fn bold_text_is_highlighted() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "This has **bold text** in it.\n";
    let tree = parser.parse(source, None).unwrap();

    // Load our actual highlights.scm queries
    let highlights = std::fs::read_to_string("languages/quarto/highlights.scm")
        .expect("Failed to read highlights.scm");

    let query = Query::new(&language, &highlights).expect("Failed to compile highlights.scm");

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    // Collect all captures
    let mut bold_text_captured = false;
    let mut delimiter_captured = false;

    while let Some(m) = matches.next_mut() {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node_text = capture.node.utf8_text(source.as_bytes()).unwrap();

            // Check if we captured the bold text
            if capture_name == "emphasis.strong" && node_text == "bold text" {
                bold_text_captured = true;
            }

            // Check if we captured the delimiters
            if capture_name == "punctuation.delimiter.emphasis" && (node_text == "**") {
                delimiter_captured = true;
            }
        }
    }

    assert!(
        bold_text_captured,
        "highlights.scm should capture bold text content with @emphasis.strong"
    );
    assert!(
        delimiter_captured,
        "highlights.scm should capture ** delimiters with @punctuation.delimiter.emphasis"
    );
}

#[test]
fn italic_text_is_highlighted() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "This has *italic text* in it.\n";
    let tree = parser.parse(source, None).unwrap();

    // Load our actual highlights.scm queries
    let highlights = std::fs::read_to_string("languages/quarto/highlights.scm")
        .expect("Failed to read highlights.scm");

    let query = Query::new(&language, &highlights).expect("Failed to compile highlights.scm");

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    // Collect all captures
    let mut italic_text_captured = false;
    let mut delimiter_captured = false;

    while let Some(m) = matches.next_mut() {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node_text = capture.node.utf8_text(source.as_bytes()).unwrap();

            // Check if we captured the italic text
            if capture_name == "emphasis" && node_text == "italic text" {
                italic_text_captured = true;
            }

            // Check if we captured the delimiters
            if capture_name == "punctuation.delimiter.emphasis" && (node_text == "*") {
                delimiter_captured = true;
            }
        }
    }

    assert!(
        italic_text_captured,
        "highlights.scm should capture italic text content with @emphasis"
    );
    assert!(
        delimiter_captured,
        "highlights.scm should capture * delimiters with @punctuation.delimiter.emphasis"
    );
}

#[test]
fn heading_with_bold_and_italic() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "## Heading with *italic* and **bold**\n";
    let tree = parser.parse(source, None).unwrap();

    let highlights = std::fs::read_to_string("languages/quarto/highlights.scm")
        .expect("Failed to read highlights.scm");

    let query = Query::new(&language, &highlights).expect("Failed to compile highlights.scm");

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    let mut found_heading = false;
    let mut found_italic = false;
    let mut found_bold = false;

    while let Some(m) = matches.next_mut() {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node_text = capture.node.utf8_text(source.as_bytes()).unwrap();

            if capture_name == "text.title" {
                found_heading = true;
            }
            if capture_name == "emphasis" && node_text == "italic" {
                found_italic = true;
            }
            if capture_name == "emphasis.strong" && node_text == "bold" {
                found_bold = true;
            }
        }
    }

    assert!(found_heading, "Should capture heading content");
    assert!(found_italic, "Should capture italic text inside heading");
    assert!(found_bold, "Should capture bold text inside heading");
}

#[test]
fn link_text_is_highlighted() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "Check out [this link](https://example.com) for more.\n";
    let tree = parser.parse(source, None).unwrap();

    let highlights = std::fs::read_to_string("languages/quarto/highlights.scm")
        .expect("Failed to read highlights.scm");

    let query = Query::new(&language, &highlights).expect("Failed to compile highlights.scm");

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    let mut link_text_captured = false;
    let mut link_url_captured = false;

    while let Some(m) = matches.next_mut() {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node_text = capture.node.utf8_text(source.as_bytes()).unwrap();

            if capture_name == "link_text" && node_text.contains("this link") {
                link_text_captured = true;
            }
            if capture_name == "link_uri" && node_text.contains("example.com") {
                link_url_captured = true;
            }
        }
    }

    assert!(link_text_captured, "Should capture link text");
    assert!(link_url_captured, "Should capture link URL");
}

#[test]
fn diagnostic_print_all_captures() {
    // This test prints all captures for debugging
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "Test **bold** and *italic* and [link](url).\n";
    let tree = parser.parse(source, None).unwrap();

    let highlights = std::fs::read_to_string("languages/quarto/highlights.scm")
        .expect("Failed to read highlights.scm");

    let query = Query::new(&language, &highlights).expect("Failed to compile highlights.scm");

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    println!("\n=== Highlight Captures ===");
    while let Some(m) = matches.next_mut() {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node_text = capture.node.utf8_text(source.as_bytes()).unwrap();
            let node_kind = capture.node.kind();

            println!("  @{:<25} {:?} ({})", capture_name, node_text, node_kind);
        }
    }
    println!("======================\n");
}
