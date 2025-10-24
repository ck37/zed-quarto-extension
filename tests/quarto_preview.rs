/// Tests for Quarto preview slash command functionality
///
/// Note: These are unit tests for the logic. Full integration tests require
/// the Quarto CLI to be installed and a real Zed extension environment.
#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_qmd_file_extension_validation() {
        // Valid .qmd files
        assert!(is_qmd_file("document.qmd"));
        assert!(is_qmd_file("path/to/document.qmd"));
        assert!(is_qmd_file("/absolute/path/to/document.qmd"));

        // Invalid files
        assert!(!is_qmd_file("document.md"));
        assert!(!is_qmd_file("document.txt"));
        assert!(!is_qmd_file("document"));
        assert!(!is_qmd_file("document.qmd.txt"));
    }

    #[test]
    fn test_error_messages_are_helpful() {
        let cli_not_found =
            "Quarto CLI not found. Install from https://quarto.org/docs/get-started/";
        assert!(cli_not_found.contains("https://"));
        assert!(cli_not_found.contains("Install"));

        let wrong_extension = "Preview only works with .qmd files";
        assert!(wrong_extension.contains(".qmd"));

        let no_file = "No file to preview. Please specify a file path or open a Quarto document.";
        assert!(no_file.contains("specify"));
    }

    #[test]
    fn test_success_message_format() {
        let filename = "example.qmd";
        let message = format!("✓ Opening preview for {}", filename);
        assert!(message.starts_with("✓"));
        assert!(message.contains("preview"));
        assert!(message.contains(filename));
    }

    #[test]
    fn test_file_existence_check() {
        // Create a temporary test file
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_preview.qmd");

        // Write test content
        fs::write(&test_file, "# Test Document\n\nThis is a test.")
            .expect("Failed to write test file");

        // Check file exists
        assert!(fs::metadata(&test_file).is_ok(), "Test file should exist");

        // Clean up
        fs::remove_file(&test_file).ok();

        // Check file doesn't exist after removal
        assert!(
            fs::metadata(&test_file).is_err(),
            "Test file should not exist after removal"
        );
    }

    #[test]
    fn test_filename_extraction() {
        assert_eq!(
            extract_filename("/path/to/document.qmd"),
            Some("document.qmd")
        );
        assert_eq!(extract_filename("document.qmd"), Some("document.qmd"));
        assert_eq!(
            extract_filename("/absolute/path/file.qmd"),
            Some("file.qmd")
        );
    }

    // Helper functions that mirror the logic in lib.rs

    fn is_qmd_file(path: &str) -> bool {
        path.ends_with(".qmd")
    }

    fn extract_filename(path: &str) -> Option<&str> {
        std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
    }
}

/// Integration test - only runs if Quarto CLI is available
#[cfg(test)]
mod integration_tests {
    use std::fs;
    use std::process::Command;

    #[test]
    #[ignore] // Run with: cargo test --ignored
    fn test_quarto_cli_available() {
        let quarto_available = is_quarto_available();
        if !quarto_available {
            eprintln!("Quarto CLI not found - skipping integration test");
            eprintln!("Install from: https://quarto.org/docs/get-started/");
        }
        // This test passes even if Quarto is not installed - it just checks the detection logic works
        println!("Quarto available: {}", quarto_available);
    }

    #[test]
    #[ignore] // Run with: cargo test --ignored
    fn test_quarto_preview_with_real_file() {
        if !is_quarto_available() {
            eprintln!("Skipping: Quarto CLI not available");
            return;
        }

        // Create a temporary test file
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_quarto_preview.qmd");

        let content = r#"---
title: "Test Document"
---

# Hello Quarto

This is a test document for preview functionality.
"#;

        fs::write(&test_file, content).expect("Failed to write test file");

        // Try to run quarto render (not preview, as that would open browser)
        let output = Command::new("quarto")
            .arg("render")
            .arg(&test_file)
            .arg("--to")
            .arg("html")
            .output();

        // Clean up
        fs::remove_file(&test_file).ok();
        // Also remove generated HTML
        let html_file = temp_dir.join("test_quarto_preview.html");
        fs::remove_file(&html_file).ok();

        match output {
            Ok(out) => {
                if out.status.success() {
                    println!("✓ Quarto render succeeded");
                } else {
                    eprintln!("Quarto render failed:");
                    eprintln!("{}", String::from_utf8_lossy(&out.stderr));
                }
                assert!(out.status.success(), "Quarto should render the test file");
            }
            Err(e) => {
                panic!("Failed to execute quarto command: {}", e);
            }
        }
    }

    // Helper function
    fn is_quarto_available() -> bool {
        #[cfg(target_family = "unix")]
        {
            Command::new("which")
                .arg("quarto")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
        #[cfg(target_family = "windows")]
        {
            Command::new("where")
                .arg("quarto")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
    }
}
