use tree_sitter::{Language, Parser};
use tree_sitter_highlight::{HighlightConfiguration, Highlighter};

#[link(name = "tree-sitter-quarto", kind = "static")]
extern "C" {
    fn tree_sitter_quarto() -> Language;
}

fn language() -> Language {
    unsafe { tree_sitter_quarto() }
}

fn highlight_configuration() -> HighlightConfiguration {
    let highlight_query = include_str!("../grammars/quarto-vendored/queries/zed/highlights.scm");
    let injection_query = include_str!("../grammars/quarto-vendored/queries/injections.scm");
    let locals_query = "";

    let mut config = HighlightConfiguration::new(
        language(),
        "quarto",
        highlight_query,
        injection_query,
        locals_query,
    )
    .expect("valid highlight configuration");

    let scope_names: Vec<String> = config.names().iter().map(|s| s.to_string()).collect();
    let scope_names_refs: Vec<&str> = scope_names.iter().map(|s| s.as_str()).collect();
    config.configure(&scope_names_refs);

    config
}

#[test]
fn executable_code_cell_is_highlighted() {
    let source = r#"```{python}
print("Hello from Python")
```

```{r}
summary(data)
```
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).expect("parser loads language");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("parse succeeds");

    eprintln!("\n=== PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

    let config = highlight_configuration();
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(&config, source.as_bytes(), None, |_| None)
        .expect("highlighting succeeds");

    let mut rendered = Vec::new();
    for event in events {
        match event.expect("valid event") {
            tree_sitter_highlight::HighlightEvent::HighlightStart(s) => {
                let scope_name = config.names()[s.0];
                rendered.push(format!("<{}>", scope_name));
            }
            tree_sitter_highlight::HighlightEvent::HighlightEnd => {
                rendered.push("</>".into());
            }
            tree_sitter_highlight::HighlightEvent::Source { start, end } => {
                rendered.push(source[start..end].to_string());
            }
        }
    }

    let rendered_output = rendered.join("");
    eprintln!("\n=== RENDERED OUTPUT ===\n{}\n", rendered_output);

    // Check that code fence delimiters are highlighted
    assert!(
        rendered_output.contains("<punctuation.delimiter>```</>"),
        "Code fence delimiters should be highlighted"
    );

    // Check that language names are highlighted as function.builtin
    assert!(
        rendered_output.contains("<function.builtin>python</>"),
        "Python language name should be highlighted"
    );
    assert!(
        rendered_output.contains("<function.builtin>r</>"),
        "R language name should be highlighted"
    );

    println!("✓ Executable code cells are properly highlighted");
}

#[test]
fn code_cell_with_chunk_options() {
    let source = r#"```{python}
#| label: fig-plot
#| echo: false
#| fig-cap: "My plot"

import matplotlib.pyplot as plt
plt.plot([1, 2, 3])
```
"#;

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!("\n=== PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

    let config = highlight_configuration();
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(&config, source.as_bytes(), None, |_| None)
        .unwrap();

    let mut rendered = Vec::new();
    for event in events {
        match event.unwrap() {
            tree_sitter_highlight::HighlightEvent::HighlightStart(s) => {
                let scope_name = config.names()[s.0];
                rendered.push(format!("<{}>", scope_name));
            }
            tree_sitter_highlight::HighlightEvent::HighlightEnd => {
                rendered.push("</>".into());
            }
            tree_sitter_highlight::HighlightEvent::Source { start, end } => {
                rendered.push(source[start..end].to_string());
            }
        }
    }

    let rendered_output = rendered.join("");
    eprintln!("\n=== RENDERED OUTPUT ===\n{}\n", rendered_output);

    // Check that #| prefix is highlighted
    assert!(
        rendered_output.contains("<punctuation.special>#|</>"),
        "Chunk option prefix should be highlighted"
    );

    // Check that chunk option keys are highlighted
    assert!(
        rendered_output.contains("<property>label</>")
            || rendered_output.contains("<property>echo</>")
            || rendered_output.contains("<property>fig-cap</>"),
        "Chunk option keys should be highlighted as properties"
    );

    // Check that chunk option values are highlighted
    assert!(
        rendered_output.contains("<string>fig-plot</>")
            || rendered_output.contains("<string>false</>")
            || rendered_output.contains(r#"<string>"My plot"</>"#),
        "Chunk option values should be highlighted as strings"
    );

    println!("✓ Chunk options are properly highlighted");
}

#[test]
fn inline_code_cell_is_highlighted() {
    let source = "The result is `{python} 2 + 2` and `{r} sum(1:10)`.\n";

    let mut parser = Parser::new();
    let lang = language();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source.as_bytes(), None).unwrap();

    eprintln!("\n=== PARSE TREE ===\n{}\n", tree.root_node().to_sexp());

    let config = highlight_configuration();
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(&config, source.as_bytes(), None, |_| None)
        .unwrap();

    let mut rendered = Vec::new();
    for event in events {
        match event.unwrap() {
            tree_sitter_highlight::HighlightEvent::HighlightStart(s) => {
                let scope_name = config.names()[s.0];
                rendered.push(format!("<{}>", scope_name));
            }
            tree_sitter_highlight::HighlightEvent::HighlightEnd => {
                rendered.push("</>".into());
            }
            tree_sitter_highlight::HighlightEvent::Source { start, end } => {
                rendered.push(source[start..end].to_string());
            }
        }
    }

    let rendered_output = rendered.join("");
    eprintln!("\n=== RENDERED OUTPUT ===\n{}\n", rendered_output);

    // Check that inline cell delimiters are highlighted
    assert!(
        rendered_output.contains("<punctuation.bracket>`</>")
            || rendered_output.contains("<punctuation.bracket>{</>"),
        "Inline cell delimiters should be highlighted"
    );

    // Check that language names in inline cells are highlighted
    assert!(
        rendered_output.contains("<function.builtin>python</>")
            || rendered_output.contains("<function.builtin>r</>"),
        "Language names in inline cells should be highlighted"
    );

    println!("✓ Inline code cells are properly highlighted");
}
