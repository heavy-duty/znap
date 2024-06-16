use crate::utils::{build_collection, get_collections};

pub fn run() {
    let collections = get_collections();

    for collection in collections.iter() {
        match build_collection(collection.name.clone()) {
            Ok(_) => {}
            _ => panic!("Failed to build collection: {}", collection.name),
        }
    }
}