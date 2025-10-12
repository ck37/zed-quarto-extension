use std::path::PathBuf;
use std::process::Command;

const REPO_URL: &str = "https://github.com/ck37/tree-sitter-pandoc-markdown";
const COMMIT: &str = "77de308d04c994a2a3fa2056e66fae6a5630fb87";

fn main() {
    // Only compile the grammar for native tests, not for WASM
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("wasm") {
        return;
    }

    let dir = PathBuf::from("grammars");
    let pandoc_dir = dir.join("pandoc_markdown");
    let src_dir = pandoc_dir.join("tree-sitter-pandoc-markdown").join("src");

    std::fs::create_dir_all(&dir).ok();

    let repo_path = pandoc_dir.to_str().expect("path is valid utf-8");
    if !pandoc_dir.join(".git").exists() {
        eprintln!("Cloning pandoc-markdown grammar repository...");
        let status = Command::new("git")
            .args(["clone", REPO_URL, repo_path])
            .status()
            .expect("failed to spawn git clone");

        if !status.success() {
            panic!("Could not clone pandoc-markdown grammar repository");
        }
    }

    eprintln!("Ensuring pandoc-markdown grammar is at commit {COMMIT}...");

    let fetch_status = Command::new("git")
        .current_dir(&pandoc_dir)
        .args(["fetch", "origin"])
        .status()
        .expect("failed to fetch pandoc-markdown grammar");

    if !fetch_status.success() {
        panic!("Could not fetch pandoc-markdown grammar updates");
    }

    // Discard any local changes before checkout
    let _clean_status = Command::new("git")
        .current_dir(&pandoc_dir)
        .args(["reset", "--hard"])
        .status();

    let checkout_status = Command::new("git")
        .current_dir(&pandoc_dir)
        .args(["checkout", COMMIT])
        .status()
        .expect("failed to checkout pandoc-markdown grammar commit");

    if !checkout_status.success() {
        panic!("Could not checkout pandoc-markdown grammar commit {COMMIT}");
    }

    let reset_status = Command::new("git")
        .current_dir(&pandoc_dir)
        .args(["reset", "--hard", COMMIT])
        .status()
        .expect("failed to reset pandoc-markdown grammar");

    if !reset_status.success() {
        panic!("Could not reset pandoc-markdown grammar to commit {COMMIT}");
    }

    // Patch Cargo.toml to add edition if missing
    let cargo_toml_path = pandoc_dir.join("Cargo.toml");
    if cargo_toml_path.exists() {
        let cargo_toml =
            std::fs::read_to_string(&cargo_toml_path).expect("failed to read grammar Cargo.toml");
        if !cargo_toml.contains("edition = ") {
            // Insert edition after version line
            let patched = cargo_toml.replace(
                "version = \"0.1.0\"\nauthors",
                "version = \"0.1.0\"\nedition = \"2021\"\nauthors",
            );
            std::fs::write(&cargo_toml_path, patched).expect("failed to patch grammar Cargo.toml");
            eprintln!("Patched pandoc-markdown Cargo.toml to add edition = \"2021\"");
        }
    }

    let sync_status = Command::new("git")
        .current_dir(&pandoc_dir)
        .args(["submodule", "sync", "--recursive"])
        .status()
        .expect("failed to sync pandoc-markdown submodules");

    if !sync_status.success() {
        panic!("Could not sync pandoc-markdown grammar submodules");
    }

    let configure_status = Command::new("git")
        .current_dir(&pandoc_dir)
        .args([
            "config",
            "submodule.tree-sitter-markdown.url",
            "https://github.com/tree-sitter-grammars/tree-sitter-markdown.git",
        ])
        .status()
        .expect("failed to configure pandoc-markdown submodule url");

    if !configure_status.success() {
        panic!("Could not configure pandoc-markdown submodule url");
    }

    let update_status = Command::new("git")
        .current_dir(&pandoc_dir)
        .args(["submodule", "update", "--init", "--recursive"])
        .status()
        .expect("failed to update pandoc-markdown submodules after configuration");

    if !update_status.success() {
        panic!("Could not update pandoc-markdown grammar submodules after configuration");
    }

    // Compile both grammars together into one library for easier linking
    let inline_src_dir = pandoc_dir
        .join("tree-sitter-pandoc-markdown-inline")
        .join("src");

    if src_dir.join("parser.c").exists() && inline_src_dir.join("parser.c").exists() {
        eprintln!("Compiling pandoc-markdown grammars (block + inline)...");

        // Compile both grammars into one library
        // Suppress warnings for unused functions/variables in upstream grammar code
        cc::Build::new()
            .include(&src_dir)
            .include(&inline_src_dir)
            .file(src_dir.join("parser.c"))
            .file(src_dir.join("scanner.c"))
            .file(inline_src_dir.join("parser.c"))
            .file(inline_src_dir.join("scanner.c"))
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-function")
            .flag_if_supported("-Wno-unused-const-variable")
            .compile("tree-sitter-pandoc-markdown");

        println!("cargo:rerun-if-changed={}", src_dir.display());
        println!("cargo:rerun-if-changed={}", inline_src_dir.display());
    } else {
        panic!("Pandoc-markdown grammar source not found. Run: git clone --depth=1 --branch=feat/phase-1-pandoc-grammar https://github.com/ck37/tree-sitter-pandoc-markdown grammars/pandoc_markdown");
    }
}
