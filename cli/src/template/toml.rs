use crate::utils::Collection;

pub fn template(name: &String, collections: &Vec<Collection>) -> String {
    let cargo_content_without_collections = format!("[package]\n\
        name = \"{}\"\n\
        version = \"0.1.0\"\n\
        edition = \"2021\"\n\
        \n[dependencies]\n\
        tokio = {{ version = \"1\", features = [\"full\"] }}\n\
        axum = \"0.7\"\n\
    ", name);
    let collection_dependencies: Vec<String> = collections
        .iter()
        .map(|collection| {
            format!(
                "{} = {{ path = \"{}\" }}",
                collection.name, collection.path.to_str().unwrap()
            )
        })
        .collect();
    let collection_dependencies_joined = collection_dependencies.join("\n");

    format!(
        "{}{}",
        cargo_content_without_collections, collection_dependencies_joined
    )
}

