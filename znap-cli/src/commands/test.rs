use crate::utils::{
    generate_collection_executable_files, get_config, get_identity, run_test_suite, start_server,
    wait_for_server, Config,
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

    // Generate all server
    generate_collection_executable_files(name);

    // Start server in background
    let mut start_server_process =
        start_server(name, &get_identity(&identity), address, port, protocol);

    // While true with a sleep until server is online
    wait_for_server(address, port, protocol);

    // Run tests suite
    run_test_suite();

    // Kill the server process.
    start_server_process.kill().unwrap();
}
