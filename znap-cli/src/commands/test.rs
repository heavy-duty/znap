use crate::utils::{
    generate_collection_executable_files, get_config, run_test_suite, start_server, wait_for_server,
};

pub fn run() {
    // get config
    let config = get_config();
    let collections = config.collections.as_deref().unwrap_or_default();

    // start and wait for each server to be running
    let mut server_processes = vec![];

    for collection in collections {
        // Generate all server
        generate_collection_executable_files(&config, collection);

        // Start server in background
        let server_process = start_server(&config, collection, None, None, None);

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
