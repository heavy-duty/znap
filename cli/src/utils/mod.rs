use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{
    fs::{read_to_string, File},
    path::{Path, PathBuf},
    process::Stdio,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub collections: Vec<String>,
}

pub struct Collection {
    pub path: PathBuf,
    pub name: String,
}

pub fn get_collections() -> Vec<Collection> {
    let cwd: PathBuf = std::env::current_dir().unwrap();
    let cwd_string = cwd.to_str().unwrap();
    let znap_file_path = format!("{}/Znap.toml", cwd_string);
    let znap_file = read_to_string(znap_file_path).expect("Should have been able to read the file");
    let config: Config = toml::from_str(&znap_file).unwrap();
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

pub fn write_file(toml_path: &Path, content: &String) {
    let mut file = File::create(&toml_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub fn build_collection(collection_name: String) -> Result<()> {
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