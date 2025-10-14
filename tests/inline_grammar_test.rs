use tree_sitter::{Language, Parser};

// Link to the compiled pandoc-markdown-inline grammar
#[link(name = "tree-sitter-pandoc-markdown", kind = "static")]
extern "C" {
    fn tree_sitter_pandoc_markdown_inline() -> Language;
}

fn inline_language() -> Language {
    unsafe { tree_sitter_pandoc_markdown_inline() }
}

#[test]
fn inline_grammar_triple_asterisk_parsing() {
    // Test various combinations of asterisks using the inline grammar directly
    let test_cases = vec![
        (
            "*single italic*",
            "emphasis",
            "should parse single asterisks as emphasis",
        ),
        (
            "**double bold**",
            "strong_emphasis",
            "should parse double asterisks as strong_emphasis",
        ),
        (
            "***triple***",
            "emphasis",
            "should parse triple asterisks (contains nested emphasis/strong)",
        ),
        (
            "****quadruple****",
            "strong_emphasis",
            "should parse quadruple asterisks",
        ),
        (
            "_single underscore_",
            "emphasis",
            "should parse single underscores as emphasis",
        ),
        (
            "__double underscore__",
            "strong_emphasis",
            "should parse double underscores as strong_emphasis",
        ),
        (
            "___triple underscore___",
            "emphasis",
            "should parse triple underscores",
        ),
    ];

    let mut parser = Parser::new();
    let lang = inline_language();
    parser
        .set_language(&lang)
        .expect("parser loads inline language");

    for (source, expected_node, description) in test_cases {
        eprintln!("\n=== Testing: {} ===", description);
        eprintln!("Source: {:?}", source);

        let tree = parser
            .parse(source.as_bytes(), None)
            .expect("parse succeeds");

        let root = tree.root_node();

        eprintln!("Parse tree: {}", root.to_sexp());

        // Check for errors
        if root.has_error() {
            eprintln!("WARNING: Parse has errors!");
            let mut cursor = tree.walk();
            fn print_errors(cursor: &mut tree_sitter::TreeCursor, source: &str, depth: usize) {
                let node = cursor.node();
                if node.is_error() || node.is_missing() {
                    eprintln!(
                        "{}Error: {} | text: {:?}",
                        "  ".repeat(depth),
                        node.kind(),
                        &source[node.byte_range()]
                    );
                }
                if cursor.goto_first_child() {
                    loop {
                        print_errors(cursor, source, depth + 1);
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                    cursor.goto_parent();
                }
            }
            print_errors(&mut cursor, source, 0);
        }

        // Check that we found the expected node type
        let sexp = root.to_sexp();
        assert!(
            sexp.contains(expected_node),
            "Expected to find '{}' in parse tree for input '{}'\nGot: {}",
            expected_node,
            source,
            sexp
        );

        eprintln!("✓ Test passed!");
    }

    println!("\n=== All inline grammar tests passed! ===");
}

#[test]
fn inline_grammar_triple_asterisk_structure() {
    // Specifically test the structure of triple asterisk parsing
    let source = "***bold and italic***";

    let mut parser = Parser::new();
    let lang = inline_language();
    parser
        .set_language(&lang)
        .expect("parser loads inline language");

    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    let root = tree.root_node();
    let sexp = root.to_sexp();

    eprintln!("\n=== Triple Asterisk Structure Test ===");
    eprintln!("Source: {}", source);
    eprintln!("Parse tree:\n{}", sexp);

    // Should not have errors
    assert!(
        !root.has_error(),
        "Triple asterisks should parse without errors. Got: {}",
        sexp
    );

    // Should contain emphasis (outer) and strong_emphasis (inner) or vice versa
    let has_emphasis = sexp.contains("emphasis");
    let has_strong = sexp.contains("strong_emphasis");

    assert!(
        has_emphasis && has_strong,
        "Triple asterisks should create nested emphasis and strong_emphasis. Got: {}",
        sexp
    );

    // Check the nesting structure
    let mut cursor = tree.walk();
    cursor.goto_first_child(); // Go to document root child

    fn check_nesting(cursor: &mut tree_sitter::TreeCursor) -> bool {
        let node = cursor.node();
        let kind = node.kind();

        if kind == "emphasis" || kind == "strong_emphasis" {
            if cursor.goto_first_child() {
                loop {
                    let child_kind = cursor.node().kind();
                    if (kind == "emphasis" && child_kind == "strong_emphasis")
                        || (kind == "strong_emphasis" && child_kind == "emphasis")
                    {
                        return true;
                    }
                    if check_nesting(cursor) {
                        return true;
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        } else if cursor.goto_first_child() {
            loop {
                if check_nesting(cursor) {
                    return true;
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }

        false
    }

    let found_nested = check_nesting(&mut cursor);

    assert!(
        found_nested,
        "Triple asterisks should create properly nested emphasis/strong_emphasis. Got: {}",
        sexp
    );

    println!("\n✓ Triple asterisk structure test passed!");
    println!("  - No parse errors");
    println!("  - Contains both emphasis and strong_emphasis");
    println!("  - Properly nested structure");
}
