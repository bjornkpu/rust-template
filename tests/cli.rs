// Marks this file as test code, so clippy.toml allows unwrap in its helpers too.
#![cfg(test)]

use std::path::Path;
use std::process::{Command, Output};

/// Runs the binary with its home in `home`, so the user's real config is never touched.
fn run(home: &Path, log: Option<&str>, args: &[&str]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rust-template"));
    cmd.args(args)
        .env("RUST_TEMPLATE_HOME", home)
        .env_remove("RUST_TEMPLATE_LOG");
    if let Some(filter) = log {
        cmd.env("RUST_TEMPLATE_LOG", filter);
    }
    cmd.output().unwrap()
}

/// INV-1: logs never go to stdout.
#[test]
fn stdout_has_no_logs() {
    let home = tempfile::tempdir().unwrap();
    let out = run(home.path(), Some("trace"), &["greet", "BK"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "hello, BK\n");
    let log = std::fs::read_to_string(home.path().join("logs/rust-template.log")).unwrap();
    assert!(log.contains("starting"), "{log}");
}

#[test]
fn no_log_env_writes_no_log_file() {
    let home = tempfile::tempdir().unwrap();
    let out = run(home.path(), None, &["greet"]);
    assert!(out.status.success());
    assert!(!home.path().join("logs").exists());
}

#[test]
fn invalid_log_filter_fails_with_a_message() {
    let home = tempfile::tempdir().unwrap();
    let out = run(home.path(), Some("[[["), &["greet"]);
    assert!(!out.status.success());
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(
        stderr.contains("RUST_TEMPLATE_LOG is not a valid filter"),
        "{stderr}"
    );
    assert!(!stderr.contains("panicked"), "{stderr}");
}

#[test]
fn greet_help() {
    let home = tempfile::tempdir().unwrap();
    let out = run(home.path(), None, &["greet", "--help"]);
    insta::assert_snapshot!(String::from_utf8(out.stdout).unwrap(), @"
    Print a greeting

    Usage: rust-template greet [NAME]

    Arguments:
      [NAME]  Who to greet [default: world]

    Options:
      -h, --help  Print help
    ");
}
