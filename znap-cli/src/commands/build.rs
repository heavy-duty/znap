use crate::utils::{build_for_release, generate_collection_executable_files, get_config, Config};

pub fn run(name: &String) {
    let Config { collections, .. } = get_config();

    if let Some(collections) = collections {
        if collections
            .iter()
            .all(|collection| &collection.name != name)
        {
            panic!("Collection not found.")
        }
    } else {
        panic!("Workspace has no collections.")
    }

    generate_collection_executable_files(name);

    build_for_release(name);
}
