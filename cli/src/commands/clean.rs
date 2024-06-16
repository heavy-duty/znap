use std::{
    fs::{create_dir, remove_dir_all, File},
    path::PathBuf,
};

pub fn run() {
    let cwd: PathBuf = std::env::current_dir().unwrap();
    remove_dir_all(cwd.join(".znap")).unwrap();
    create_dir(cwd.join(".znap")).unwrap();
    File::create(cwd.join(".znap/.gitkeep")).unwrap();
}
