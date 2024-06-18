use heck::ToSnekCase;

use crate::utils::Collection;

pub fn template(collections: &Vec<Collection>) -> String {
    let collection_imports: Vec<String> = collections
        .iter()
        .map(|collection| {
            format!(
                "use {}::collection_router as {}_collection_router;",
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
    let collection_router = format!("let router = Router::new(){};", collection_routes.join(""));

    format!(
        r#"use axum::Router;
use tokio::net::TcpListener;
use colored::Colorize;
use console::Emoji;
{}

#[tokio::main]
async fn main() -> Result<(), axum::Error> {{
    println!("");
    println!(
        "{{}} Znap Server {{}} \n\n Service is running at {{}}\n{{}} {{}}\n",
        Emoji("✨", ""),
        Emoji("✨", ""),
        "http://localhost:3000".cyan(),
        Emoji("💡", ""),
        "Press Ctrl+C to stop the server".bright_red().italic(),
        
    );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    {}

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}}
"#,
        collection_imports.join("\n"),
        collection_router
    )
}
