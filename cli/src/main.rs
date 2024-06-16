use clap::{Parser, Subcommand};
use commands::build::run as build_command;
use commands::clean::run as clean_command;
use commands::serve::run as serve_command;
use commands::test::run as test_command;
mod commands;
pub mod template;
pub mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Builds all collections from the workspace
    Build,
    /// Serves all collections from the workspace
    Serve,
    /// Runs the test suite for the workspace
    Test,
    /// Cleans all the temp files
    Clean,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build) => build_command(),
        Some(Commands::Serve) => serve_command(),
        Some(Commands::Test) => test_command(),
        Some(Commands::Clean) => clean_command(),
        None => {}
    }
}
