use znap_common::get_config;

use crate::utils::{generate_collection_executable_files, start_server_blocking};

pub fn run(name: &str, address: Option<&str>, port: Option<&u16>, protocol: Option<&str>) {
    let config = get_config(None);
    let collections = config.collections.as_deref().unwrap_or_default();
    let collection = collections
        .iter()
        .find(|collection| collection.name == *name);

    if let Some(collection) = collection {
        // Generate all the required files
        generate_collection_executable_files(&config, collection);

        // Run the server
        start_server_blocking(&config, collection, address, port, protocol);
    } else {
        panic!("Collection not found in the workspace.")
    }
}
