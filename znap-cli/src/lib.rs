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
        /// Address that will be used for the server once running.
        #[clap(short, long, default_value = "127.0.0.1")]
        address: String,
        /// Port that wuill be used for the server once running.
        #[clap(short, long, default_value = "3000")]
        port: u16,
        /// Protocol that wuill be used for the server once running.
        #[clap(long, default_value = "http")]
        protocol: String,
    },
    /// Serves all collections from the workspace
    Serve {
        /// Address that will be used for the server once running.
        #[clap(short, long, default_value = "127.0.0.1")]
        address: String,
        /// Port that wuill be used for the server once running.
        #[clap(short, long, default_value = "3000")]
        port: u16,
        /// Protocol that wuill be used for the server once running.
        #[clap(long, default_value = "http")]
        protocol: String,
    },
    /// Runs the test suite for the workspace
    Test {
        /// Address that will be used for the server once running.
        #[clap(short, long, default_value = "127.0.0.1")]
        address: String,
        /// Port that wuill be used for the server once running.
        #[clap(short, long, default_value = "3000")]
        port: u16,
        /// Protocol that wuill be used for the server once running.
        #[clap(long, default_value = "http")]
        protocol: String,
    },
    /// Deploys a workspace using shuttle
    Deploy {
        /// The name of the project in shuttle
        name: String,
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
        Command::Build {
            name,
            address,
            port,
            protocol,
        } => Ok(commands::build::run(&name, &address, port, protocol)),
        Command::Serve {
            address,
            port,
            protocol,
        } => Ok(commands::serve::run(&address, port, protocol)),
        Command::Test {
            address,
            port,
            protocol,
        } => Ok(commands::test::run(&address, port, protocol)),
        Command::Clean => Ok(commands::clean::run()),
        Command::Init { name, dry_run } => Ok(commands::init::run(&name, &dry_run)),
        Command::New { name, dry_run } => Ok(commands::new::run(&name, &dry_run)),
        Command::Deploy { name } => Ok(commands::deploy::run(name)),
    }
}

pub fn entry(opts: Opts) -> Result<()> {
    process_command(opts)
}
