extern crate assert_cli;
extern crate tempdir;

use std::{env, fs, path::PathBuf};

use assert_cli::Assert;

fn cmd_diff() -> Assert {
    base_cmd().with_args(&["diff"])
}

fn cmd_current() -> Assert {
    base_cmd().with_args(&["current"])
}

#[test]
fn diff_shows_diff() {
    cmd_diff()
        .with_args(&["rand:0.6.0", "rand:0.6.1"])
        .stdout()
        .contains("< version = \"0.6.0\"")
        .unwrap();
}

#[test]
fn diff_reports_error_for_invalid_package_id() {
    cmd_diff()
        .with_args(&["rand:0.6.0", "rand-0.6.1"])
        .fails_with(101)
        .stderr()
        .contains("error: invalid package specification: \"rand-0.6.1\"; expected \"name:x.y.z\"")
        .unwrap();
}

#[test]
fn diff_copies_sources_to_dest() {
    let dir = tempdir::TempDir::new("diff-tests").unwrap();
    cmd_diff()
        .with_args(&["rand:0.6.0", "rand:0.6.1", "--destination"])
        .with_args(&[dir.path()])
        .stdout()
        .is("")
        .unwrap();
    assert!(dir.path().join("rand:0.6.0").exists());
    assert!(dir.path().join("rand:0.6.1").exists());
}

#[test]
fn current_reports_deps() -> std::io::Result<()> {
    let project_dir = tempdir::TempDir::new("temp-project")?;
    let dest = project_dir.path().join("dest");

    fs::write(
        project_dir.path().join("Cargo.toml"),
        r#"
        [package]
        name = "test-pkg"
        version = "0.0.0"

        [dependencies]
        thread_local = "=0.3.6"

        [lib]
        path = "./Cargo.toml"
    "#,
    )?;
    cmd_current()
        .current_dir(project_dir.path())
        .with_args(&["--destination"])
        .with_args(&[&dest.as_path()])
        .stderr()
        .contains("Skipping package `test-pkg`")
        .unwrap();
    assert!(dest.join("thread_local:0.3.6").exists());
    Ok(())
}

// Adapted from
// https://github.com/rust-lang/cargo/blob/485670b3983b52289a2f353d589c57fae2f60f82/tests/testsuite/support/mod.rs#L507
fn target_dir() -> PathBuf {
    env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        }).unwrap()
}

fn cargo_review_deps_exe() -> PathBuf {
    target_dir().join(format!("cargo-review-deps{}", env::consts::EXE_SUFFIX))
}

fn base_cmd() -> Assert {
    Assert::command(&[&cargo_review_deps_exe()]).with_args(&["review-deps"])
}
