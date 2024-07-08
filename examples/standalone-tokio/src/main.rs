use axum::Router;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), axum::Error> {
    let address = env::var("COLLECTION_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let port = env::var("COLLECTION_PORT").unwrap_or("3000".to_string());
    let protocol = env::var("COLLECTION_PROTOCOL").unwrap_or("http".to_string());

    println!("");
    println!(
        " Znap Server \n\n Service is running at {}",
        format!("{protocol}://{address}:{port}")
    );

    my_znap_workspace::display_collection_routes();

    println!("Press Ctrl+C to stop the server",);

    let listener = TcpListener::bind(format!("{address}:{port}"))
        .await
        .unwrap();
    let router = Router::new().merge(my_znap_workspace::collection_router());

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
