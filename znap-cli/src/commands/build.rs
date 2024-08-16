use znap_common::get_config;

use crate::utils::{build_for_release, generate_collection_executable_files};

pub fn run(name: &str) {
    let config = get_config(None);
    let collections = config.collections.as_deref().unwrap_or_default();
    let collection = collections
        .iter()
        .find(|collection| collection.name == *name);

    if let Some(collection) = collection {
        generate_collection_executable_files(&config, collection);

        build_for_release(name);
    } else {
        panic!("Collection not found in the workspace.")
    }
}
