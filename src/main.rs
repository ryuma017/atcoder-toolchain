use clap::{Parser, Subcommand};

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
    let rustcoder = RustCoder::parse();

    use rustcoder::actions::*;
    match rustcoder.action {
        Action::Dbg => dbg(),
        Action::Init => init(),
        Action::Login => login(),
        Action::Strat => start(),
        Action::Submit => submit(),
        Action::Test => test(),
    }
}
