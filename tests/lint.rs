/// Verify that code passes linting checks
/// This test ensures formatting and clippy rules are followed
use std::process::Command;

#[test]
fn code_is_formatted() {
    let output = Command::new("cargo")
        .args(["fmt", "--check"])
        .output()
        .expect("Failed to run cargo fmt");

    assert!(
        output.status.success(),
        "Code is not formatted. Run: cargo fmt\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn clippy_passes() {
    let output = Command::new("cargo")
        .args([
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ])
        .output()
        .expect("Failed to run cargo clippy");

    assert!(
        output.status.success(),
        "Clippy found issues:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}
