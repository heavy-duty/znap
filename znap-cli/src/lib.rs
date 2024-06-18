use anyhow::Result;
use clap::Parser;
mod commands;
mod template;
mod utils;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
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

fn process_command(opts: Opts) -> Result<()> {
    match &opts.command {
        Command::Build => Ok(commands::build::run()),
        Command::Serve => Ok(commands::serve::run()),
        Command::Test => Ok(commands::test::run()),
        Command::Clean => Ok(commands::clean::run()),
        Command::Init { name } => Ok(commands::init::run(&name)),
        Command::New { name } => Ok(commands::new::run(&name)),
    }
}

pub fn entry(opts: Opts) -> Result<()>  {

    process_command(opts)
}