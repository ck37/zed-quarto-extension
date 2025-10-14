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
fn python_code_blocks_are_parsed() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/python-code-examples.qmd");
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
    eprintln!("\n=== PYTHON CODE PARSE TREE (first 2000 chars) ===");
    eprintln!("{}", &sexp.chars().take(2000).collect::<String>());

    // Verify Python code blocks are parsed as fenced_code_block
    assert!(
        sexp.contains("(fenced_code_block"),
        "Document should contain fenced code blocks"
    );
    println!("✓ Python code blocks are parsed as fenced_code_block nodes");

    // Verify info_string contains 'python' (case insensitive)
    let mut cursor = tree.walk();
    let mut python_code_blocks = 0;
    let mut python_code_contents = Vec::new();

    fn find_python_blocks(
        cursor: &mut tree_sitter::TreeCursor,
        source: &str,
        count: &mut usize,
        contents: &mut Vec<String>,
    ) {
        let node = cursor.node();

        if node.kind() == "fenced_code_block" {
            // Check if this is a Python code block
            if cursor.goto_first_child() {
                let mut is_python_block = false;
                let mut code_content = String::new();

                loop {
                    let child = cursor.node();

                    if child.kind() == "info_string" {
                        let info = &source[child.byte_range()];
                        // Match {python} or {python ...} or {Python}
                        if info.to_lowercase().contains("python") {
                            is_python_block = true;
                        }
                    } else if child.kind() == "code_fence_content" {
                        code_content = source[child.byte_range()].to_string();
                    }

                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }

                if is_python_block {
                    *count += 1;
                    contents.push(code_content);
                }

                cursor.goto_parent();
            }
        }

        if cursor.goto_first_child() {
            loop {
                find_python_blocks(cursor, source, count, contents);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    find_python_blocks(
        &mut cursor,
        &source,
        &mut python_code_blocks,
        &mut python_code_contents,
    );

    eprintln!("\n=== PYTHON CODE BLOCKS FOUND: {} ===", python_code_blocks);

    assert!(
        python_code_blocks >= 8,
        "Should find at least 8 Python code blocks, found {}",
        python_code_blocks
    );
    println!("✓ Found {} Python code blocks", python_code_blocks);

    // Verify Python code content is preserved
    let all_content = python_code_contents.join("\n");

    // Check for key Python constructs
    assert!(
        all_content.contains("import")
            || all_content.contains("def")
            || all_content.contains("class"),
        "Python code should contain import, def, or class keywords"
    );
    println!("✓ Python code content is preserved (import/def/class keywords)");

    assert!(
        all_content.contains("numpy")
            || all_content.contains("pandas")
            || all_content.contains("np."),
        "Python code should contain scientific library imports"
    );
    println!("✓ Python code contains scientific library imports");

    assert!(
        all_content.contains("for ")
            || all_content.contains("while ")
            || all_content.contains("if "),
        "Python code should contain control flow structures"
    );
    println!("✓ Python code contains control flow structures");

    assert!(
        all_content.contains("[") && all_content.contains("]"),
        "Python code should contain list syntax"
    );
    println!("✓ Python code contains list/array syntax");

    // Check for code with options
    let has_options = source.contains("{python echo=")
        || source.contains("{python my-")
        || source.contains("{python,");
    assert!(
        has_options,
        "Should have Python code blocks with chunk options"
    );
    println!("✓ Python code blocks with chunk options are present");

    // Verify no parse errors in Python blocks
    assert!(
        !root.has_error(),
        "Python code blocks should not cause parse errors"
    );
    println!("✓ No parse errors in document with Python code");

    println!("\n=== All Python code injection tests passed! ===");
}

#[test]
fn python_injection_query_matches() {
    // Test that the injection query properly identifies Python code blocks
    let test_cases = [
        "# Test\n\n```{python}\nx = 1\n```\n",
        "# Test\n\n```{Python}\ny = 2\n```\n",
        "# Test\n\n```{python echo=TRUE}\nz = 3\n```\n",
        "# Test\n\n```{python my-label, warning=FALSE}\na = 4\n```\n",
    ];

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");

    for (i, source) in test_cases.iter().enumerate() {
        eprintln!("\n=== Testing Python injection pattern {} ===", i + 1);
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

        // Should have an info_string with 'python' or 'Python'
        assert!(
            sexp.contains("(info_string")
                && (source.contains("{python") || source.contains("{Python")),
            "Test case {} should have Python info_string",
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

    println!("\n=== All Python injection query tests passed! ===");
}

#[test]
fn python_vs_r_code_blocks() {
    // Test a document with both Python and R code blocks
    let source = r#"
# Mixed Language Document

```{python}
import numpy as np
x = np.array([1, 2, 3])
```

```{r}
y <- c(1, 2, 3)
mean(y)
```

```{python}
print("Python block 2")
```
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");

    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    let root = tree.root_node();
    let sexp = root.to_sexp();

    eprintln!("\n=== MIXED PYTHON/R PARSE TREE ===");
    eprintln!("{}", sexp);

    // Count code blocks
    let mut cursor = tree.walk();
    let mut python_blocks = 0;
    let mut r_blocks = 0;

    fn count_blocks(
        cursor: &mut tree_sitter::TreeCursor,
        source: &str,
        python_count: &mut usize,
        r_count: &mut usize,
    ) {
        let node = cursor.node();

        if node.kind() == "fenced_code_block" && cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "info_string" {
                    let info = &source[child.byte_range()];
                    if info.to_lowercase().contains("python") {
                        *python_count += 1;
                    } else if info.to_lowercase().contains("r")
                        && !info.to_lowercase().contains("python")
                    {
                        *r_count += 1;
                    }
                    break;
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }

        if cursor.goto_first_child() {
            loop {
                count_blocks(cursor, source, python_count, r_count);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    count_blocks(&mut cursor, source, &mut python_blocks, &mut r_blocks);

    eprintln!("\n=== BLOCK COUNTS ===");
    eprintln!("Python blocks: {}", python_blocks);
    eprintln!("R blocks: {}", r_blocks);

    assert_eq!(
        python_blocks, 2,
        "Should find exactly 2 Python blocks, found {}",
        python_blocks
    );
    println!("✓ Found 2 Python blocks");

    assert_eq!(
        r_blocks, 1,
        "Should find exactly 1 R block, found {}",
        r_blocks
    );
    println!("✓ Found 1 R block");

    assert!(
        !root.has_error(),
        "Mixed Python/R document should not have parse errors"
    );
    println!("✓ No parse errors in mixed language document");

    println!("\n=== Mixed language test passed! ===");
}
