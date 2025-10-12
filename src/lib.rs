use zed_extension_api as zed;

struct QuartoExtension;

impl zed::Extension for QuartoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        // Use worktree.which() to find quarto in PATH
        let command = worktree.which("quarto").ok_or_else(|| {
            "Quarto CLI not found in PATH. Install Quarto or ensure it's in PATH before launching Zed.".to_string()
        })?;

        Ok(zed::Command {
            command,
            args: vec!["language-server".into()],
            env: worktree.shell_env(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>, String> {
        Ok(None)
    }
}

zed::register_extension!(QuartoExtension);
