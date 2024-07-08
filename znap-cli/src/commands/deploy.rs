use crate::utils::{deploy_to_shuttle, generate_collection_executable_files, get_config, Config};

pub fn run(name: &String, project: &String) {
    let Config { collections, .. } = get_config();

    if let Some(collections) = collections {
        if collections
            .iter()
            .all(|collection| &collection.name != name)
        {
            panic!("Collection not found.")
        }
    } else {
        panic!("Workspace has no collections.")
    }

    // Generate all the required files
    generate_collection_executable_files(name);

    // Deploy to shuttle
    deploy_to_shuttle(name, project);
}
