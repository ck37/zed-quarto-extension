/// Test to validate that all node types used in highlight queries
/// actually exist in the grammar. This prevents silent highlighting failures
/// when queries reference non-existent node types.
use std::path::Path;
use tree_sitter::{Language, Query};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

#[test]
fn all_query_node_types_exist_in_grammar() {
    let language = unsafe { tree_sitter_quarto() };
    let highlights_query = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("grammars/quarto-vendored/queries/zed/highlights.scm"),
    )
    .expect("Failed to read highlights.scm");

    // Try to compile the query - this will fail if node types don't exist
    let result = Query::new(&language, &highlights_query);

    match result {
        Ok(_) => {
            // Query compiled successfully - all node types are valid
        }
        Err(e) => {
            panic!(
                "highlights.scm contains invalid node types!\n\n\
                Error: {:?}\n\n\
                This usually means the query references node types that don't exist in the grammar.\n\
                Check that:\n\
                1. All node types in captures exist in tree-sitter-quarto grammar\n\
                2. Node names match exactly (case-sensitive)\n\
                3. The grammar version matches what extension.toml expects\n\n\
                To debug:\n\
                - Run: tree-sitter parse <test-file> to see actual AST nodes\n\
                - Run: tree-sitter query grammars/quarto-vendored/queries/zed/highlights.scm to validate query",
                e
            );
        }
    }
}

#[test]
fn emphasis_and_strong_nodes_exist() {
    let language = unsafe { tree_sitter_quarto() };

    // Test that basic inline formatting nodes exist
    let test_query = r#"
        (emphasis) @test
        (strong_emphasis) @test
        (atx_heading) @test
        (text) @test
    "#;

    let result = Query::new(&language, test_query);

    assert!(
        result.is_ok(),
        "Grammar is missing basic inline formatting node types: emphasis, strong_emphasis, or atx_heading"
    );
}

#[test]
fn grammar_parses_basic_inline_content() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&language)
        .expect("Failed to set language");

    // Test emphasis
    let tree = parser
        .parse("*italic text*", None)
        .expect("Failed to parse emphasis");
    let root = tree.root_node();

    // Should have an emphasis node somewhere in the tree
    let has_emphasis = check_for_node_type(&root, "emphasis");
    assert!(
        has_emphasis,
        "Grammar failed to parse emphasis (*italic*). Tree:\n{}",
        root.to_sexp()
    );

    // Test strong emphasis
    let tree = parser
        .parse("**bold text**", None)
        .expect("Failed to parse strong");
    let root = tree.root_node();

    let has_strong = check_for_node_type(&root, "strong_emphasis");
    assert!(
        has_strong,
        "Grammar failed to parse strong emphasis (**bold**). Tree:\n{}",
        root.to_sexp()
    );

    // Test heading
    let tree = parser
        .parse("# Heading\n", None)
        .expect("Failed to parse heading");
    let root = tree.root_node();

    let has_heading = check_for_node_type(&root, "atx_heading");
    assert!(
        has_heading,
        "Grammar failed to parse ATX heading. Tree:\n{}",
        root.to_sexp()
    );

    // Verify text nodes exist inside emphasis
    let tree = parser
        .parse("*italic text*", None)
        .expect("Failed to parse");
    let root = tree.root_node();

    let has_text_in_emphasis = check_for_nested_node(&root, "emphasis", "text");
    assert!(
        has_text_in_emphasis,
        "Grammar doesn't create text nodes inside emphasis. Tree:\n{}",
        root.to_sexp()
    );
}

// Helper function to recursively check for a node type
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

// Helper function to check if a node of type parent_type contains a child of type child_type
fn check_for_nested_node(node: &tree_sitter::Node, parent_type: &str, child_type: &str) -> bool {
    if node.kind() == parent_type {
        // Found parent, check children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == child_type {
                return true;
            }
        }
    }

    // Recursively search children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if check_for_nested_node(&child, parent_type, child_type) {
            return true;
        }
    }

    false
}

#[test]
fn document_unsupported_pandoc_features() {
    // This test documents Pandoc features that are NOT yet supported
    // by tree-sitter-quarto. If these start passing, we can add queries for them!

    let language = unsafe { tree_sitter_quarto() };
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&language)
        .expect("Failed to set language");

    // Test strikethrough (NOT supported yet)
    let tree = parser.parse("~~strikethrough~~", None).unwrap();
    let has_strikethrough = check_for_node_type(&tree.root_node(), "strikethrough");

    if has_strikethrough {
        println!("✨ Grammar now supports strikethrough! Add highlight queries for it.");
    } else {
        println!("ℹ️  Strikethrough (~~text~~) not yet supported by grammar - parses as text");
    }

    // Test highlight (NOT supported yet)
    let tree = parser.parse("==highlight==", None).unwrap();
    let has_highlight = check_for_node_type(&tree.root_node(), "highlight");

    if has_highlight {
        println!("✨ Grammar now supports highlight! Add highlight queries for it.");
    } else {
        println!("ℹ️  Highlight (==text==) not yet supported by grammar - parses as text");
    }

    // Test subscript (NOT supported yet)
    let tree = parser.parse("H~2~O", None).unwrap();
    let has_subscript = check_for_node_type(&tree.root_node(), "subscript");

    if has_subscript {
        println!("✨ Grammar now supports subscript! Add highlight queries for it.");
    } else {
        println!("ℹ️  Subscript (~text~) not yet supported by grammar - parses as text");
    }

    // Test superscript (NOT supported yet)
    let tree = parser.parse("x^2^", None).unwrap();
    let has_superscript = check_for_node_type(&tree.root_node(), "superscript");

    if has_superscript {
        println!("✨ Grammar now supports superscript! Add highlight queries for it.");
    } else {
        println!("ℹ️  Superscript (^text^) not yet supported by grammar - parses as text");
    }

    // This test always passes - it just documents the current state
    // When grammar adds support, the println messages will tell us
}
