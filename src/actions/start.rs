use std::{fs, path::Path, process::Command};

use anyhow::{bail, Ok, Result};
use clap::Parser;

use crate::atcoder::AtCoderScraper;

// use crate::atcoder;

#[derive(Parser)]
pub struct StartOptions {
    contest_name: String,
}

pub async fn start(opt: StartOptions) -> Result<()> {
    let task_ids = {
        let scraper = AtCoderScraper::new();
        scraper.task_ids(&opt.contest_name).await?
    };

    let project_dir = Path::new(&opt.contest_name);
    if project_dir.is_dir() {
        bail!("`{}` is already exists", project_dir.display())
    }

    // run `cargo new <contest-name>`
    let is_succeed = Command::new("cargo")
        .arg("new")
        .arg(&opt.contest_name)
        .status()?
        .success();
    if !is_succeed {
        bail!("Failed to create cargo project: {}", &opt.contest_name)
    }

    // remove `src/main.rs`
    fs::remove_file(project_dir.join("src").join("main.rs"))?;
    // create `src/bin`
    fs::create_dir(project_dir.join("src").join("bin"))?;

    for task_id in task_ids {
        fs::File::create(
            project_dir
                .join("src")
                .join("bin")
                .join(task_id)
                .with_extension("rs"),
        )?;
    }

    println!("Create {} project!", &opt.contest_name);

    Ok(())
}
