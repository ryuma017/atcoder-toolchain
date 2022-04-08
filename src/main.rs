use anyhow::Result;
use clap::{Parser, Subcommand};

use rustcoder::actions::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct RustCoder {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Create workspace for AtCoder
    Init(InitOptions),
    /// Cerate new workspace for specified contest
    Start(StartOptions),
}

#[tokio::main]
async fn main() -> Result<()> {
    let rustcoder = RustCoder::parse();

    match rustcoder.action {
        Action::Init(opt) => init(opt),
        Action::Start(opt) => start(opt).await,
    }
}
