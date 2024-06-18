use crate::utils::Collection;

pub fn template(collections: &Vec<Collection>) -> String {
    let cargo_content_without_collections = format!("[package]\n\
        name = \"znap-server\"\n\
        version = \"0.1.4\"\n\
        edition = \"2021\"\n\
        \n[dependencies]\n\
        tokio = {{ version = \"1\", features = [\"full\"] }}\n\
        axum = \"0.7\"\n\
        colored = \"2.1.0\"\n\
        console = \"0.15.8\"\n\
    ");
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

