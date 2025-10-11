use zed_extension_api as zed;

struct QuartoExtension;

impl zed::Extension for QuartoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        let command = resolve_quarto_command()?;

        Ok(zed::Command {
            command,
            args: vec!["language-server".into()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>, String> {
        Ok(None)
    }
}

zed::register_extension!(QuartoExtension);

#[cfg(not(target_os = "wasi"))]
fn resolve_quarto_command() -> Result<String, String> {
    let path = which::which("quarto").map_err(|_| {
        "Quarto CLI not found in PATH. Install Quarto or set PATH before launching Zed.".to_string()
    })?;
    path.to_str()
        .ok_or_else(|| "Quarto path contains invalid UTF-8".to_string())
        .map(|s| s.to_string())
}

#[cfg(target_os = "wasi")]
fn resolve_quarto_command() -> Result<String, String> {
    Ok("quarto".to_string())
}
