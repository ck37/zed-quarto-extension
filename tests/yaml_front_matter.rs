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
fn yaml_front_matter_is_parsed() {
    let source = r#"---
title: "Test Document"
author: "Test Author"
date: "2024-01-01"
format: html
---

# Content

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
        "\n=== YAML FRONT MATTER PARSE TREE ===\n{}\n",
        tree.root_node().to_sexp()
    );

    let root = tree.root_node();

    // Verify we have a yaml_front_matter node
    let yaml_front_matter = root.child(0).expect("should have first child");

    assert_eq!(
        yaml_front_matter.kind(),
        "yaml_front_matter",
        "First node should be yaml_front_matter"
    );

    // Verify it has start delimiter
    let has_start = yaml_front_matter.child_by_field_name("start").is_some();
    assert!(has_start, "yaml_front_matter should have start delimiter");

    // Verify it has content
    let has_content = yaml_front_matter.child_by_field_name("content").is_some();
    assert!(has_content, "yaml_front_matter should have content field");

    // Verify it has close delimiter
    let has_close = yaml_front_matter.child_by_field_name("close").is_some();
    assert!(has_close, "yaml_front_matter should have close delimiter");

    println!("✓ YAML front matter structure is correct");
}

#[test]
fn yaml_content_field_exists_for_injection() {
    let source = r#"---
title: "Complex YAML"
nested:
  key1: value1
  key2: value2
list:
  - item1
  - item2
---

# Content
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    let root = tree.root_node();
    let yaml_front_matter = root.child(0).expect("should have yaml_front_matter");

    // Verify we have a content field for injection
    let content = yaml_front_matter
        .child_by_field_name("content")
        .expect("yaml_front_matter should have content field");

    assert_eq!(
        content.kind(),
        "yaml_content",
        "Content field should be yaml_content node type"
    );

    // Verify the content captures the YAML text (excluding delimiters)
    let content_text = &source[content.byte_range()];
    assert!(
        content_text.contains("title:"),
        "Content should include YAML"
    );
    assert!(
        content_text.contains("nested:"),
        "Content should include nested YAML"
    );
    assert!(
        !content_text.contains("---"),
        "Content should not include delimiters"
    );

    println!("✓ YAML content field is available for injection");
}

#[test]
fn empty_yaml_front_matter() {
    let source = r#"---
---

# Content
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    let root = tree.root_node();
    let yaml_front_matter = root.child(0).expect("should have yaml_front_matter");

    assert_eq!(yaml_front_matter.kind(), "yaml_front_matter");

    // Empty YAML should still have the structure
    let has_content = yaml_front_matter.child_by_field_name("content").is_some();

    // Empty YAML may or may not have a content field - just verify it parses
    println!(
        "✓ Empty YAML front matter parses: content field present = {}",
        has_content
    );
}
