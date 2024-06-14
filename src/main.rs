use my_actions::collection_router;
use tokio::net::TcpListener;
use znap_lang::*;

#[tokio::main]
async fn main() -> Result<(), ActionError> {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    
    println!("{:?}",collection_router());
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, collection_router().into_make_service())
        .await
        .unwrap();

    Ok(())
}
