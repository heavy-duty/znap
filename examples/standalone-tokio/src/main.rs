use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), axum::Error> {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let router = Router::new()
        .merge(my_znap_workspace::collection_router());

    my_znap_workspace::display_collection_routes();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
