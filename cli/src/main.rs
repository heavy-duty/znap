use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::Stdio;
use template::toml::template as toml_template;
use template::api::template as api_template;
use utils::get_collections;
mod template;
mod utils;

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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build) => build_all(),
        Some(Commands::Serve) => serve_all(),
        Some(Commands::Test) => {
            println!("Testing your workspace");
        }
        None => {}
    }
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

fn serve_all() {
    /*

    What happens if there's a server already? Should it be updated?
    Generated entirely again?

    What happens if we are running the server and another instance
    starts? Should updates be allowed? Would those changes even
    affect our current thing?

    There is no way to have a duplicate collection, but duplicate actions
    can occur. Each action can be prefixed with the collection and since
    there can't be duplicate actions within a collection, the problem
    is solved.

    */

    let collections = get_collections();

    // Generate Cargo.toml content
    let cargo_content = toml_template(&collections);
    println!("Cargo: \n{}\n", cargo_content);

    // Generate server file content.
    let api_content = api_template(&collections);
    println!("API: \n{}\n", api_content);

    // Check .znap and .znap/server exist
    
    // Create/Update cargo.toml and main.rs

    // Run the server using cargo run --manifest-path

    println!("Serving your workspace");
}
