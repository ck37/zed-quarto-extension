/// Tests for indentation query validation
///
/// This test validates that the indents.scm file uses capture names
/// that are recognized by Zed's tree-sitter implementation.
///
/// ## Why This Test Exists
///
/// Zed logs showed a warning:
/// ```
/// WARN [language] unrecognized capture name 'dedent' in Quarto indents
/// TreeSitter query (suppress this warning by prefixing with '_')
/// ```
///
/// This test documents which capture names are valid for indent queries
/// and ensures our queries don't use unrecognized names.
use tree_sitter::{Language, Query};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

#[test]
fn indents_query_is_valid() {
    let language = unsafe { tree_sitter_quarto() };
    let indents = std::fs::read_to_string("languages/quarto/indents.scm")
        .expect("Failed to read indents.scm");

    let result = Query::new(&language, &indents);

    assert!(
        result.is_ok(),
        "indents.scm contains invalid queries: {:?}",
        result.err()
    );
}

#[test]
fn indents_query_uses_recognized_captures() {
    // Document valid capture names for Zed indent queries
    // Based on tree-sitter documentation and Zed's implementation

    let language = unsafe { tree_sitter_quarto() };
    let indents = std::fs::read_to_string("languages/quarto/indents.scm")
        .expect("Failed to read indents.scm");

    let query = Query::new(&language, &indents).expect("Failed to parse indents.scm");

    let capture_names: Vec<_> = query.capture_names().iter().collect();

    // Valid capture names for indent queries (based on Zed's implementation)
    let valid_captures = ["indent", "_indent"];

    // Note: @dedent is NOT recognized by Zed (causes warning)
    // If using unrecognized captures, prefix with _ to suppress warnings

    for capture in &capture_names {
        if !capture.starts_with('_') {
            let capture_str: &str = capture;
            assert!(
                valid_captures.contains(&capture_str),
                "indents.scm uses unrecognized capture name '{}'. \
                 Valid names: {:?}. \
                 To suppress warnings, prefix with '_' (e.g., '_dedent')",
                capture,
                valid_captures
            );
        }
    }
}

#[test]
fn document_indent_capture_behavior() {
    // This test documents what capture names do in indent queries

    println!("\n=== Zed Indent Query Capture Names ===\n");
    println!("Recognized captures:");
    println!("  @indent     - Increase indentation for child nodes");
    println!("  @_indent    - Same as @indent but suppresses unused warning");
    println!();
    println!("Unrecognized captures (will trigger warnings):");
    println!("  @dedent     - NOT recognized by Zed");
    println!("  @outdent    - NOT recognized by Zed");
    println!();
    println!("Workaround: Prefix unrecognized captures with '_' to suppress warnings");
    println!("  @_dedent    - Won't trigger warning (but also won't do anything)");
    println!();
    println!("Note: Zed may have limited indent query support compared to Neovim.");
    println!("See: https://zed.dev/docs/languages for Zed-specific documentation");
}

#[test]
fn indents_query_compiles_without_warnings() {
    // This test will fail if indents.scm has issues that would cause
    // runtime warnings in Zed

    let language = unsafe { tree_sitter_quarto() };
    let indents = std::fs::read_to_string("languages/quarto/indents.scm")
        .expect("Failed to read indents.scm");

    // Check for @dedent which causes warnings
    if indents.contains("@dedent") && !indents.contains("@_dedent") {
        panic!(
            "indents.scm uses @dedent which triggers warnings in Zed.\n\
             Either:\n\
             1. Remove @dedent (Zed may not support it)\n\
             2. Rename to @_dedent (suppresses warning)\n\
             3. Document why the warning is acceptable"
        );
    }

    let result = Query::new(&language, &indents);
    assert!(
        result.is_ok(),
        "Failed to compile indents.scm: {:?}",
        result.err()
    );
}
