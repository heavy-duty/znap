use crate::utils::{build_for_release, generate_collection_executable_files, get_collections, get_config};

pub fn run(name: &String, address: &String, port: &u16, protocol: &String) {
    let config = get_config();

    let collections = get_collections(&config);

    if collections
        .iter()
        .all(|collection| &collection.name != name)
    {
        panic!("Collection not found.")
    }

    generate_collection_executable_files(name, address, port, protocol);

    build_for_release(name);
}
