use crate::template::{
    server_api::template as server_api_template, server_toml::template as server_toml_template,
};
use serde::{Deserialize, Serialize};
use std::fs::create_dir;
use std::io::Write;
use std::process::{Child, Stdio};
use std::{
    fs::{read_to_string, File},
    path::{Path, PathBuf},
};
use znap::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub collections: Vec<String>,
    pub identity: String,
}

pub struct Collection {
    pub path: PathBuf,
    pub name: String,
}

pub fn get_cwd() -> PathBuf {
    std::env::current_dir().expect("Shoud be able to read cwd")
}

pub fn get_config() -> Config {
    let cwd = get_cwd();
    let znap_file_path = cwd.join("Znap.toml");
    let znap_file = read_to_string(znap_file_path)
        .expect("Should be able to read Znap.toml file. Make sure you are in a Znap workspace.");
    let config: Config =
        toml::from_str(&znap_file).expect("Znap.toml file should have the proper format");

    Config {
        collections: config.collections,
        identity: config.identity,
    }
}

pub fn get_collections(Config { collections, .. }: &Config) -> Vec<Collection> {
    let cwd = get_cwd();
    let collections_dir_path = cwd.join("collections");

    collections
        .iter()
        .map(|collection| Collection {
            path: collections_dir_path.join(collection),
            name: collection.clone(),
        })
        .collect()
}

pub fn get_identity(Config { identity, .. }: &Config) -> String {
    shellexpand::tilde(&identity).into()
}

pub fn write_file(path: &Path, content: &String) {
    let mut file = File::create(&path).expect("Should be able to open file");
    file.write_all(content.as_bytes())
        .expect("Should be able to write file");
}

pub fn generate_server_files(config: &Config, address: &str, port: &u16, protocol: &str) {
    // Get all the collections in the workspace
    let collections = get_collections(&config);

    // Create a server directories if they dont exists.
    let cwd = get_cwd();

    let znap_path = cwd.join(".znap");

    if !znap_path.exists() {
        create_dir(&znap_path).expect("Could not create .znap folder");
    }

    let znap_server_path = znap_path.join("server");

    if !znap_server_path.exists() {
        create_dir(&znap_server_path).expect("Could not create .znap/server folder");
    }

    let znap_server_src_path = znap_server_path.join("src");

    if !znap_server_src_path.exists() {
        create_dir(&znap_server_src_path).expect("Could not create .znap/server/src folder");
    }

    // Generate api file
    let znap_server_src_api_path = znap_server_src_path.join("main.rs");
    write_file(
        &znap_server_src_api_path,
        &server_api_template(&collections, address, port, protocol),
    );

    // Generate cargo file
    let znap_server_toml_path = znap_server_path.join("Cargo.toml");
    write_file(&znap_server_toml_path, &server_toml_template(&collections));
}

pub fn start_server_blocking(config: &Config) {
    let start_server_process = start_server(&config);
    let exit = start_server_process
        .wait_with_output()
        .expect("Should be able to start server");

    if !exit.status.success() {
        std::process::exit(exit.status.code().unwrap_or(1));
    }
}

pub fn start_server(config: &Config) -> Child {
    std::process::Command::new("cargo")
        .env("IDENTITY_KEYPAIR_PATH", get_identity(config))
        .arg("run")
        .arg("--manifest-path")
        .arg(get_cwd().join(".znap/server/Cargo.toml"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| anyhow::format_err!("{}", e.to_string()))
        .expect("Should be able to start server")
}

pub fn run_test_suite() {
    std::process::Command::new("npm")
        .arg("run")
        .arg("test")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(anyhow::Error::from)
        .expect("Should be able to run tests")
        .wait_with_output()
        .expect("Should wait until the tests are over");
}

pub fn wait_for_server(address: &str, port: &u16, protocol: &str) {
    let url = format!("{protocol}://{address}:{port}/status");

    loop {
        if let Ok(response) = reqwest::blocking::get(&url) {
            if let Ok(status) = response.json::<Status>() {
                if status.active {
                    break;
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
