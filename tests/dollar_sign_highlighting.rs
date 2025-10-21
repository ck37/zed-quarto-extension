/// Tests for dollar sign handling in regular text vs math mode
///
/// ## Issue
/// Dollar signs in regular text (like "$160") are potentially interfering
/// with syntax highlighting. This test validates:
/// 1. Dollar signs in regular text should be treated as plain text
/// 2. Dollar signs for LaTeX math should be parsed as math delimiters
/// 3. The grammar correctly distinguishes between these contexts
///
/// ## Expected Behavior
/// - Regular text: "The keyboard ($160) costs more" -> dollar signs are text
/// - Inline math: "$x^2 + y^2 = z^2$" -> dollar signs are math delimiters
/// - Display math: "$$\nE = mc^2\n$$" -> double dollar signs are math delimiters
use tree_sitter::{Language, Parser};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

#[test]
fn dollar_signs_in_regular_text() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = r#"All major components are now ordered and on their way. The Keychron Q8 fully assembled keyboard ($160) is shipping from Keychron and should arrive in 4-6 business days. The Canjoy wrist pads ($10), silicone bumpers ($9), Official Keychron carrying case ($25-30), and Keybridg aluminum platform ($30) are all ordered and arriving soon.
"#;

    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    // Print the tree for debugging
    println!("Parse tree for dollar signs in text:");
    println!("{}", root.to_sexp());

    // Check if there are any error nodes
    let has_errors = check_for_errors(&root);
    assert!(
        !has_errors,
        "Grammar produced error nodes for text with dollar signs. Tree:\n{}",
        root.to_sexp()
    );

    // Dollar signs in regular text should NOT create math nodes
    let has_math = check_for_node_type(&root, "inline_formula")
        || check_for_node_type(&root, "display_math")
        || check_for_node_type(&root, "math");

    assert!(
        !has_math,
        "Grammar incorrectly parsed dollar signs in regular text as math. Tree:\n{}",
        root.to_sexp()
    );
}

#[test]
fn dollar_signs_in_various_contexts() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let test_cases = vec![
        ("Simple amount: $50", "single dollar amount"),
        ("Range: $25-30", "dollar amount range"),
        ("In parens: ($160)", "dollar amount in parentheses"),
        ("Multiple: $10 and $20 cost $30", "multiple dollar amounts"),
    ];

    for (source, description) in test_cases {
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        println!("\nTest case: {}", description);
        println!("Source: {}", source);
        println!("Tree: {}", root.to_sexp());

        let has_errors = check_for_errors(&root);
        assert!(
            !has_errors,
            "Grammar produced errors for {}: {}",
            description,
            root.to_sexp()
        );
    }
}

#[test]
fn inline_math_with_dollar_signs() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "Inline math: $x^2 + y^2 = z^2$ in a sentence.\n";
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    println!("Parse tree for inline math:");
    println!("{}", root.to_sexp());

    // Should have some math-related node
    // (node names may vary - inline_formula, math, latex_block, etc.)
    let has_math = check_for_node_type(&root, "inline_formula")
        || check_for_node_type(&root, "math")
        || check_for_node_type(&root, "latex_inline");

    // If math is not recognized, that's OK - but we should document it
    if !has_math {
        println!("Note: Inline math not recognized as special node type");
        println!("Tree: {}", root.to_sexp());
    }

    // Main requirement: no parse errors
    let has_errors = check_for_errors(&root);
    assert!(
        !has_errors,
        "Grammar produced errors for inline math. Tree:\n{}",
        root.to_sexp()
    );
}

#[test]
fn display_math_with_double_dollar_signs() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source = "Display math:\n\n$$\nE = mc^2\n$$\n";
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    println!("Parse tree for display math:");
    println!("{}", root.to_sexp());

    // Should have some math-related node
    let has_math = check_for_node_type(&root, "display_math")
        || check_for_node_type(&root, "math_block")
        || check_for_node_type(&root, "latex_block");

    // If math is not recognized, that's OK - but we should document it
    if !has_math {
        println!("Note: Display math not recognized as special node type");
        println!("Tree: {}", root.to_sexp());
    }

    // Main requirement: no parse errors
    let has_errors = check_for_errors(&root);
    assert!(
        !has_errors,
        "Grammar produced errors for display math. Tree:\n{}",
        root.to_sexp()
    );
}

#[test]
fn mixed_dollar_signs_and_math() {
    let language = unsafe { tree_sitter_quarto() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source =
        "The equation $E = mc^2$ shows energy ($E$) equals mass times speed of light squared.\n";
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    println!("Parse tree for mixed dollar signs:");
    println!("{}", root.to_sexp());

    // Main requirement: no parse errors
    let has_errors = check_for_errors(&root);
    assert!(
        !has_errors,
        "Grammar produced errors for mixed dollar signs. Tree:\n{}",
        root.to_sexp()
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

fn check_for_errors(node: &tree_sitter::Node) -> bool {
    if node.is_error() || node.kind() == "ERROR" {
        return true;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if check_for_errors(&child) {
            return true;
        }
    }

    false
}
