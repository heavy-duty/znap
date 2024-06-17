use crate::{
    template::{
        collection_body::template as collection_body_template,
        collection_toml::template as collection_toml_template,
    },
    utils::{write_file, Config},
};
use heck::ToKebabCase;
use std::fs::{create_dir, read_to_string};

pub fn run(name: &String) {
    // Create a folder for the collection in the collections folder.
    let cwd = std::env::current_dir().unwrap();
    let collection_dir = cwd.join("collections").join(name.to_kebab_case());
    create_dir(&collection_dir).unwrap();

    // Create a Cargo.toml for the collection.
    write_file(
        collection_dir.join("Cargo.toml").as_path(),
        &collection_toml_template(&name),
    );

    // Create a src folder.
    let collection_src_dir = collection_dir.join("src");
    create_dir(&collection_src_dir).unwrap();

    // Create a lib.rs in the src folder.
    write_file(
        collection_src_dir.join("lib.rs").as_path(),
        &collection_body_template(&name),
    );

    // Add to collections list in Znap.toml.
    let znap_toml_path = cwd.join("Znap.toml");
    let znap_toml = read_to_string(&znap_toml_path).unwrap();

    let Config { mut collections } = toml::from_str(&znap_toml).unwrap();

    collections.push(name.to_kebab_case());

    write_file(
        &znap_toml_path.as_path(),
        &toml::to_string(&Config { collections }).unwrap(),
    );
}
