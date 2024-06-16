use heck::ToSnekCase;

pub fn template(collections: &Vec<String>) -> String {
    let collection_imports: Vec<String> = collections
        .iter()
        .map(|collection| {
            format!(
                "use {}::collection_router as {}_collection_router;",
                collection.to_snek_case(), collection.to_snek_case()
            )
        })
        .collect();

    let collection_routes: Vec<String> = collections
        .iter()
        .map(|collection| format!(".merge({}_collection_router())", collection.to_snek_case()))
        .collect();

    let collection_router = format!("let router = Router::new(){};", collection_routes.join(""));

    format!(
        r#"use axum::Router;
use tokio::net::TcpListener;
{}

#[tokio::main]
async fn main() -> Result<(), axum::Error> {{
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    {}

    println!("->> LISTENING on {{:?}}\n", listener.local_addr());
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