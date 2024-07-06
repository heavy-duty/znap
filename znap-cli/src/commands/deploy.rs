use crate::utils::{deploy_to_shuttle, generate_deploy_files, get_config};

pub fn run(name: &String) {
    let config = get_config();

    // Generate all the required files
    generate_deploy_files(&config);

    // Deploy to shuttle
    deploy_to_shuttle(name);
}
