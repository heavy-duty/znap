use crate::{
    template::{
        server_api::template as server_api_template, server_toml::template as server_toml_template,
    },
    utils::{get_collections, get_identity, write_file},
};
use std::{
    fs::{create_dir, remove_dir_all},
    path::PathBuf,
    process::Stdio,
};
use tempfile::tempdir_in;

pub fn run(address: &str, port: u16) {
    // Get all the collections in the workspace
    let collections = get_collections();

    // Get identity of the workspace
    let identity = get_identity();

    // Create a temporal directory and store the code there.
    let cwd: PathBuf = std::env::current_dir().unwrap();
    let dir = tempdir_in(cwd.join(".znap")).unwrap();
    let dir_path = dir.path();
    let toml_path = dir_path.join("Cargo.toml");
    let api_path = dir_path.join("src/main.rs");

    // Generate api file
    create_dir(dir_path.join("src")).unwrap();
    write_file(&api_path, &server_api_template(&collections, address, port));

    // Generate cargo file
    write_file(&toml_path, &server_toml_template(&collections));

    // Remove tmp files if user hits Ctrl-C
    ctrlc::set_handler(move || {
        println!("Removing temp files");

        remove_dir_all(&dir.path()).unwrap()
    })
    .expect("Error setting Ctrl-C handler");

    // Run the server
    let exit = std::process::Command::new("cargo")
        .env("IDENTITY_KEYPAIR_PATH", identity)
        .arg("run")
        .arg("--manifest-path")
        .arg(toml_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| anyhow::format_err!("{}", e.to_string()))
        .unwrap();

    if !exit.status.success() {
        std::process::exit(exit.status.code().unwrap_or(1));
    }
}
