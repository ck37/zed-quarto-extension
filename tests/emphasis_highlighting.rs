use std::fs;
use std::path::Path;

use tree_sitter::{Language, Parser};
use tree_sitter_highlight::{HighlightConfiguration, Highlighter};

// Link to the compiled pandoc-markdown grammar
#[link(name = "tree-sitter-pandoc-markdown", kind = "static")]
extern "C" {
    fn tree_sitter_pandoc_markdown() -> Language;
}

fn language() -> Language {
    unsafe { tree_sitter_pandoc_markdown() }
}

fn highlight_configuration() -> HighlightConfiguration {
    let highlight_query = include_str!("../languages/quarto/highlights.scm");
    let injection_query = include_str!("../languages/quarto/injections.scm");
    let locals_query = "";

    let mut config = HighlightConfiguration::new(
        language(),
        "quarto",
        highlight_query,
        injection_query,
        locals_query,
    )
    .expect("valid highlight configuration with pandoc-markdown grammar");

    config.configure(&[
        "annotation",
        "attribute",
        "comment.documentation",
        "function.macro",
        "markup",
        "punctuation.special",
        "punctuation.delimiter",
        "string.escape",
        "string.special.symbol",
        "text.emphasis",
        "emphasis.strong",
        "text.literal",
        "text.reference",
        "text.title",
        "text.uri",
    ]);
    config
}

#[test]
fn emphasis_variations_are_highlighted() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/emphasis-variations.qmd");
    let source = fs::read_to_string(&fixture).expect("fixture readable");

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    let config = highlight_configuration();
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(&config, source.as_bytes(), None, |_| None)
        .expect("highlighting succeeds");

    let mut rendered = Vec::new();
    for event in events {
        match event.expect("valid event") {
            tree_sitter_highlight::HighlightEvent::HighlightStart(s) => {
                rendered.push(format!("<{}>", config.names()[s.0]));
            }
            tree_sitter_highlight::HighlightEvent::HighlightEnd => {
                rendered.push("</>".into());
            }
            tree_sitter_highlight::HighlightEvent::Source { start, end } => {
                rendered.push(source[start..end].to_string());
            }
        }
    }

    let rendered = rendered.join("");

    // Debug: print full rendered output
    eprintln!("\n=== EMPHASIS VARIATIONS RENDERED OUTPUT ===\n{}\n", rendered);

    // Test single asterisks (italic)
    assert!(
        rendered.contains("<text.emphasis>") && rendered.contains("italic text"),
        "Single asterisks should create italic text (text.emphasis)"
    );
    println!("✓ Single asterisks highlighted as italic");

    // Test double asterisks (bold)
    assert!(
        rendered.contains("<emphasis.strong>") && rendered.contains("bold text"),
        "Double asterisks should create bold text (emphasis.strong)"
    );
    println!("✓ Double asterisks highlighted as bold");

    // Test triple asterisks (bold + italic)
    let has_triple_content = rendered.contains("bold and italic");
    let has_triple_highlighting = rendered.contains("<text.emphasis>")
        || rendered.contains("<emphasis.strong>");

    assert!(
        has_triple_content && has_triple_highlighting,
        "Triple asterisks should be highlighted with emphasis or strong scopes"
    );
    println!("✓ Triple asterisks highlighted (nested emphasis/strong)");

    // Test underscore variants
    assert!(
        rendered.contains("single underscore italic"),
        "Single underscores should be present"
    );
    println!("✓ Single underscore variant present");

    assert!(
        rendered.contains("double underscore bold"),
        "Double underscores should be present"
    );
    println!("✓ Double underscore variant present");

    assert!(
        rendered.contains("triple underscore bold+italic"),
        "Triple underscores should be present"
    );
    println!("✓ Triple underscore variant present");

    // Overall check that we have both emphasis types
    assert!(
        rendered.contains("<text.emphasis>") && rendered.contains("<emphasis.strong>"),
        "Document should have both italic (text.emphasis) and bold (emphasis.strong) highlighting"
    );
    println!("✓ Both emphasis types present in document");

    println!("\n=== All emphasis variation tests passed! ===");
}
