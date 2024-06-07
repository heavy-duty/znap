use std::str::FromStr;

pub use self::error::{Error, Result};

use axum::{extract::{Path, Query}, routing::get, Json, Router};
use bincode::serialize;
use serde::{Deserialize, Serialize};
use solana_sdk::{message::Message, pubkey::Pubkey, transaction::Transaction};
use tokio::net::TcpListener;
use base64::prelude::*;

mod error;

#[tokio::main]
async fn main() -> Result<()> {
    let routes_all = Router::new().merge(routes_actions());

    // region:    --- Start Server
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
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
}

// e.g., `/actions/:actionId`
async fn handle_get_action(
    Path(params): Path<GetActionParams>,
) -> Result<Json<GetFixedTransferResponse>> {
    println!("->> {:<12} - get_action - {params:?}", "HANDLER");

    let response = match params.action_id {
        0 => {
            let icon = String::from("https://url.com");
            let title = String::from("Send a fixed transfer");
            let description = String::from("This action allows you to send a fixed transfer");
            let label = String::from("Send");

            GetFixedTransferResponse {
                icon,
                title,
                description,
                label,
            }
        }
        _ => return Err(Error::ActionNotFound),
    };

    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
struct CreateActionParams {
    action_id: u32,
}


#[derive(Debug, Deserialize)]
struct CreateActionQuery {
    amount: u64,
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
    Query(query): Query<CreateActionQuery>,
    Path(params): Path<CreateActionParams>,
    Json(payload): Json<CreateActionPayload>,
) -> Result<Json<CreateFixedTransferResponse>> {
    println!(
        "->> {:<12} - create_action - {params:?} - {payload:?}",
        "HANDLER"
    );

    let account_pubkey = match Pubkey::from_str(&payload.account) {
        Ok(account_pubkey) => account_pubkey,
        _ => return Err(Error::InvalidAccountPubkey)
    };
    
    let response = match params.action_id {
        0 => {
            let mint_pubkey = Pubkey::from_str(&"4PYnraBJbdPXeMXdgL5k1m3TCcfNMaEWycvEQu2cteEV").unwrap();
            let receiver_pubkey = Pubkey::from_str(&"6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G").unwrap();
            let source_pubkey = spl_associated_token_account::get_associated_token_address(
                &account_pubkey,
                &mint_pubkey,
            );
            let destination_pubkey = spl_associated_token_account::get_associated_token_address(
                &receiver_pubkey,
                &mint_pubkey,
            );
            let transfer_instruction = match spl_token::instruction::transfer(
                &spl_token::ID,
                &source_pubkey,
                &destination_pubkey,
                &account_pubkey,
                &[&account_pubkey],
                query.amount,
            ) {
                Ok(transfer_instruction) => transfer_instruction,
                _ => return Err(Error::InvalidTransferInstruction)
            };
            let transaction_message = Message::new(&[transfer_instruction], None);
            let transaction: Transaction = Transaction::new_unsigned(transaction_message);
            let serialized_transaction = match serialize(&transaction) {
                Ok(serialized_transaction) => serialized_transaction,
                _ => return Err(Error::InvalidTransferInstruction)
            };
            let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);
            
            CreateFixedTransferResponse {
                transaction: encoded_transaction,
                message: None,
            }
        }
        _ => return Err(Error::ActionNotFound),
    };

    Ok(Json(response))
}
