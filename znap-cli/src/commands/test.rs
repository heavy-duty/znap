use crate::utils::{
    generate_collection_executable_files, get_config, get_identity, run_test_suite, start_server,
    wait_for_server,
};
use std::process::Child;

pub fn run() {
    // get config
    let config = get_config();
    let collections = config.collections.unwrap_or(vec![]);

    // start and wait for each server to be running
    let mut server_processes: Vec<Child> = vec![];

    for collection in collections {
        // Generate all server
        generate_collection_executable_files(&collection);

        // Start server in background
        let server_process = start_server(
            &collection.name,
            &get_identity(&config.identity),
            &Some(collection.address.clone()),
            &Some(collection.port.clone()),
            &Some(collection.protocol.clone()),
        );

        // While true with a sleep until server is online
        wait_for_server(&collection.address, &collection.port, &collection.protocol);

        // Push to vector of processes
        server_processes.push(server_process);
    }

    // Run the test suite
    run_test_suite();

    // kill all servers
    for mut process in server_processes {
        process.kill().unwrap();
    }
}
