use clap::{Parser, Subcommand};
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
    /// Initializes a new workspace
    Init {
        name: String,
    },
    /// Create a new collection in the workspace
    New {
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build) => commands::build::run(),
        Some(Commands::Serve) => commands::serve::run(),
        Some(Commands::Test) => commands::test::run(),
        Some(Commands::Clean) => commands::clean::run(),
        Some(Commands::Init { name }) => commands::init::run(&name),
        Some(Commands::New { name }) => commands::new::run(&name),
        None => {}
    }
}
