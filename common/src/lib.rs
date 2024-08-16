use std::fs::read_to_string;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub protocol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub collections: Option<Vec<Collection>>,
    pub identity: Option<String>,
    pub rpc_url: Option<String>,
}

pub fn get_cwd() -> PathBuf {
    std::env::current_dir().expect("Shoud be able to read cwd")
}

pub fn get_config(path: Option<PathBuf>) -> Config {
    let cwd = path.unwrap_or_else(|| get_cwd());
    let znap_file_path = cwd.join("Znap.toml");
    let znap_file = read_to_string(znap_file_path)
        .expect("Should be able to read Znap.toml file. Make sure you are in a Znap workspace.");

    toml::from_str(&znap_file).expect("Znap.toml file should have the proper format")
}
