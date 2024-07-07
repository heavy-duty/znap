use crate::template;
use serde::{Deserialize, Serialize};
use std::fs::{copy, create_dir, create_dir_all, read_dir, remove_dir_all};
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

pub fn start_server_blocking(name: &String, config: &Config) {
    let start_server_process = start_server(name, config);
    let exit = start_server_process
        .wait_with_output()
        .expect("Should be able to start server");

    if !exit.status.success() {
        std::process::exit(exit.status.code().unwrap_or(1));
    }
}

pub fn start_server(name: &String, config: &Config) -> Child {
    std::process::Command::new("cargo")
        .env("IDENTITY_KEYPAIR_PATH", get_identity(config))
        .arg("run")
        .arg("--manifest-path")
        .arg(get_cwd().join(&format!(".znap/collections/{name}/Cargo.toml")))
        .arg("--bin")
        .arg("serve")
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

pub fn deploy_to_shuttle(name: &String, project: &String) {
    std::process::Command::new("cargo")
        .arg("shuttle")
        .arg("deploy")
        .arg("--allow-dirty")
        .arg("--name")
        .arg(project)
        .arg("--working-directory")
        .arg(get_cwd().join(&format!(".znap/collections/{name}")))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(anyhow::Error::from)
        .expect("Make sure you are logged into shuttle.")
        .wait_with_output()
        .expect("Should wait until the deploy is over");
}

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) {
    create_dir_all(&destination).unwrap();
    for entry in read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let filetype = entry.file_type().unwrap();
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()));
        } else {
            copy(entry.path(), destination.as_ref().join(entry.file_name())).unwrap();
        }
    }
}

pub fn generate_collection_executable_files(
    name: &String,
    address: &String,
    port: &u16,
    protocol: &String,
) {
    let cwd = get_cwd();

    let znap_path = cwd.join(".znap");

    if !znap_path.exists() {
        create_dir(&znap_path).expect("Could not create .znap folder");
    }

    let znap_toml_path = znap_path.join("Cargo.toml");

    write_file(
        &znap_toml_path,
        &String::from(
            "[workspace]\nmembers = [\"collections/*\"]\nresolver = \"2\"\n\n[patch.crates-io]\ncurve25519-dalek = { git = \"https://github.com/dalek-cryptography/curve25519-dalek\", rev = \"8274d5cbb6fc3f38cdc742b4798173895cd2a290\" }",
        ),
    );

    let znap_collections_path = znap_path.join("collections");

    if !znap_collections_path.exists() {
        create_dir(&znap_collections_path).expect("Could not create .znap/collections folder");
    }

    let znap_collection_path = znap_collections_path.join(name);

    if znap_collection_path.exists() {
        remove_dir_all(&znap_collection_path)
            .expect(&format!("Could not delete .znap/{name} folder"))
    }

    create_dir(&znap_collection_path).expect(&format!("Could not create .znap/{name} folder"));

    let znap_collection_src_path = znap_collection_path.join("src");

    create_dir(&znap_collection_src_path)
        .expect(&format!("Could not create .znap/{name}/src folder"));

    let znap_collection_src_bin_path = znap_collection_src_path.join("bin");

    create_dir(&znap_collection_src_bin_path)
        .expect(&format!("Could not create .znap/{name}/src/bin folder"));

    let collection_path = cwd.join(&format!("collections/{name}"));
    let collection_src_path = collection_path.join("src");

    copy_recursively(collection_src_path, znap_collection_src_path);

    // Generate the binaries
    let znap_collection_src_bin_serve_path = znap_collection_src_bin_path.join("serve.rs");
    write_file(
        &znap_collection_src_bin_serve_path,
        &template::collection_serve_binary::template(name, address, port, protocol),
    );

    let znap_collection_src_bin_deploy_path = znap_collection_src_bin_path.join("deploy.rs");
    write_file(
        &znap_collection_src_bin_deploy_path,
        &template::collection_deploy_binary::template(name),
    );

    // Generate a toml with collection and extras for serve/deploy
    let znap_collection_toml_path = znap_collection_path.join("Cargo.toml");
    let collection_toml_path = collection_path.join("Cargo.toml");

    let collection_toml = read_to_string(collection_toml_path).unwrap();
    let znap_toml_extras = template::collection_toml::template(name);

    write_file(
        &znap_collection_toml_path,
        &format!("{collection_toml}\n{znap_toml_extras}"),
    );
}

pub fn build_for_release(name: &String) {
    std::process::Command::new("cargo")
        .arg("build")
        .arg("--manifest-path")
        .arg(get_cwd().join(&format!(".znap/collections/{name}/Cargo.toml")))
        .arg("--release")
        .arg("--bin")
        .arg("serve")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(anyhow::Error::from)
        .expect("Should be able to build collection.")
        .wait_with_output()
        .expect("Should wait until the build is over");
}
