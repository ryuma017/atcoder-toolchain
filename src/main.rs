use clap::{Parser, Subcommand};

use atcoder_toolchain::tools;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct AtCoderToolChain {
    #[clap(subcommand)]
    tool: Tool,
}

#[derive(Subcommand)]
enum Tool {
    /// Debug sample case
    Dbg,
    /// Create workspace for AtCoder
    Init,
    /// Login to AtCoder
    Login,
    /// Cerate new workspace for specified contest
    Strat,
    /// Submit solution
    Submit,
    /// Test sample case
    Test,
}

fn main() {
    let actc = AtCoderToolChain::parse();

    match &actc.tool {
        Tool::Dbg => tools::dbg(),
        Tool::Init => tools::init(),
        Tool::Login => tools::login(),
        Tool::Strat => tools::start(),
        Tool::Submit => tools::submit(),
        Tool::Test => tools::test(),
    }
}
