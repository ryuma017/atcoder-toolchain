use std::{fs, path::Path, process::Command};

use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser)]
pub struct InitOptions {
    #[clap(default_value_t = String::from("atcoder_workspace"))]
    directory: String,
}

pub fn init(opt: InitOptions) -> Result<()> {
    let dir = Path::new(&opt.directory);

    if !dir.exists() {
        fs::create_dir(dir)?;
    }

    // create `.cargo/config.toml` if it does not exist and write configuration to it
    if !dir.join(".cargo").exists() {
        fs::create_dir(dir.join(".cargo"))?;
        fs::write(
            dir.join(".cargo").join("config").with_extension("toml"),
            "[build]\ntarget-dir = \"target\"",
        )?;
    }

    // initialize empty git repository if it does not exist
    if !dir.join(".git").exists() {
        let is_succeed = Command::new("git")
            .arg("init")
            .arg(dir)
            .arg("--quiet")
            .status()?
            .success();
        if !is_succeed {
            bail!("Failed to initialize empty Git repository")
        }
    }

    // create `.gitignore` if it does not exist
    if !dir.join(".gitignore").exists() {
        fs::write(dir.join(".gitignore"), "/target\nCargo.lock\n/.cargo")?;
    }

    Ok(())
}
