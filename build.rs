use std::path::{Path, PathBuf};
use std::process::Command;

const REPO_URL: &str = "https://github.com/ck37/tree-sitter-pandoc-markdown";
const COMMIT: &str = "f2e5718a1b2190cf59dd22d9a97fc9b7329a25b6";

// tree-sitter-quarto for automated highlighting tests
const QUARTO_REPO_URL: &str = "https://github.com/ck37/tree-sitter-quarto";
const QUARTO_COMMIT: &str = "4012bc7d9930654c81f1ade1d2070e0b951aa689";

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

    // NOTE: Patching code no longer needed as of commit 4f184e2 (zed-compatible-scopes branch)
    // The grammar now uses Zed-compatible scopes directly, so no runtime patching needed
    // See docs/scope-naming-decision.md for full rationale and migration path
    //
    // Keeping patching code below commented out for reference:
    /*
    let inline_highlights_path = pandoc_dir
        .join("tree-sitter-pandoc-markdown-inline")
        .join("queries")
        .join("highlights.scm");
    if inline_highlights_path.exists() {
        let highlights = std::fs::read_to_string(&inline_highlights_path)
            .expect("failed to read inline highlights.scm");

        let patched = highlights
            .replace("@markup.italic", "@text.emphasis")
            .replace("@markup.bold", "@emphasis.strong")
            .replace("@markup.raw.inline", "@text.literal")
            .replace("@markup.link.label", "@text.reference")
            .replace("@markup.link.url", "@text.uri")
            .replace("@markup.reference.citation", "@text.reference")
            .replace("@markup.reference.cross_ref", "@text.reference")
            .replace("@markup.reference.footnote", "@text.reference")
            .replace("@markup.strikethrough", "@text.strike")
            .replace("@markup.highlight", "@text.highlight")
            .replace("@markup.subscript", "@text.subscript")
            .replace("@markup.superscript", "@text.super")
            .replace("@markup.underline", "@text.underline")
            .replace("@markup.math.inline", "@string")
            .replace("@attribute", "@property");

        std::fs::write(&inline_highlights_path, patched)
            .expect("failed to patch inline highlights.scm");
        eprintln!("Patched inline grammar highlights.scm to use Zed-compatible scopes");
    }
    */

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

    // Also compile tree-sitter-quarto for heading_highlighting tests
    compile_quarto_grammar(&dir);
}

fn compile_quarto_grammar(dir: &Path) {
    // Build.rs clones tree-sitter-quarto to grammars/quarto/ for test compilation
    // Zed will also clone its own copy when installing the extension
    let quarto_dir = dir.join("quarto");
    let src_dir = quarto_dir.join("src");

    if !quarto_dir.join(".git").exists() {
        eprintln!("Cloning tree-sitter-quarto repository...");
        let status = Command::new("git")
            .args(["clone", QUARTO_REPO_URL, quarto_dir.to_str().unwrap()])
            .status()
            .expect("failed to spawn git clone for quarto");

        if !status.success() {
            panic!("Could not clone tree-sitter-quarto repository");
        }
    }

    eprintln!("Ensuring tree-sitter-quarto is at commit {QUARTO_COMMIT}...");

    let _fetch_status = Command::new("git")
        .current_dir(&quarto_dir)
        .args(["fetch", "origin"])
        .status();

    let _clean_status = Command::new("git")
        .current_dir(&quarto_dir)
        .args(["reset", "--hard"])
        .status();

    let checkout_status = Command::new("git")
        .current_dir(&quarto_dir)
        .args(["checkout", QUARTO_COMMIT])
        .status()
        .expect("failed to checkout quarto grammar commit");

    if !checkout_status.success() {
        panic!("Could not checkout tree-sitter-quarto commit {QUARTO_COMMIT}");
    }

    if src_dir.join("parser.c").exists() {
        // Patch the grammar's queries to use Zed-compatible scopes for tests
        // This simulates what we want Zed to do: load our extension's queries instead of grammar's
        patch_quarto_queries(&quarto_dir);

        eprintln!("Compiling tree-sitter-quarto grammar...");

        cc::Build::new()
            .include(&src_dir)
            .file(src_dir.join("parser.c"))
            .file(src_dir.join("scanner.c"))
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-function")
            .flag_if_supported("-Wno-unused-const-variable")
            .compile("tree-sitter-quarto");

        println!("cargo:rerun-if-changed={}", src_dir.display());
    } else {
        eprintln!("Warning: tree-sitter-quarto grammar source not found, heading tests may fail");
    }
}

fn patch_quarto_queries(quarto_dir: &Path) {
    let highlights_dest = quarto_dir.join("queries").join("highlights.scm");
    let injections_dest = quarto_dir.join("queries").join("injections.scm");

    if !highlights_dest.exists() {
        eprintln!("Warning: Could not find tree-sitter-quarto queries/highlights.scm to patch");
        return;
    }

    eprintln!("Copying extension's Zed-compatible queries over grammar's queries...");

    // Copy our extension's queries (which use Zed-compatible scopes) over the grammar's queries
    // This simulates what we want Zed to do
    let extension_highlights = std::path::PathBuf::from("languages/quarto/highlights.scm");
    let extension_injections = std::path::PathBuf::from("languages/quarto/injections.scm");

    if extension_highlights.exists() {
        std::fs::copy(&extension_highlights, &highlights_dest)
            .expect("failed to copy extension highlights.scm");
        eprintln!(
            "✓ Copied languages/quarto/highlights.scm → grammars/quarto/queries/highlights.scm"
        );
    } else {
        eprintln!("Warning: Could not find languages/quarto/highlights.scm");
    }

    if extension_injections.exists() {
        std::fs::copy(&extension_injections, &injections_dest)
            .expect("failed to copy extension injections.scm");
        eprintln!(
            "✓ Copied languages/quarto/injections.scm → grammars/quarto/queries/injections.scm"
        );
    }

    eprintln!("✓ Grammar now uses extension's Zed-compatible queries for testing");
}
