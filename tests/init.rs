use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::Context as _;
use tempfile::TempDir;

const TIMEOUT: Duration = Duration::from_secs(10);
const WORKSPACE_DEFAULT_NAME: &str = "atcoder_workspace";

#[test]
fn init_with_no_args() -> anyhow::Result<()> {
    let tempdir = TempDir::new()?;

    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("init")
        .current_dir(tempdir.path())
        .timeout(TIMEOUT)
        .assert()
        .success();

    let workspace = tempdir.path().join(WORKSPACE_DEFAULT_NAME);

    assert!(workspace.is_dir());
    assert_does_cargo_configure_complete(&workspace);
    assert_does_git_setup_complete(&workspace);

    Ok(())
}

#[test]
fn init_in_an_existing_dir() -> anyhow::Result<()> {
    for d in [".", "some", "./some", "some/dir", "./some/dir"] {
        let tempdir = TempDir::new()?;
        fs::create_dir_all(tempdir.path().join("some/dir"))
            .context("Failed to create temporary directories to test.")?;

        assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .args(&["init", d])
            .current_dir(tempdir.path())
            .timeout(TIMEOUT)
            .assert()
            .success();

        let existing_dir = tempdir.path().join(d);
        assert!(existing_dir.is_dir());
        assert_does_cargo_configure_complete(&existing_dir);
        assert_does_git_setup_complete(&existing_dir);
    }

    Ok(())
}

fn assert_does_cargo_configure_complete(workspace: &Path) {
    assert!(workspace.join(".cargo").is_dir());
    assert!(workspace
        .join(".cargo")
        .join("config")
        .with_extension("toml")
        .exists())
}

fn assert_does_git_setup_complete(workspace: &Path) {
    assert!(workspace.join(".git").is_dir());
    assert!(workspace.join(".gitignore").exists());
}
