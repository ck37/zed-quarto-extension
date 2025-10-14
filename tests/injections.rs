use std::fs;
use std::path::Path;

#[test]
fn injection_languages_are_valid() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Read injections.scm
    let injections_str = fs::read_to_string(manifest_dir.join("languages/quarto/injections.scm"))
        .expect("injections.scm readable");

    // Read extension manifest
    let manifest_str = fs::read_to_string(manifest_dir.join("extension.toml"))
        .expect("extension manifest readable");
    let manifest: toml::Value = toml::from_str(&manifest_str).expect("manifest parses as TOML");

    // Known built-in Zed grammars that can be injected
    let known_builtin_grammars = [
        "markdown-inline",
        "markdown",
        "yaml",
        "json",
        "toml",
        "python",
        "r",
        "julia",
        "sql",
        "javascript",
        "typescript",
        "css",
        "html",
        "latex",
        "rust",
        "go",
        "bash",
        "c",
        "cpp",
        "ruby",
        "java",
        "lua",
        "vim",
        "xml",
        "regex",
    ];

    // Extract language names from (#set! injection.language "LANGUAGE_NAME") predicates
    // This captures explicit language assignments, not dynamic captures like @injection.language
    let injection_language_pattern =
        regex::Regex::new(r#"#set!\s+injection\.language\s+"([^"]+)""#).expect("regex compiles");

    let mut invalid_languages = Vec::new();

    for captures in injection_language_pattern.captures_iter(&injections_str) {
        let language_name = &captures[1];

        // Check if grammar is defined in extension manifest
        let is_defined_in_manifest = manifest
            .get("grammars")
            .and_then(|value| value.as_table())
            .map(|table| table.contains_key(language_name))
            .unwrap_or(false);

        let is_builtin = known_builtin_grammars.contains(&language_name);

        // Check if it's a language defined in this extension's languages/ directory
        let is_extension_language = manifest
            .get("languages")
            .and_then(|value| value.as_array())
            .map(|langs| {
                langs.iter().any(|lang_path| {
                    if let Some(lang_path_str) = lang_path.as_str() {
                        let config_path = manifest_dir.join(lang_path_str).join("config.toml");
                        if let Ok(config_str) = fs::read_to_string(&config_path) {
                            if let Ok(config) = toml::from_str::<toml::Value>(&config_str) {
                                return config
                                    .get("name")
                                    .and_then(|n| n.as_str())
                                    .map(|n| n == language_name)
                                    .unwrap_or(false);
                            }
                        }
                    }
                    false
                })
            })
            .unwrap_or(false);

        if !is_defined_in_manifest && !is_builtin && !is_extension_language {
            invalid_languages.push(language_name.to_string());
        }
    }

    assert!(
        invalid_languages.is_empty(),
        "The following injection languages are not valid: {:?}\n\
         This means the injection will fail at runtime.\n\
         Languages must be either:\n\
         - Defined in extension.toml [grammars] section\n\
         - A known built-in Zed grammar\n\
         - A language defined in this extension's languages/ directory",
        invalid_languages
    );
}

#[test]
fn injections_file_exists_and_parses() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let injections_path = manifest_dir.join("languages/quarto/injections.scm");

    assert!(
        injections_path.exists(),
        "injections.scm must exist at languages/quarto/injections.scm"
    );

    let injections_str =
        fs::read_to_string(&injections_path).expect("injections.scm should be readable");

    assert!(
        !injections_str.is_empty(),
        "injections.scm should not be empty"
    );

    // Basic validation that it looks like a tree-sitter query file
    assert!(
        injections_str.contains("@injection.content")
            || injections_str.contains("@injection.language"),
        "injections.scm should contain tree-sitter injection predicates"
    );
}

#[test]
fn pandoc_inline_grammar_is_defined() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Read injections.scm
    let injections_str = fs::read_to_string(manifest_dir.join("languages/quarto/injections.scm"))
        .expect("injections.scm readable");

    // If this branch uses pandoc_markdown_inline, verify it's defined in extension.toml
    if injections_str.contains("pandoc_markdown_inline") {
        let manifest_str = fs::read_to_string(manifest_dir.join("extension.toml"))
            .expect("extension manifest readable");
        let manifest: toml::Value = toml::from_str(&manifest_str).expect("manifest parses as TOML");

        let has_pandoc_inline = manifest
            .get("grammars")
            .and_then(|value| value.as_table())
            .map(|table| table.contains_key("pandoc_markdown_inline"))
            .unwrap_or(false);

        assert!(
            has_pandoc_inline,
            "injections.scm references 'pandoc_markdown_inline' but it's not defined in extension.toml.\n\
             This injection requires the Zed fix for extension-to-extension grammar injection.\n\
             Add pandoc_markdown_inline to [grammars] section in extension.toml."
        );
    }
}
