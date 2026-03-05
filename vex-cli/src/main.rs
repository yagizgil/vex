pub mod args;
pub mod compiler;
pub mod metrics;

use crate::args::CliArgs;
use crate::compiler::CompilerApp;
use clap::Parser;

fn main() {
    // 1. Get arguments from the user using clap
    let args = CliArgs::parse();

    // 2. Create the compiler application and run it.
    let mut app = CompilerApp::new(args);
    app.run();
}