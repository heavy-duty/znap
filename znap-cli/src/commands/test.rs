use crate::utils::{
    generate_collection_executable_files, get_collections, get_config, run_test_suite,
    start_server, wait_for_server,
};

pub fn run(name: &String, address: &String, port: &u16, protocol: &String) {
    let config = get_config();

    let collections = get_collections(&config);

    if collections
        .iter()
        .all(|collection| &collection.name != name)
    {
        panic!("Collection not found.")
    }

    // Generate all server
    generate_collection_executable_files(name, address, port, protocol);

    // Start server in background
    let mut start_server_process = start_server(name, &config);

    // While true with a sleep until server is online
    wait_for_server(address, port, protocol);

    // Run tests suite
    run_test_suite();

    // Kill the server process.
    start_server_process.kill().unwrap();
}
