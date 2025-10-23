use tree_sitter::{Language, Parser};

// External linkage to tree-sitter-quarto grammar
#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

fn language() -> Language {
    unsafe { tree_sitter_quarto() }
}

#[test]
fn heading_with_class_attribute() {
    let source = r#"## Quick Start {.unnumbered}

Some text.
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    // Debug: print the tree structure
    eprintln!(
        "\n=== HEADING WITH CLASS ATTRIBUTE ===\n{}\n",
        tree.root_node().to_sexp()
    );

    let root = tree.root_node();
    let heading = root.child(0).expect("should have heading");

    assert_eq!(heading.kind(), "atx_heading");

    // Verify heading has attributes field
    let attributes = heading
        .child_by_field_name("attributes")
        .expect("heading should have attributes");

    assert_eq!(attributes.kind(), "attribute_list");

    // Find the class attribute
    let mut found_class = false;
    for i in 0..attributes.child_count() {
        if let Some(child) = attributes.child(i) {
            if child.kind() == "attribute_class" {
                let class_text = &source[child.byte_range()];
                assert!(class_text.contains("unnumbered"));
                found_class = true;
            }
        }
    }

    assert!(found_class, "Should have found .unnumbered class attribute");
    println!("✓ Heading with class attribute parses correctly");
}

#[test]
fn heading_with_id_attribute() {
    let source = r#"## Section {#custom-id}

Content.
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    let root = tree.root_node();
    let heading = root.child(0).expect("should have heading");

    // Verify heading has attributes
    let attributes = heading
        .child_by_field_name("attributes")
        .expect("heading should have attributes");

    // Find the id attribute
    let mut found_id = false;
    for i in 0..attributes.child_count() {
        if let Some(child) = attributes.child(i) {
            if child.kind() == "attribute_id" {
                let id_text = &source[child.byte_range()];
                assert!(id_text.contains("custom-id"));
                found_id = true;
            }
        }
    }

    assert!(found_id, "Should have found #custom-id attribute");
    println!("✓ Heading with ID attribute parses correctly");
}

#[test]
fn heading_with_multiple_attributes() {
    let source = r#"## Title {#intro .important .highlight key="value"}

Text.
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    eprintln!(
        "\n=== MULTIPLE ATTRIBUTES ===\n{}\n",
        tree.root_node().to_sexp()
    );

    let root = tree.root_node();
    let heading = root.child(0).expect("should have heading");

    let attributes = heading
        .child_by_field_name("attributes")
        .expect("heading should have attributes");

    // Count different attribute types
    let mut id_count = 0;
    let mut class_count = 0;
    let mut kv_count = 0;

    for i in 0..attributes.child_count() {
        if let Some(child) = attributes.child(i) {
            match child.kind() {
                "attribute_id" => id_count += 1,
                "attribute_class" => class_count += 1,
                "key_value_attribute" => kv_count += 1,
                _ => {}
            }
        }
    }

    assert_eq!(id_count, 1, "Should have 1 ID attribute");
    assert_eq!(class_count, 2, "Should have 2 class attributes");
    assert_eq!(kv_count, 1, "Should have 1 key-value attribute");

    println!("✓ Heading with multiple attributes parses correctly");
}

#[test]
fn heading_without_attributes() {
    let source = r#"## Plain Heading

Text.
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    let root = tree.root_node();
    let heading = root.child(0).expect("should have heading");

    assert_eq!(heading.kind(), "atx_heading");

    // Heading without attributes should not have attributes field
    let has_attributes = heading.child_by_field_name("attributes").is_some();

    assert!(
        !has_attributes,
        "Plain heading should not have attributes field"
    );
    println!("✓ Plain heading without attributes parses correctly");
}

#[test]
fn no_error_nodes_with_attributes() {
    // This was the original bug - headings with attributes created ERROR nodes
    let source = r#"## Quick Start - Where You Are Now {.unnumbered}

This is plain text that should not be highlighted as part of the heading.

## Normal Heading

More text.
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    // Walk the entire tree looking for ERROR nodes
    let mut cursor = tree.walk();
    let mut has_errors = false;

    fn check_for_errors(cursor: &mut tree_sitter::TreeCursor, has_errors: &mut bool) {
        if cursor.node().kind() == "ERROR" {
            *has_errors = true;
        }

        if cursor.goto_first_child() {
            loop {
                check_for_errors(cursor, has_errors);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    check_for_errors(&mut cursor, &mut has_errors);

    assert!(
        !has_errors,
        "Document with heading attributes should not contain ERROR nodes"
    );
    println!("✓ No ERROR nodes with heading attributes - bug is fixed!");
}
