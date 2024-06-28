use axum::Router;
use colored::Colorize;
use console::Emoji;
use my_actions::{collection_router, display_collection_routes};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), axum::Error> {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let router = Router::new()
        .merge(collection_router());

    println!(
        "\n{} Znap Server {} \n\n Service is running at {}",
        Emoji("✨", ""),
        Emoji("✨", ""),
        "http://127.0.0.1:3000".cyan()
    );

    display_collection_routes();

    println!(
        "\n{} {}\n",
        Emoji("💡", ""),
        "Press Ctrl+C to stop the server".bright_red().italic(),
    );

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
