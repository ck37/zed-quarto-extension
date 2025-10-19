/// Test to diagnose the adjacent emphasis parsing issue
///
/// Issue: The pattern *italic***bold***italic* breaks syntax highlighting
/// in Zed after this line. This test checks if it's a grammar parsing error.
use tree_sitter::{Language, Parser};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

#[test]
fn test_adjacent_emphasis_parsing() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    // The problematic pattern from inline-highlighting.qmd line 155
    let source =
        "Before line.\n\n*italic***bold***italic*\n\nAfter line with **bold** and *italic*.\n";

    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    // Print the parse tree for debugging
    fn print_tree(node: tree_sitter::Node, source: &str, indent: usize) {
        let indent_str = "  ".repeat(indent);
        let text = node.utf8_text(source.as_bytes()).unwrap();
        let text_preview = if text.len() > 40 {
            format!("{}...", &text[..40])
        } else {
            text.replace('\n', "\\n")
        };

        println!(
            "{}{} [{}..{}] \"{}\"",
            indent_str,
            node.kind(),
            node.start_position().row,
            node.end_position().row,
            text_preview
        );

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                print_tree(child, source, indent + 1);
            }
        }
    }

    println!("\n=== Parse Tree for Adjacent Emphasis ===");
    print_tree(root, source, 0);
    println!("=====================================\n");

    // Check for parse errors
    if root.has_error() {
        println!("⚠️  PARSE ERROR DETECTED\n");

        fn find_errors(node: tree_sitter::Node, source: &str) {
            if node.is_error() {
                let text = node.utf8_text(source.as_bytes()).unwrap_or("<invalid>");
                println!(
                    "  ERROR node at row {}: {:?}",
                    node.start_position().row,
                    text
                );
            }
            if node.is_missing() {
                println!("  MISSING node: {}", node.kind());
            }
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    find_errors(child, source);
                }
            }
        }

        find_errors(root, source);

        panic!("Grammar failed to parse adjacent emphasis pattern without errors");
    } else {
        println!("✓ Grammar parsed the pattern without errors");

        // But we should check if the structure makes sense
        // Count how many emphasis/strong_emphasis nodes we got
        let mut emphasis_count = 0;
        let mut strong_count = 0;

        fn count_nodes(node: tree_sitter::Node, emphasis: &mut usize, strong: &mut usize) {
            match node.kind() {
                "emphasis" => *emphasis += 1,
                "strong_emphasis" => *strong += 1,
                _ => {}
            }

            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    count_nodes(child, emphasis, strong);
                }
            }
        }

        count_nodes(root, &mut emphasis_count, &mut strong_count);

        println!(
            "\nFound: {} emphasis nodes, {} strong_emphasis nodes",
            emphasis_count, strong_count
        );

        // For *italic***bold***italic* we expect:
        // Either: 2 emphasis + 1 strong_emphasis (if parsed correctly)
        // Or: Something else (if parsed incorrectly)

        println!(
            "\nExpected: 2 emphasis + 1 strong_emphasis for the pattern *italic***bold***italic*"
        );

        // Don't fail the test, just document what we found
        // The important thing is whether it has parse errors
    }
}
