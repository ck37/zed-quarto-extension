use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Only compile the grammar for native tests, not for WASM
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("wasm") {
        return;
    }

    let dir = PathBuf::from("grammars");
    let pandoc_dir = dir.join("pandoc_markdown");
    let src_dir = pandoc_dir.join("tree-sitter-pandoc-markdown").join("src");

    // Check if we need to fetch the grammar
    if !src_dir.join("parser.c").exists() {
        eprintln!("Fetching pandoc-markdown grammar for tests...");
        std::fs::create_dir_all(&dir).ok();

        // Clone the grammar repository at the specific commit
        let status = Command::new("git")
            .args([
                "clone",
                "https://github.com/ck37/tree-sitter-pandoc-markdown",
                pandoc_dir.to_str().unwrap(),
            ])
            .status();

        if status.is_err() || !status.unwrap().success() {
            panic!("Could not clone pandoc-markdown grammar repository");
        }

        // Checkout the specific commit used in extension.toml
        let checkout_status = Command::new("git")
            .current_dir(&pandoc_dir)
            .args(["checkout", "031ae4a1636d8964955fd4e2f629b2b542b84d01"])
            .status();

        if checkout_status.is_err() || !checkout_status.unwrap().success() {
            panic!("Could not checkout pandoc-markdown grammar commit 031ae4a");
        }
    }

    // Compile the pandoc-markdown grammar if source exists
    if src_dir.join("parser.c").exists() {
        eprintln!("Compiling pandoc-markdown grammar...");

        // Compile the grammar - cc::Build automatically handles linking
        cc::Build::new()
            .include(&src_dir)
            .file(src_dir.join("parser.c"))
            .file(src_dir.join("scanner.c"))
            .compile("tree-sitter-pandoc-markdown");

        println!("cargo:rerun-if-changed={}", src_dir.display());
    } else {
        panic!("Pandoc-markdown grammar source not found. Run: git clone --depth=1 --branch=feat/phase-1-pandoc-grammar https://github.com/ck37/tree-sitter-pandoc-markdown grammars/pandoc_markdown");
    }
}
