use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn quarto_cli_presence() {
    if which::which("quarto").is_err() {
        eprintln!("skipping: Quarto CLI not installed");
        return;
    }

    Command::new("quarto").arg("--version").assert().success();
}
