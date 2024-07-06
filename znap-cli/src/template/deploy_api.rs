use heck::ToSnekCase;

use crate::utils::Collection;

pub fn template(collections: &[Collection]) -> String {
    let collection_imports: Vec<String> = collections
        .iter()
        .map(|collection| {
            format!(
                "use {}::{{collection_router as {}_collection_router, display_collection_routes as {}_display_collection_routes}};",
                collection.name.to_snek_case(),
                collection.name.to_snek_case(),
                collection.name.to_snek_case()
            )
        })
        .collect();

    let collection_routes: Vec<String> = collections
        .iter()
        .map(|collection| {
            format!(
                ".merge({}_collection_router())",
                collection.name.to_snek_case()
            )
        })
        .collect();

    let collection_prints: Vec<String> = collections
        .iter()
        .map(|collection| {
            format!(
                "{}_display_collection_routes();",
                collection.name.to_snek_case()
            )
        })
        .collect();

    let collection_router = format!("let router = Router::new(){};", collection_routes.join(""));

    format!(
        r#"use axum::Router;
use colored::Colorize;
use console::Emoji;
{}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {{
    println!("");
    println!(
        "{{}} Znap Server {{}} \n\n Service is running.",
        Emoji("✨", ""),
        Emoji("✨", "")
    );

    {}

    {}

    println!(
        "\n{{}} {{}}\n",
        Emoji("💡", ""),
        "Press Ctrl+C to stop the server".bright_red().italic(), 
    );
    
    Ok(router.into())
}}
"#,
        collection_imports.join("\n"),
        collection_prints.join("\n"),
        collection_router
    )
}
