use std::{fs, path::Path};

use clap::Parser;
use anyhow::{Result, bail};

#[derive(Parser)]
pub struct InitOptions {
    #[clap(default_value_t = String::from("atcoder_workspace"))]
    directory: String,
}

pub fn init(opt: InitOptions) -> Result<()> {
    // ここなんか汚い、直したほうが良さそう
    let dir = match &opt.directory[..] {
        "." => Path::new(&opt.directory),
        _ => {
            create_wordspace_dir(&opt.directory)?;
            Path::new(&opt.directory)
        }
    };

    // create `.cargo` directory
    fs::create_dir(dir.join(".cargo"))?;
    // create `.cargo/config.toml`
    fs::write(
        dir.join(".cargo").join("config").with_extension("toml"),
        "[build]\ntarget-dir = \"target\""
    )?;


    Ok(())
}

fn create_wordspace_dir(path: &str) -> Result<()> {
    let dir = Path::new(path);
    if dir.is_dir() {
        bail!("`{}` is already exists", dir.display())
    }

    fs::create_dir(dir)?;

    Ok(())
}
