use znap_common::get_config;

use crate::utils::{deploy_to_shuttle, generate_collection_executable_files};

pub fn run(name: &str, project: &str) {
    let config = get_config(None);
    let collections = config.collections.as_deref().unwrap_or_default();
    let collection = collections
        .iter()
        .find(|collection| collection.name == *name);

    if let Some(collection) = collection {
        // Generate all the required files
        generate_collection_executable_files(&config, collection);

        // Deploy to shuttle
        deploy_to_shuttle(project, collection);
    } else {
        panic!("Collection not found in the workspace.")
    }
}
