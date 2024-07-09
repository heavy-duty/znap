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
    /// Build a collection from the workspace
    Build {
        /// The name of the collection
        name: String,
    },
    /// Serves all collections from the workspace
    Serve {
        /// The name of the collection
        name: String,
        /// Address that will be used for the server once running.
        #[clap(short, long)]
        address: Option<String>,
        /// Port that wuill be used for the server once running.
        #[clap(short, long)]
        port: Option<u16>,
        /// Protocol that wuill be used for the server once running.
        #[clap(long)]
        protocol: Option<String>,
    },
    /// Runs the test suite for the workspace
    Test,
    /// Deploys a workspace using shuttle
    Deploy {
        /// The name of the collection
        name: String,
        /// The name of the project in shuttle
        project: String,
    },
    /// Cleans all the temp files
    Clean,
    /// Initializes a new workspace
    Init {
        /// The name of the workspace
        name: String,
        /// Skip writing the files.
        #[clap(short, long)]
        dry_run: bool,
    },
    /// Create a new collection in the workspace
    New {
        name: String,
        /// Skip writing the files.
        #[clap(short, long)]
        dry_run: bool,
    },
}

fn process_command(opts: Opts) -> Result<()> {
    match &opts.command {
        Command::Build { name } => {
            commands::build::run(name);
            Ok(())
        },
        Command::Serve {
            name,
            address,
            port,
            protocol,
        } => {
            commands::serve::run(name, address, port, protocol);
            Ok(())
        },
        Command::Test => {
            commands::test::run();
            Ok(())
        },
        Command::Clean => {
            commands::clean::run();
            Ok(())
        },
        Command::Init { name, dry_run } => {
            commands::init::run(name, dry_run);
            Ok(())
        },
        Command::New { name, dry_run } => {
            commands::new::run(name, dry_run);
            Ok(())
        },
        Command::Deploy { name, project } => {
            commands::deploy::run(name, project);
            Ok(())
        },
    }
}

pub fn entry(opts: Opts) -> Result<()> {
    process_command(opts)
}
