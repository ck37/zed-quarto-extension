/// Tests for inline content highlighting within emphasis, strong, links, etc.
///
/// This test validates that the grammar provides the necessary node types
/// for highlighting content separately from delimiters. This is critical
/// for proper syntax highlighting in editors.
///
/// ## Why This Test Exists
///
/// When we tried to add highlighting for text content inside bold/italic,
/// we discovered that some node types we expected (like `link_text_delimiter`)
/// didn't actually exist in the grammar. This test prevents that issue by
/// validating the actual node structure.
///
/// ## What We're Testing
///
/// 1. Emphasis and strong emphasis have delimiter nodes
/// 2. Emphasis and strong emphasis contain text nodes
/// 3. Links have link_text and link_destination nodes
/// 4. Images have image_alt and image_source nodes
/// 5. Headings contain inline content with nested formatting
use tree_sitter::{Language, Parser, Query};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

#[test]
fn emphasis_has_delimiter_and_text_nodes() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "*italic text*\n";
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    // Should have emphasis_delimiter nodes
    let has_delimiter = check_for_node_type(&root, "emphasis_delimiter");
    assert!(
        has_delimiter,
        "Grammar doesn't create emphasis_delimiter nodes. Tree:\n{}",
        root.to_sexp()
    );

    // Should have text nodes inside emphasis
    let has_text = check_for_nested_node(&root, "emphasis", "text");
    assert!(
        has_text,
        "Grammar doesn't create text nodes inside emphasis. Tree:\n{}",
        root.to_sexp()
    );
}

#[test]
fn strong_emphasis_has_delimiter_and_text_nodes() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "**bold text**\n";
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    // Should have strong_emphasis_delimiter nodes
    let has_delimiter = check_for_node_type(&root, "strong_emphasis_delimiter");
    assert!(
        has_delimiter,
        "Grammar doesn't create strong_emphasis_delimiter nodes. Tree:\n{}",
        root.to_sexp()
    );

    // Should have text nodes inside strong_emphasis
    let has_text = check_for_nested_node(&root, "strong_emphasis", "text");
    assert!(
        has_text,
        "Grammar doesn't create text nodes inside strong_emphasis. Tree:\n{}",
        root.to_sexp()
    );
}

#[test]
fn link_has_text_and_destination_nodes() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "This is a [link](https://example.com) in text.\n";
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    // Should have link_text node
    let has_link_text = check_for_node_type(&root, "link_text");
    assert!(
        has_link_text,
        "Grammar doesn't create link_text nodes. Tree:\n{}",
        root.to_sexp()
    );

    // Should have link_destination node
    let has_link_dest = check_for_node_type(&root, "link_destination");
    assert!(
        has_link_dest,
        "Grammar doesn't create link_destination nodes. Tree:\n{}",
        root.to_sexp()
    );
}

#[test]
fn image_has_alt_and_source_nodes() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "![alt text](image.png)\n";
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    // Should have image_alt node
    let has_image_alt = check_for_node_type(&root, "image_alt");
    assert!(
        has_image_alt,
        "Grammar doesn't create image_alt nodes. Tree:\n{}",
        root.to_sexp()
    );

    // Should have image_source node
    let has_image_src = check_for_node_type(&root, "image_source");
    assert!(
        has_image_src,
        "Grammar doesn't create image_source nodes. Tree:\n{}",
        root.to_sexp()
    );
}

#[test]
fn heading_contains_inline_with_nested_formatting() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "## Heading with *italic* and **bold**\n";
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    // Should have atx_heading with inline content
    let has_heading = check_for_node_type(&root, "atx_heading");
    assert!(has_heading, "Grammar doesn't parse ATX headings");

    // Should have emphasis inside heading
    let has_emphasis_in_heading = check_for_nested_node(&root, "atx_heading", "emphasis");
    assert!(
        has_emphasis_in_heading,
        "Grammar doesn't parse emphasis inside headings. Tree:\n{}",
        root.to_sexp()
    );

    // Should have strong_emphasis inside heading
    let has_strong_in_heading = check_for_nested_node(&root, "atx_heading", "strong_emphasis");
    assert!(
        has_strong_in_heading,
        "Grammar doesn't parse strong_emphasis inside headings. Tree:\n{}",
        root.to_sexp()
    );
}

#[test]
fn highlights_query_uses_valid_node_types() {
    // This test validates that our highlights.scm only uses node types
    // that actually exist in the grammar
    let language = unsafe { tree_sitter_quarto() };
    let highlights = std::fs::read_to_string("languages/quarto/highlights.scm")
        .expect("Failed to read highlights.scm");

    let result = Query::new(&language, &highlights);

    assert!(
        result.is_ok(),
        "highlights.scm contains invalid node types: {:?}\n\n\
         This usually means we're referencing nodes that don't exist in the grammar.\n\
         Check the grammar's actual node structure with: tree-sitter parse <file>",
        result.err()
    );
}

#[test]
fn emphasis_queries_compile_successfully() {
    // Test that our emphasis queries from highlights.scm can compile
    let language = unsafe { tree_sitter_quarto() };

    // Our emphasis queries from highlights.scm
    let emphasis_queries = r#"
        (emphasis_delimiter) @punctuation.delimiter
        (emphasis (text) @text.emphasis)
        (strong_emphasis_delimiter) @punctuation.delimiter
        (strong_emphasis (text) @emphasis.strong)
    "#;

    let result = Query::new(&language, emphasis_queries);

    assert!(
        result.is_ok(),
        "Failed to compile emphasis queries: {:?}",
        result.err()
    );
}

#[test]
fn link_queries_compile_successfully() {
    // Test that our link queries from highlights.scm can compile
    let language = unsafe { tree_sitter_quarto() };

    // Our link queries from highlights.scm
    let link_queries = r#"
        (link_text) @text.reference
        (link_destination) @text.uri
    "#;

    let result = Query::new(&language, link_queries);

    assert!(
        result.is_ok(),
        "Failed to compile link queries: {:?}",
        result.err()
    );
}

// Helper functions

fn check_for_node_type(node: &tree_sitter::Node, node_type: &str) -> bool {
    if node.kind() == node_type {
        return true;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if check_for_node_type(&child, node_type) {
            return true;
        }
    }

    false
}

fn check_for_nested_node(node: &tree_sitter::Node, parent_type: &str, child_type: &str) -> bool {
    if node.kind() == parent_type {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == child_type {
                return true;
            }
            // Also check grandchildren in case child_type is nested deeper
            if check_for_node_type(&child, child_type) {
                return true;
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if check_for_nested_node(&child, parent_type, child_type) {
            return true;
        }
    }

    false
}
