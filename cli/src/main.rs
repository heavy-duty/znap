use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs::{create_dir, remove_dir_all, File};
use std::path::PathBuf;
use std::process::Stdio;
use tempfile::tempdir_in;
use template::api::template as api_template;
use template::toml::template as toml_template;
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
    // Get all the collections in the workspace
    let collections = get_collections();

    // Create a temporal directory and store the code there.
    let cwd: PathBuf = std::env::current_dir().unwrap();
    let dir = tempdir_in(cwd.join(".znap")).unwrap();
    create_dir(dir.path().join("src")).unwrap();

    // Generate Cargo.toml content
    let cargo_content = toml_template(&collections);

    // Generate server file content.
    let api_content = api_template(&collections);

    let api_path = dir.path().join("src/main.rs");
    let mut api_file = File::create(api_path).unwrap();
    api_file.write_all(api_content.as_bytes()).unwrap();

    let toml_path = dir.path().join("Cargo.toml");
    let mut toml_file = File::create(&toml_path).unwrap();
    toml_file.write_all(cargo_content.as_bytes()).unwrap();

    // Remove tmp files if user hits Ctrl-C
    ctrlc::set_handler(move || {
        println!("Removing temp files");
        remove_dir_all(dir.path()).unwrap()
    })
    
    .expect("Error setting Ctrl-C handler");

    // Run the server using cargo run --manifest-path
    let exit = std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg(&toml_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| anyhow::format_err!("{}", e.to_string()))
        .unwrap();

    if !exit.status.success() {
        std::process::exit(exit.status.code().unwrap_or(1));
    }
}
