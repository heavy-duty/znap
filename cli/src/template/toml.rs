pub fn template(collections: &Vec<String>) -> String {
    let cargo_content_without_collections = "[package]\n\
        name = \"znap-server\"\n\
        version = \"0.1.0\"\n\
        edition = \"2021\"\n\
        \n[dependencies]\n\
        tokio = { version = \"1\", features = [\"full\"] }\n\
        axum = \"0.7\"\n\
    ";
    let collection_dependencies: Vec<String> = collections
        .iter()
        .map(|collection| {
            format!(
                "{} = {{ path = \"../../collections/{}\" }}",
                collection, collection
            )
        })
        .collect();
    let collection_dependencies_joined = collection_dependencies.join("\n");

    format!(
        "{}{}",
        cargo_content_without_collections, collection_dependencies_joined
    )
}

