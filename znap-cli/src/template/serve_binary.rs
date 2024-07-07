use heck::ToSnekCase;

pub fn template(name: &String, address: &String, port: &u16, protocol: &String) -> String {
    format!(
        r#"
use tokio::net::TcpListener;
use colored::Colorize;
use console::Emoji;
use {}::{{collection_router, display_collection_routes}};

#[tokio::main]
async fn main() -> Result<(), axum::Error> {{
    println!("");
    println!(
        "{{}} Znap Server {{}} \n\n Service is running at {{}}",
        Emoji("✨", ""),
        Emoji("✨", ""),
        "{protocol}://{address}:{port}".cyan()
    );

    display_collection_routes();

    println!(
        "\n{{}} {{}}\n",
        Emoji("💡", ""),
        "Press Ctrl+C to stop the server".bright_red().italic(), 
    );
    
    let listener = TcpListener::bind("{address}:{port}").await.unwrap();
    axum::serve(listener, collection_router().into_make_service())
        .await
        .unwrap();

    Ok(())
}}
"#,
        name.to_snek_case()
    )
}
