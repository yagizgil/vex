use std::path::PathBuf;
use clap::Parser;

/// Vex Programming Language CLI
#[derive(Parser, Debug)]
#[command(name = "vex-cli", about = "Vex compiler command line interface", version)]
pub struct CliArgs {
    /// The path to the .vx file to compile
    #[arg(required = true)]
    pub file_path: PathBuf,

    /// Start the visual inspector
    #[arg(short = 'i', long)]
    pub inspect: bool,

    /// Start the GUI inspector (instead of TUI)
    #[arg(long)]
    pub gui: bool,

    /// Show performance and memory statistics
    #[arg(short = 's', long)]
    pub stats: bool,
}
