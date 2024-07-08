use crate::utils::{
    generate_collection_executable_files, get_config, get_identity, start_server_blocking, Config,
};

pub fn run(name: &String, address: &String, port: &u16, protocol: &String) {
    let Config { collections, identity } = get_config();

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

    // Run the server
    start_server_blocking(name, &get_identity(&identity), address, port, protocol);
}
