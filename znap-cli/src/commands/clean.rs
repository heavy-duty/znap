use crate::utils::get_cwd;
use std::fs::{create_dir, remove_dir_all, File};

pub fn run() {
    let znap_path = get_cwd().join(".znap");
    let znap_gitkeep_path = znap_path.join(".gitkeep");

    // Remove the current .znap folder and its content
    remove_dir_all(&znap_path).expect("Should be able to remove .znap folder");

    // Create a new .znap folder with a .gitkeep
    create_dir(&znap_path).expect("Should be able to create a .znap folder");
    File::create(&znap_gitkeep_path).expect("Should be able to create a .znap/.gitkeep file");
}
