use crate::utils::{generate_server_files, get_config, start_server_blocking};

pub fn run(address: &String, port: &u16, protocol: &String) {
    let config = get_config();

    // Generate all the required files
    generate_server_files(&config, address, port, protocol);

    // Run the server
    start_server_blocking(&config);
}
