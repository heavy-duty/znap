use axum::Router;
use my_actions::collection_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), axum::Error> {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let router = Router::new().merge(collection_router());

    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
