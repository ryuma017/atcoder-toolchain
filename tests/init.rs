use std::path::Path;
use std::time::Duration;

use tempfile::TempDir;
// use pretty_assertions::assert_eq;

const TIMEOUT: Duration = Duration::from_secs(10);
const WORKSPACE_DEFAULT_NAME: &str = "atcoder_workspace";

#[test]
fn default() -> anyhow::Result<()> {
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

fn assert_does_cargo_configure_complete(workspace: &Path) {
    assert!(workspace.join(".cargo").is_dir());
    assert!(workspace.join(".cargo").join("config").with_extension("toml").exists())
}

fn assert_does_git_setup_complete(workspace: &Path) {
    assert!(workspace.join(".git").is_dir());
    assert!(workspace.join(".gitignore").exists());
}