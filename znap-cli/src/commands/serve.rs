use crate::utils::{generate_server_files, get_config, start_server};

pub fn run(address: &str, port: u16) {
    let config = get_config();

    // Generate all server
    generate_server_files(&config, address, port);

    // Run the server
    start_server(&config);
}
