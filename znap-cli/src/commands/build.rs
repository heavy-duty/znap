use crate::utils::{get_collections, get_config};
use std::process::Stdio;

pub fn run() {
    let config = get_config();
    let collections = get_collections(&config);

    for collection in collections.iter() {
        let maybe_exit = std::process::Command::new("cargo")
            .arg("build")
            .arg("-p")
            .arg(&collection.name)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| anyhow::format_err!("{}", e.to_string()));

        match maybe_exit {
            Ok(exit) => {
                if !exit.status.success() {
                    std::process::exit(exit.status.code().unwrap_or(1));
                }
            }
            _ => panic!("Failed to build collection: {}", collection.name),
        }
    }
}
