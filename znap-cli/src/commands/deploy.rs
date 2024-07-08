use crate::utils::{
    deploy_to_shuttle, generate_collection_executable_files, get_collections, get_config,
};

pub fn run(name: &String, project: &String) {
    let config = get_config();

    let collections = get_collections(&config);

    if collections
        .iter()
        .all(|collection| &collection.name != name)
    {
        panic!("Collection not found.")
    }

    // Generate all the required files
    generate_collection_executable_files(name);

    // Deploy to shuttle
    deploy_to_shuttle(name, project);
}
