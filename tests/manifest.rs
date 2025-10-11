use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn language_grammar_is_valid() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let config_str = fs::read_to_string(manifest_dir.join("languages/quarto/config.toml"))
        .expect("language config readable");
    let config: toml::Value = toml::from_str(&config_str).expect("language config parses as TOML");
    let language_grammar = config
        .get("grammar")
        .and_then(|value| value.as_str())
        .expect("language config must declare a grammar");

    let manifest_str = fs::read_to_string(manifest_dir.join("extension.toml"))
        .expect("extension manifest readable");
    let manifest: toml::Value = toml::from_str(&manifest_str).expect("manifest parses as TOML");

    // Check if grammar is defined in extension manifest or is a known built-in
    let known_builtin_grammars = ["markdown", "json", "toml", "yaml"];
    let is_defined_in_manifest = manifest
        .get("grammars")
        .and_then(|value| value.as_table())
        .map(|table| table.contains_key(language_grammar))
        .unwrap_or(false);

    let is_builtin = known_builtin_grammars.contains(&language_grammar);

    assert!(
        is_defined_in_manifest || is_builtin,
        "grammar '{}' must be either defined in extension.toml or be a known Zed built-in grammar",
        language_grammar
    );
}

#[test]
fn wasm_extension_builds_successfully() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let wasm_target_dir = manifest_dir.join("target/wasm-install-test");

    let status = Command::new("cargo")
        .current_dir(manifest_dir)
        .env("CARGO_TARGET_DIR", &wasm_target_dir)
        .args(["build", "--release", "--target", "wasm32-wasip2"])
        .status()
        .expect("failed to invoke cargo build for wasm32-wasip2");

    assert!(
        status.success(),
        "cargo build --target wasm32-wasip2 failed with status: {status}"
    );
}
