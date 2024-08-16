use heck::ToSnekCase;
use znap_common::Collection;

pub fn template(collection: &Collection) -> String {
    format!(
        r#"
use tokio::net::TcpListener;
use colored::Colorize;
use console::Emoji;
use std::env;
use {}::{{collection_router, display_collection_routes}};

#[tokio::main]
async fn main() -> Result<(), axum::Error> {{
    let address = env::var("COLLECTION_ADDRESS").unwrap_or("{}".to_string());
    let port = env::var("COLLECTION_PORT").unwrap_or("{}".to_string());
    let protocol = env::var("COLLECTION_PROTOCOL").unwrap_or("{}".to_string());

    println!("");
    println!(
        "{{}} Znap Server {{}} \n\n Service is running at {{}}",
        Emoji("✨", ""),
        Emoji("✨", ""),
        format!("{{protocol}}://{{address}}:{{port}}").cyan()
    );

    display_collection_routes();

    println!(
        "\n{{}} {{}}\n",
        Emoji("💡", ""),
        "Press Ctrl+C to stop the server".bright_red().italic(), 
    );
    
    let listener = TcpListener::bind(format!("{{address}}:{{port}}")).await.unwrap();
    axum::serve(listener, collection_router().into_make_service())
        .await
        .unwrap();

    Ok(())
}}
"#,
        &collection.name.to_snek_case(),
        &collection.address,
        &collection.port,
        &collection.protocol,
    )
}
