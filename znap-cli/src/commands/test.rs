use crate::utils::{generate_server_files, get_config, run_test_suite, start_server};

pub fn run(address: &str, port: u16) {
    let config = get_config();

    // Generate all server
    generate_server_files(&config, address, port);

    // Start server in background
    let mut start_server_process = start_server(&config);

    // While true with a sleep until server is online
    std::thread::sleep(std::time::Duration::from_millis(5000));

    // Run tests suite
    run_test_suite();

    // Kill the server process.
    start_server_process.kill().unwrap();
}
