use axum::{routing::get, Router};
use my_actions::{
    handle_get_dynamic_transfer_action, handle_get_fixed_transfer_action,
    handle_post_dynamic_transfer_action, handle_post_fixed_transfer_action,
};
use tokio::net::TcpListener;
use znap_lang::*;

#[tokio::main]
async fn main() -> Result<(), ActionError> {
    let routes_all = Router::new()
        .route(
            "/actions/fixed_transfer",
            get(handle_get_fixed_transfer_action).post(handle_post_fixed_transfer_action),
        )
        .route(
            "/actions/dynamic_transfer",
            get(handle_get_dynamic_transfer_action).post(handle_post_dynamic_transfer_action),
        );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
