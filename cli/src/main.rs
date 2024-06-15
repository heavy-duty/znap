use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::PathBuf, process::Stdio};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    collections: Vec<String>,
}

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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build) => build_all(),
        Some(Commands::Serve) => {
            /*

            Now I should build each collection, then create a router that
            merges them all together and starts a TCP server that serves
            the requests.


            Dynamic imports are a problem and are not really what we're looking for,
            instead it seems like we need something different.

            Something like:

            - Build all the collections.
            - Creating a temp server file.
            - Using a macro generate the appropiate router

            Having like a declarative macro that's used to create a temp
            server file, then we run that.

             */

            println!("Serving your workspace");
        }
        None => {}
    }
}

pub fn get_collections() -> Vec<String> {
    let cwd: PathBuf = std::env::current_dir().unwrap();
    let cwd_string = cwd.to_str().unwrap();
    let znap_file_path = format!("{}/Znap.toml", cwd_string);
    let znap_file = read_to_string(znap_file_path).expect("Should have been able to read the file");
    let Config { collections } = toml::from_str(&znap_file).unwrap();

    collections
}

// Runs the build command outside of a workspace.
fn build_collection(collection_name: String) -> Result<()> {
    let exit = std::process::Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg(collection_name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| anyhow::format_err!("{}", e.to_string()))?;

    if !exit.status.success() {
        std::process::exit(exit.status.code().unwrap_or(1));
    }

    Ok(())
}

fn build_all() {
    let collections = get_collections();

    for collection in collections.iter() {
        match build_collection(collection.clone()) {
            Ok(_) => {}
            _ => panic!("Failed to build collection: {}", collection),
        }
    }
}
