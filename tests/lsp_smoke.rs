use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn quarto_cli_presence() {
    // Try to run quarto --version; if it fails, the test passes (it's optional)
    let result = Command::new("quarto").arg("--version").output();

    match result {
        Ok(output) if output.status.success() => {
            // Quarto is installed, verify it works
            Command::new("quarto").arg("--version").assert().success();
        }
        _ => {
            eprintln!("skipping: Quarto CLI not installed");
        }
    }
}
