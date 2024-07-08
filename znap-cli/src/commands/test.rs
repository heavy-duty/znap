use crate::utils::{
    generate_collection_executable_files, get_config, get_identity, run_test_suite, start_server,
    wait_for_server,
};

pub fn run(name: &String, address: &String, port: &u16, protocol: &String) {
    let config = get_config();
    let collections = config.collections.unwrap_or(vec![]);
    let collection = collections
        .iter()
        .find(|collection| collection.name == *name);

    if let Some(collection) = collection {
        // Generate all server
        generate_collection_executable_files(collection);

        // Start server in background
        let mut start_server_process = start_server(
            name,
            &get_identity(&config.identity),
            &Some(address.clone()),
            &Some(port.clone()),
            &Some(protocol.clone()),
        );

        // While true with a sleep until server is online
        wait_for_server(address, port, protocol);

        // Run tests suite
        run_test_suite();

        // Kill the server process.
        start_server_process.kill().unwrap();
    } else {
        panic!("Collection not found in the workspace.")
    }
}
