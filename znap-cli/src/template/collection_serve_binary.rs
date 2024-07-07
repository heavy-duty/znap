use heck::ToSnekCase;

pub fn template(name: &String) -> String {
    format!(
        r#"
use tokio::net::TcpListener;
use colored::Colorize;
use console::Emoji;
use {}::{{collection_router, display_collection_routes}};

#[tokio::main]
async fn main() -> Result<(), axum::Error> {{
    let address = env::var("COLLECTION_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let port = env::var("COLLECTION_PORT").unwrap_or("3000".to_string());
    let protocol = env::var("COLLECTION_PROTOCOL").unwrap_or("3000".to_string());

    println!("");
    println!(
        "{{}} Znap Server {{}} \n\n Service is running at {{}}",
        Emoji("✨", ""),
        Emoji("✨", ""),
        "{{protocol}}://{{address}}:{{port}}".cyan()
    );

    display_collection_routes();

    println!(
        "\n{{}} {{}}\n",
        Emoji("💡", ""),
        "Press Ctrl+C to stop the server".bright_red().italic(), 
    );
    
    let listener = TcpListener::bind("{{address}}:{{port}}").await.unwrap();
    axum::serve(listener, collection_router().into_make_service())
        .await
        .unwrap();

    Ok(())
}}
"#,
        name.to_snek_case()
    )
}
