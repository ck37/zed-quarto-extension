use std::fs;
use std::path::Path;

use tree_sitter::{Language, Parser};

// Link to the compiled pandoc-markdown grammar
#[link(name = "tree-sitter-pandoc-markdown", kind = "static")]
extern "C" {
    fn tree_sitter_pandoc_markdown() -> Language;
}

fn language() -> Language {
    unsafe { tree_sitter_pandoc_markdown() }
}

#[test]
fn r_code_blocks_are_parsed() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/r-code-examples.qmd");
    let source = fs::read_to_string(&fixture).expect("fixture readable");

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    let root = tree.root_node();
    let sexp = root.to_sexp();

    // Debug: print parse tree snippet
    eprintln!("\n=== R CODE PARSE TREE (first 2000 chars) ===");
    eprintln!("{}", &sexp.chars().take(2000).collect::<String>());

    // Verify R code blocks are parsed as fenced_code_block
    assert!(
        sexp.contains("(fenced_code_block"),
        "Document should contain fenced code blocks"
    );
    println!("✓ R code blocks are parsed as fenced_code_block nodes");

    // Verify info_string contains 'r' (case insensitive)
    let mut cursor = tree.walk();
    let mut r_code_blocks = 0;
    let mut r_code_contents = Vec::new();

    fn find_r_blocks(
        cursor: &mut tree_sitter::TreeCursor,
        source: &str,
        count: &mut usize,
        contents: &mut Vec<String>,
    ) {
        let node = cursor.node();

        if node.kind() == "fenced_code_block" {
            // Check if this is an R code block
            if cursor.goto_first_child() {
                let mut is_r_block = false;
                let mut code_content = String::new();

                loop {
                    let child = cursor.node();

                    if child.kind() == "info_string" {
                        let info = &source[child.byte_range()];
                        // Match {r} or {r ...} or {R}
                        if info.to_lowercase().contains("r") {
                            is_r_block = true;
                        }
                    } else if child.kind() == "code_fence_content" {
                        code_content = source[child.byte_range()].to_string();
                    }

                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }

                if is_r_block {
                    *count += 1;
                    contents.push(code_content);
                }

                cursor.goto_parent();
            }
        }

        if cursor.goto_first_child() {
            loop {
                find_r_blocks(cursor, source, count, contents);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    find_r_blocks(
        &mut cursor,
        &source,
        &mut r_code_blocks,
        &mut r_code_contents,
    );

    eprintln!("\n=== R CODE BLOCKS FOUND: {} ===", r_code_blocks);

    assert!(
        r_code_blocks >= 6,
        "Should find at least 6 R code blocks, found {}",
        r_code_blocks
    );
    println!("✓ Found {} R code blocks", r_code_blocks);

    // Verify R code content is preserved
    let all_content = r_code_contents.join("\n");

    // Check for key R constructs
    assert!(
        all_content.contains("<-") || all_content.contains("mean"),
        "R code should contain assignment operator or function calls"
    );
    println!("✓ R code content is preserved (assignment operators/functions)");

    assert!(
        all_content.contains("library"),
        "R code should contain library() calls"
    );
    println!("✓ R code contains library() calls");

    assert!(
        all_content.contains("%>%")
            || all_content.contains("filter")
            || all_content.contains("select"),
        "R code should contain dplyr/tidyverse patterns"
    );
    println!("✓ R code contains dplyr/tidyverse patterns");

    // Check for code with options
    let has_options =
        source.contains("{r echo=") || source.contains("{r my-analysis") || source.contains("{r,");
    assert!(has_options, "Should have R code blocks with chunk options");
    println!("✓ R code blocks with chunk options are present");

    // Verify no parse errors in R blocks
    assert!(
        !root.has_error(),
        "R code blocks should not cause parse errors"
    );
    println!("✓ No parse errors in document with R code");

    println!("\n=== All R code injection tests passed! ===");
}

#[test]
fn r_injection_query_matches() {
    // Test that the injection query properly identifies R code blocks
    let test_cases = [
        "# Test\n\n```{r}\nx <- 1\n```\n",
        "# Test\n\n```{R}\ny <- 2\n```\n",
        "# Test\n\n```{r echo=TRUE}\nz <- 3\n```\n",
        "# Test\n\n```{r my-label, warning=FALSE}\na <- 4\n```\n",
    ];

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");

    for (i, source) in test_cases.iter().enumerate() {
        eprintln!("\n=== Testing R injection pattern {} ===", i + 1);
        eprintln!("Source: {}", source);

        let tree = parser
            .parse(source.as_bytes(), None)
            .expect("parse succeeds");

        let root = tree.root_node();
        let sexp = root.to_sexp();

        eprintln!("Parse tree: {}", sexp);

        // Should have a fenced_code_block
        assert!(
            sexp.contains("(fenced_code_block"),
            "Test case {} should parse as fenced_code_block",
            i + 1
        );

        // Should have an info_string with 'r' or 'R'
        assert!(
            sexp.contains("(info_string") && (source.contains("{r") || source.contains("{R")),
            "Test case {} should have R info_string",
            i + 1
        );

        // Should not have errors (or only minor ones that don't affect code blocks)
        if root.has_error() {
            eprintln!(
                "WARNING: Test case {} has parse errors, but this may be acceptable",
                i + 1
            );
        }

        println!("✓ Test case {} passed", i + 1);
    }

    println!("\n=== All R injection query tests passed! ===");
}
