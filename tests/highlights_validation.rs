/// Test that validates highlights.scm queries compile correctly
/// using tree-sitter CLI to prevent regressions where invalid node types
/// or syntax errors break syntax highlighting in Zed.
use assert_cmd::Command;
use std::path::PathBuf;
use tempfile::TempDir;

/// Creates a test .qmd file with various Quarto syntax
fn create_test_file(dir: &TempDir) -> PathBuf {
    let test_file = dir.path().join("test.qmd");
    std::fs::write(&test_file, r#"---
title: "Test Document"
---

## Test Heading

This is **bold** and *italic* text.

```python
print("hello")
```

```{python}
x = 1
```
"#).expect("Failed to write test file");
    test_file
}

#[test]
fn test_highlights_query_compiles() {
    // Create temp directory and test file
    let temp_dir = TempDir::new().unwrap();
    let test_file = create_test_file(&temp_dir);

    // Get paths
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let highlights_path = manifest_dir.join("languages/quarto/highlights.scm");
    let grammar_dir = manifest_dir.join("../../tmp/tree-sitter-quarto");

    // Expand the grammar path to absolute
    let grammar_dir = std::fs::canonicalize(&grammar_dir)
        .unwrap_or_else(|_| {
            // Fallback to /tmp if relative path doesn't work
            PathBuf::from("/tmp/tree-sitter-quarto")
        });

    // Run tree-sitter query command to validate the highlights.scm compiles
    // against the actual grammar
    let mut cmd = Command::cargo_bin("tree-sitter")
        .unwrap_or_else(|_| Command::new("tree-sitter"));

    let assert = cmd
        .arg("query")
        .arg(&highlights_path)
        .arg(&test_file)
        .current_dir(&grammar_dir)
        .assert();

    // The command should succeed (exit code 0)
    // If the query has syntax errors or invalid node types, it will fail
    assert.success();
}

#[test]
fn test_highlights_query_produces_captures() {
    // Create temp directory and test file
    let temp_dir = TempDir::new().unwrap();
    let test_file = create_test_file(&temp_dir);

    // Get paths
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let highlights_path = manifest_dir.join("languages/quarto/highlights.scm");
    let grammar_dir = std::fs::canonicalize(manifest_dir.join("../../tmp/tree-sitter-quarto"))
        .unwrap_or_else(|_| PathBuf::from("/tmp/tree-sitter-quarto"));

    // Run tree-sitter query and capture output
    let mut cmd = Command::cargo_bin("tree-sitter")
        .unwrap_or_else(|_| Command::new("tree-sitter"));

    let output = cmd
        .arg("query")
        .arg(&highlights_path)
        .arg(&test_file)
        .current_dir(&grammar_dir)
        .output()
        .expect("Failed to run tree-sitter query");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify we get expected captures for basic syntax
    assert!(stdout.contains("punctuation.special"), "Should capture heading markers");
    assert!(stdout.contains("text.title"), "Should capture headings");
    assert!(stdout.contains("emphasis.strong"), "Should capture bold text");
    assert!(stdout.contains("text.emphasis"), "Should capture italic text");
    assert!(stdout.contains("text.literal"), "Should capture code blocks");
    assert!(stdout.contains("function.builtin"), "Should capture language names in executable cells");
}
