use anyhow::Result;
use clap::{Parser, Subcommand};
use tempfile::tempdir;
use std::fs::{create_dir, File};
use std::process::Stdio;
use template::toml::template as toml_template;
use template::api::template as api_template;
use utils::get_collections;
mod template;
mod utils;
use std::io::Write;

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
        match build_collection(collection.name.clone()) {
            Ok(_) => {}
            _ => panic!("Failed to build collection: {}", collection.name),
        }
    }
}

fn serve_all() {
    // TODO: We need some sort of caching, at least re-using the target when possible.

    let collections = get_collections();

    // Generate Cargo.toml content
    let cargo_content = toml_template(&collections);
    println!("Cargo: \n{}\n", cargo_content);

    // Generate server file content.
    let api_content = api_template(&collections);
    println!("API: \n{}\n", api_content);

    // Create a temporal directory and store the code there.
    let dir = tempdir().unwrap();

    create_dir(dir.path().join("src")).unwrap();

    let api_path = dir.path().join("src/main.rs");
    let mut api_file = File::create(api_path).unwrap();
    api_file.write_all(api_content.as_bytes()).unwrap();

    let toml_path = dir.path().join("Cargo.toml");
    let mut toml_file = File::create(toml_path).unwrap();
    toml_file.write_all(cargo_content.as_bytes()).unwrap();

    // Run the server using cargo run --manifest-path
    let exit = std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg(dir.path().join("Cargo.toml"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| anyhow::format_err!("{}", e.to_string())).unwrap();

    if !exit.status.success() {
        std::process::exit(exit.status.code().unwrap_or(1));
    }
}
