use crate::utils::write_file;
use heck::ToKebabCase;
use std::fs::create_dir;

pub fn run(name: &String) {
    // Create a folder with the provided name.
    let cwd = std::env::current_dir().unwrap();
    let workspace_dir = cwd.join(name.to_kebab_case());
    create_dir(&workspace_dir).unwrap();

    // Create a Cargo.toml file.
    write_file(
        workspace_dir.join("Cargo.toml").as_path(),
        &String::from("[workspace]\nmembers = [\"collections/*\", \".znap/*\"]"),
    );

    // Create a Znap.toml file.
    write_file(
        workspace_dir.join("Znap.toml").as_path(),
        &String::from("collections = []"),
    );

    // Create a .gitignore file.
    write_file(
        workspace_dir.join(".gitignore").as_path(),
        &String::from("/target\n.znap/.tmp*"),
    );

    // Create a collections folder.
    let collections_dir = &workspace_dir.join("collections");
    create_dir(&collections_dir).unwrap();

    // Create a .gitkeep in the collections folder.
    write_file(
        collections_dir.join(".gitkeep").as_path(),
        &String::from(""),
    );

    // Create a .znap folder.
    let znap_dir = &workspace_dir.join(".znap");
    create_dir(&znap_dir).unwrap();

    // Create a .gitkeep in the .znap folder.
    write_file(znap_dir.join(".gitkeep").as_path(), &String::from(""));
}
