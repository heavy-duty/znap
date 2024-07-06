use crate::utils::{
    generate_server_files, get_config, run_test_suite, start_server, wait_for_server,
};

pub fn run(address: &str, port: &u16, protocol: &str) {
    let config = get_config();

    // Generate all server
    generate_server_files(&config, address, port, protocol);

    // Start server in background
    let mut start_server_process = start_server(&config);

    // While true with a sleep until server is online
    wait_for_server(address, port, protocol);

    // Run tests suite
    run_test_suite();

    // Kill the server process.
    start_server_process.kill().unwrap();
}
