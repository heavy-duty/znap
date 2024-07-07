use crate::utils::{
    generate_collection_executable_files, get_collections, get_config, start_server_blocking,
};

pub fn run(name: &String, address: &String, port: &u16, protocol: &String) {
    let config = get_config();

    let collections = get_collections(&config);

    if collections
        .iter()
        .all(|collection| &collection.name != name)
    {
        panic!("Collection not found.")
    }

    // Generate all the required files
    generate_collection_executable_files(name, address, port, protocol);

    // Run the server
    start_server_blocking(name, &config);
}
