// src/main.rs
/*
 * Main executable for EmbeddedEngine
 */

use clap::Parser;
use embeddedengine::{Result, run};

#[derive(Parser)]
#[command(version, about = "EmbeddedEngine - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
