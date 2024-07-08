use crate::utils::{deploy_to_shuttle, generate_collection_executable_files, get_config};

pub fn run(name: &String, project: &String) {
    let config = get_config();
    let collections = config.collections.unwrap_or(vec![]);
    let collection = collections
        .iter()
        .find(|collection| collection.name == *name);

    if let Some(collection) = collection {
        // Generate all the required files
        generate_collection_executable_files(collection);

        // Deploy to shuttle
        deploy_to_shuttle(name, project);
    } else {
        panic!("Collection not found in the workspace.")
    }
}
