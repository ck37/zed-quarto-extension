/// Verify that language query files are compatible with the grammar
///
/// This test ensures that the query files in languages/quarto/ use node types
/// that exist in the tree-sitter-quarto grammar.
///
/// ## Why This Test Exists
///
/// Previously, we copied query files from the main branch (which used
/// tree-sitter-pandoc-markdown node names) into the tree-sitter-quarto migration
/// branch. This caused runtime errors in Zed:
///
/// ```
/// ERROR [language::language_registry] failed to load language Quarto:
/// Error loading highlights query
/// Caused by:
///     Query error at 18:2. Invalid node type fenced_code_block_delimiter
/// ```
///
/// ## The Problem
///
/// tree-sitter-pandoc-markdown and tree-sitter-quarto use different node names:
/// - Pandoc: `fenced_code_block_delimiter`
/// - Quarto: `code_fence_delimiter`
///
/// When extension query files reference non-existent nodes, Zed fails to load
/// the language, resulting in no syntax highlighting.
///
/// ## The Solution
///
/// Query files in `languages/quarto/` must match the grammar specified in
/// `extension.toml`. Since we use tree-sitter-quarto, we must use its queries.
///
/// This test validates that highlights.scm can be parsed with the grammar.
use tree_sitter::{Language, Query};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

#[test]
fn highlights_query_compatible_with_grammar() {
    let language = unsafe { tree_sitter_quarto() };
    let highlights_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("languages/quarto/highlights.scm");

    let highlights =
        std::fs::read_to_string(&highlights_path).expect("Failed to read highlights.scm");

    // Try to parse the query with the grammar
    let result = Query::new(&language, &highlights);

    match result {
        Ok(_) => {
            // Success!
        }
        Err(e) => {
            panic!(
                "highlights.scm contains invalid queries for tree-sitter-quarto grammar:\n\
                 \n\
                 Error: {}\n\
                 \n\
                 This likely means highlights.scm was copied from a different grammar.\n\
                 \n\
                 To fix:\n\
                 1. Get the correct query files from tree-sitter-quarto repository\n\
                 2. Copy from: https://github.com/ck37/tree-sitter-quarto/tree/main/queries\n\
                 3. Match the commit specified in extension.toml\n\
                 \n\
                 Current grammar commit in extension.toml:\n\
                 rev = \"9f7e5d2ef6af2af9dd47b259d9d50fa5d0e18638\"\n\
                 \n\
                 Query files must use tree-sitter-quarto node names, not pandoc-markdown.",
                e
            );
        }
    }
}

#[test]
fn injections_query_compatible_with_grammar() {
    let language = unsafe { tree_sitter_quarto() };
    let injections_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("languages/quarto/injections.scm");

    let injections =
        std::fs::read_to_string(&injections_path).expect("Failed to read injections.scm");

    let result = Query::new(&language, &injections);

    assert!(
        result.is_ok(),
        "injections.scm contains invalid queries for tree-sitter-quarto grammar:\n{:?}\n\
         \nQuery files must match the grammar specified in extension.toml.",
        result.err()
    );
}

#[test]
fn all_scm_files_compatible_with_grammar() {
    let language = unsafe { tree_sitter_quarto() };
    let quarto_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("languages/quarto");

    let query_files = [
        "highlights.scm",
        "injections.scm",
        "folds.scm",
        "indents.scm",
        "locals.scm",
    ];

    // Note: textobjects.scm, outline.scm, and tags.scm are not included because:
    // - textobjects.scm: Not provided by tree-sitter-quarto upstream
    // - outline.scm, tags.scm: Not yet implemented for tree-sitter-quarto

    for filename in query_files {
        let path = quarto_dir.join(filename);
        if !path.exists() {
            continue; // Skip if file doesn't exist (optional files)
        }

        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read {}", filename));

        let result = Query::new(&language, &content);

        assert!(
            result.is_ok(),
            "{} contains invalid queries for tree-sitter-quarto:\n{:?}",
            filename,
            result.err()
        );
    }
}

#[test]
fn query_files_from_correct_source() {
    // Document where query files should come from
    let highlights = std::fs::read_to_string("languages/quarto/highlights.scm")
        .expect("highlights.scm should exist");

    // tree-sitter-quarto includes a header comment explaining Zed scope choices
    assert!(
        highlights.contains("tree-sitter-quarto") || highlights.contains("Quarto"),
        "highlights.scm should be from tree-sitter-quarto, not tree-sitter-pandoc-markdown.\n\
         \n\
         The file header should reference Quarto or tree-sitter-quarto.\n\
         \n\
         To fix: Download query files from:\n\
         https://github.com/ck37/tree-sitter-quarto/tree/9f7e5d2/queries"
    );
}
