pub use self::error::{Error, Result};

use axum::{extract::Path, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

mod error;

#[tokio::main]
async fn main() -> Result<()> {
    let routes_all = Router::new().merge(routes_actions());

    // region:    --- Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}

// region:    --- Routes Hello
fn routes_actions() -> Router {
    Router::new().route(
        "/actions/:action_id",
        get(handle_get_action).post(handle_create_action),
    )
}

#[derive(Debug, Deserialize)]
struct GetActionParams {
    action_id: u32,
}

#[derive(Debug, Serialize)]
struct GetFixedTransferResponse {
    icon: String,
    title: String,
    description: String,
    label: String,
    disabled: bool,
}

// e.g., `/actions/:actionId`
async fn handle_get_action(
    Path(params): Path<GetActionParams>,
) -> Result<Json<GetFixedTransferResponse>> {
    println!("->> {:<12} - get_action - {params:?}", "HANDLER");

    let response = match params.action_id {
        0 => GetFixedTransferResponse {
            icon: "icon".to_string(),
            title: "asd".to_string(),
            description: "".to_string(),
            label: "".to_string(),
            disabled: false,
        },
        _ => return Err(Error::ActionNotFound),
    };

    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
struct CreateActionParams {
    action_id: u32,
}

#[derive(Debug, Deserialize)]
struct CreateActionPayload {
    account: String,
}

#[derive(Debug, Serialize)]
struct CreateFixedTransferResponse {
    transaction: String,
    message: Option<String>,
}

// e.g., `/actions/:actionId`
async fn handle_create_action(
    Path(params): Path<CreateActionParams>,
    Json(payload): Json<CreateActionPayload>,
) -> Result<Json<CreateFixedTransferResponse>> {
    println!("->> {:<12} - create_action - {params:?}", "HANDLER");
    println!("->> {:<12} - create_action - {payload:?}", "HANDLER");

    let response = match params.action_id {
        0 => CreateFixedTransferResponse {
            transaction: "".to_string(),
            message: None,
        },
        _ => return Err(Error::ActionNotFound),
    };

    Ok(Json(response))
}
