use std::fs;
use zed_extension_api as zed;

struct QuartoExtension;

impl zed::Extension for QuartoExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        match command.name.as_str() {
            "quarto-preview" => self.preview_quarto(&args, worktree),
            _ => Err(format!("Unknown command: {}", command.name)),
        }
    }
}

impl QuartoExtension {
    /// Preview the current Quarto document in the default browser
    fn preview_quarto(
        &self,
        args: &[String],
        worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        // 1. Check Quarto CLI availability
        if !self.is_quarto_available() {
            return Err(
                "Quarto CLI not found. Install from https://quarto.org/docs/get-started/"
                    .to_string(),
            );
        }

        // 2. Get current file path
        let file_path = self.get_current_file(args, worktree)?;

        // 3. Validate file extension
        if !file_path.ends_with(".qmd") {
            return Err("Preview only works with .qmd files".to_string());
        }

        // 4. Validate file exists
        if fs::metadata(&file_path).is_err() {
            return Err(format!("File not found: {}", file_path));
        }

        // 5. Execute quarto preview
        let output = std::process::Command::new("quarto")
            .arg("preview")
            .arg(&file_path)
            .output()
            .map_err(|e| format!("Failed to run quarto: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Quarto preview failed: {}", stderr));
        }

        // 6. Return success message
        let filename = std::path::Path::new(&file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&file_path);
        let text = format!("✓ Opening preview for {}", filename);
        Ok(zed::SlashCommandOutput {
            text: text.clone(),
            sections: vec![zed::SlashCommandOutputSection {
                range: (0..text.len()).into(),
                label: "Quarto Preview".to_string(),
            }],
        })
    }

    /// Check if Quarto CLI is available in PATH
    fn is_quarto_available(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            #[cfg(target_family = "unix")]
            {
                std::process::Command::new("which")
                    .arg("quarto")
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            }
            #[cfg(target_family = "windows")]
            {
                std::process::Command::new("where")
                    .arg("quarto")
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            // In WASM, we can't run system commands during build
            // The actual check will happen at runtime in Zed
            false
        }
    }

    /// Get the current file path from arguments or worktree
    fn get_current_file(
        &self,
        args: &[String],
        worktree: Option<&zed::Worktree>,
    ) -> Result<String, String> {
        // Try to get file path from arguments first
        if !args.is_empty() {
            return Ok(args[0].clone());
        }

        // Try to get from worktree root path
        if let Some(wt) = worktree {
            return Ok(wt.root_path());
        }

        Err("No file to preview. Please specify a file path or open a Quarto document.".to_string())
    }
}

zed::register_extension!(QuartoExtension);
