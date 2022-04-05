use anyhow::Result;
use clap::{Parser, Subcommand};

use rustcoder::actions::{InitOptions, StartOptions};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct RustCoder {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Debug sample case
    Dbg,
    /// Create workspace for AtCoder
    Init(InitOptions),
    /// Login to AtCoder
    Login,
    /// Cerate new workspace for specified contest
    Start(StartOptions),
    /// Submit solution source code
    Submit,
    /// Test sample case
    Test,
}

#[tokio::main]
async fn main() -> Result<()> {
    let rustcoder = RustCoder::parse();

    use rustcoder::actions::*;
    match rustcoder.action {
        Action::Dbg => Ok(()),          //dbg(),
        Action::Init(opt) => init(opt), //init(),
        Action::Login => Ok(()),        //login(),
        Action::Start(opt) => start(opt).await,
        Action::Submit => Ok(()), //submit(),
        Action::Test => Ok(()),   //test(),
    }
}
