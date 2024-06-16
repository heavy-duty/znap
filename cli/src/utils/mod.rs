use std::{fs::read_to_string, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub collections: Vec<String>,
}


pub fn get_collections() -> Vec<String> {
    let cwd: PathBuf = std::env::current_dir().unwrap();
    let cwd_string = cwd.to_str().unwrap();
    let znap_file_path = format!("{}/Znap.toml", cwd_string);
    let znap_file = read_to_string(znap_file_path).expect("Should have been able to read the file");
    let Config { collections } = toml::from_str(&znap_file).unwrap();

    collections
}