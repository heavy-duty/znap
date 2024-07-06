use crate::utils::{generate_server_files, get_config, start_server_blocking};

pub fn run(address: &str, port: &u16, protocol: &str) {
    let config = get_config();

    // Generate all server
    generate_server_files(&config, address, port, protocol);

    // Run the server
    start_server_blocking(&config);
}
