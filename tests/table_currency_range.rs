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
fn test_table_with_currency_range() {
    let source = include_str!("fixtures/table_currency_range.qmd");

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    let root = tree.root_node();

    // Print the full syntax tree for debugging
    eprintln!("\n=== Full Syntax Tree ===");
    eprintln!("{}", root.to_sexp());

    // Find the table node
    let table_node = find_table_node(&root);

    assert!(table_node.is_some(), "Should parse table node");
    let table = table_node.unwrap();

    eprintln!("\n=== Table Node ===");
    eprintln!("{}", table.to_sexp());

    // Count table rows
    let mut row_count = 0;
    let mut has_header = false;
    let mut data_rows = Vec::new();

    for child_idx in 0..table.child_count() {
        if let Some(child) = table.child(child_idx) {
            match child.kind() {
                "pipe_table_header" => {
                    has_header = true;
                    eprintln!("\n=== Header Row ===");
                    eprintln!("{}", child.to_sexp());
                }
                "pipe_table_delimiter_row" => {
                    eprintln!("\n=== Delimiter Row ===");
                    eprintln!("{}", child.to_sexp());
                }
                "pipe_table_row" => {
                    row_count += 1;
                    data_rows.push(child);
                    eprintln!("\n=== Data Row {} ===", row_count);
                    eprintln!("{}", child.to_sexp());

                    let row_text = &source[child.byte_range()];
                    eprintln!("Row {} text: {:?}", row_count, row_text);
                }
                _ => {}
            }
        }
    }

    assert!(has_header, "Should have header row");
    assert_eq!(
        row_count, 2,
        "Should have exactly 2 data rows (got {})",
        row_count
    );

    // Verify the problematic second row
    if row_count >= 2 {
        let second_row = data_rows[1];
        let row_text = &source[second_row.byte_range()];

        eprintln!("\n=== Analyzing Second Row ===");
        eprintln!("Full row text: {:?}", row_text);
        eprintln!(
            "Row start: {}:{}",
            second_row.start_position().row + 1,
            second_row.start_position().column
        );
        eprintln!(
            "Row end: {}:{}",
            second_row.end_position().row + 1,
            second_row.end_position().column
        );

        // Check if the row contains the currency range
        assert!(
            row_text.contains("$1,360-$1,600"),
            "Second row should contain currency range '$1,360-$1,600', but got: {:?}",
            row_text
        );

        // Check if the row contains the last column
        assert!(
            row_text.contains("Save $250-490"),
            "Second row should contain last column 'Save $250-490', but got: {:?}",
            row_text
        );

        // Verify all 4 columns are present
        let mut cell_count = 0;
        for child_idx in 0..second_row.child_count() {
            if let Some(child) = second_row.child(child_idx) {
                if child.kind() == "table_cell" {
                    cell_count += 1;
                    let cell_text = &source[child.byte_range()];
                    eprintln!("  Cell {}: {:?}", cell_count, cell_text);
                }
            }
        }

        assert_eq!(
            cell_count, 4,
            "Second row should have 4 cells, but got {}",
            cell_count
        );
    }
}

fn find_table_node<'a>(node: &tree_sitter::Node<'a>) -> Option<tree_sitter::Node<'a>> {
    if node.kind() == "pipe_table" {
        return Some(*node);
    }

    for child_idx in 0..node.child_count() {
        if let Some(child) = node.child(child_idx) {
            if let Some(table) = find_table_node(&child) {
                return Some(table);
            }
        }
    }

    None
}
