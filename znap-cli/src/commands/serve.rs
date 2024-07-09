use crate::utils::{
    generate_collection_executable_files, get_config, get_identity, start_server_blocking,
};

pub fn run(name: &String, address: &Option<String>, port: &Option<u16>, protocol: &Option<String>) {
    let config = get_config();
    let collections = config.collections.unwrap_or_default();
    let collection = collections
        .iter()
        .find(|collection| collection.name == *name);

    if let Some(collection) = collection {
        // Generate all the required files
        generate_collection_executable_files(collection);

        // Run the server
        start_server_blocking(
            name,
            &get_identity(&config.identity),
            address,
            port,
            protocol,
        );
    } else {
        panic!("Collection not found in the workspace.")
    }
}
