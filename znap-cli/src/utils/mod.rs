use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{
    fs::{read_to_string, File},
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub collections: Vec<String>,
    pub identity: String,
}

pub struct Collection {
    pub path: PathBuf,
    pub name: String,
}


pub fn get_config() -> Config {
    let cwd: PathBuf = std::env::current_dir().unwrap();
    let cwd_string = cwd.to_str().unwrap();
    let znap_file_path = format!("{}/Znap.toml", cwd_string);
    let znap_file = read_to_string(znap_file_path).expect("Should have been able to read the file");

    toml::from_str(&znap_file).unwrap()
}

pub fn get_collections() -> Vec<Collection> {
    let cwd: PathBuf = std::env::current_dir().unwrap();
    let config: Config = get_config();
    let collections_dir_path = cwd.join("collections");
    let collections: Vec<Collection> = config
        .collections
        .iter()
        .map(|collection| Collection {
            path: collections_dir_path.join(collection),
            name: collection.clone(),
        })
        .collect();

    collections
}

pub fn get_identity() -> String {
    let config: Config = get_config();

    shellexpand::tilde(&config.identity).to_string()
}

pub fn write_file(path: &Path, content: &String) {
    let mut file = File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
