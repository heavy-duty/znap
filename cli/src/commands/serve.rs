use crate::{
    template::api::template as api_template,
    template::toml::template as toml_template,
    utils::{get_collections, run_server, write_file},
};
use std::{
    fs::{create_dir, remove_dir_all},
    path::PathBuf,
};
use tempfile::tempdir_in;

pub fn run() {
    // Get all the collections in the workspace
    let collections = get_collections();

    // Create a temporal directory and store the code there.
    let cwd: PathBuf = std::env::current_dir().unwrap();
    let dir = tempdir_in(cwd.join(".znap")).unwrap();
    let dir_path = dir.path();
    let toml_path = dir_path.join("Cargo.toml");
    let api_path = dir_path.join("src/main.rs");

    // Generate api file
    create_dir(dir_path.join("src")).unwrap();
    write_file(&api_path, &api_template(&collections));

    // Generate cargo file
    write_file(&toml_path, &toml_template(&collections));

    // Remove tmp files if user hits Ctrl-C
    ctrlc::set_handler(move || {
        println!("Removing temp files");

        remove_dir_all(&dir.path()).unwrap()
    })
    .expect("Error setting Ctrl-C handler");

    // Run the server
    run_server(&toml_path)
}
