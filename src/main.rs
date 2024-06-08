use std::str::FromStr;

pub use self::error::{Error, Result};

use axum::{extract::Query, routing::get, Json, Router};
use base64::prelude::*;
use bincode::serialize;
use serde::{Deserialize, Serialize};
use solana_sdk::{message::Message, pubkey::Pubkey, transaction::Transaction};
use tokio::net::TcpListener;

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
    Router::new()
        .route(
            "/actions/fixed_transfer",
            get(handle_get_fixed_transfer).post(handle_create_fixed_transfer),
        )
        .route(
            "/actions/dynamic_transfer",
            get(handle_get_dynamic_transfer).post(handle_create_dynamic_transfer),
        )
}

#[derive(Debug, Serialize)]
struct ActionMetadata {
    icon: String,
    title: String,
    description: String,
    label: String,
}

#[derive(Debug, Deserialize)]
struct CreateActionPayload {
    account: String,
}

#[derive(Debug, Serialize)]
struct CreateActionResponse {
    transaction: String,
    message: Option<String>,
}

// e.g., `/actions/fixed_transfer`
async fn handle_get_fixed_transfer() -> Result<Json<ActionMetadata>> {
    println!("->> get_fixed_transfer");

    Ok(Json(FixedTransferAction::to_metadata()))
}

// e.g., `/actions/fixed_transfer`
async fn handle_create_fixed_transfer(
    Json(payload): Json<CreateActionPayload>,
) -> Result<Json<CreateActionResponse>> {
    println!("->> - create_fixed_transfer - {payload:?}");

    Ok(Json(CreateActionResponse {
        transaction: FixedTransferAction::create_transaction(FixedTransferParams {
            account: payload.account,
        })?,
        message: None,
    }))
}

// e.g., `/actions/dynamic_transfer`
async fn handle_get_dynamic_transfer() -> Result<Json<ActionMetadata>> {
    println!("->> get_dynamic_transfer");

    Ok(Json(DynamicTransferAction::to_metadata()))
}

#[derive(Debug, Deserialize)]
struct CreateDynamicTransferQuery {
    amount: u64,
}

// e.g., `/actions/dynamic_transfer`
async fn handle_create_dynamic_transfer(
    Query(query): Query<CreateDynamicTransferQuery>,
    Json(payload): Json<CreateActionPayload>,
) -> Result<Json<CreateActionResponse>> {
    println!("->> - create_dynamic_transfer - {payload:?}");

    Ok(Json(CreateActionResponse {
        transaction: DynamicTransferAction::create_transaction(DynamicTransferParams {
            account: payload.account,
            amount: query.amount,
        })?,
        message: None,
    }))
}

trait Action<T> {
    fn create_transaction(params: T) -> Result<String>;

    fn to_metadata() -> ActionMetadata;
}

struct DynamicTransferParams {
    account: String,
    amount: u64,
}

struct DynamicTransferAction {}

impl Action<DynamicTransferParams> for DynamicTransferAction {
    fn to_metadata() -> ActionMetadata {
        let icon = String::from("https://url.com");
        let title = String::from("Send a dynamic transfer");
        let description = String::from("This action allows you to send a dynamic transfer");
        let label = String::from("Send");

        ActionMetadata {
            icon,
            title,
            description,
            label,
        }
    }

    fn create_transaction(params: DynamicTransferParams) -> Result<String> {
        let account_pubkey = match Pubkey::from_str(&params.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => return Err(Error::InvalidAccountPubkey),
        };
        let mint_pubkey =
            Pubkey::from_str(&"4PYnraBJbdPXeMXdgL5k1m3TCcfNMaEWycvEQu2cteEV").unwrap();
        let receiver_pubkey =
            Pubkey::from_str(&"6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G").unwrap();
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
            params.amount,
        ) {
            Ok(transfer_instruction) => transfer_instruction,
            _ => return Err(Error::InvalidInstruction),
        };
        let transaction_message = Message::new(&[transfer_instruction], None);
        let transaction: Transaction = Transaction::new_unsigned(transaction_message);
        let serialized_transaction = match serialize(&transaction) {
            Ok(serialized_transaction) => serialized_transaction,
            _ => return Err(Error::InvalidInstruction),
        };

        Ok(BASE64_STANDARD.encode(serialized_transaction))
    }
}

struct FixedTransferParams {
    account: String,
}

struct FixedTransferAction {}

impl Action<FixedTransferParams> for FixedTransferAction {
    fn to_metadata() -> ActionMetadata {
        let icon = String::from("https://url.com");
        let title = String::from("Send a fixed transfer");
        let description = String::from("This action allows you to send a fixed transfer");
        let label = String::from("Send");

        ActionMetadata {
            icon,
            title,
            description,
            label,
        }
    }

    fn create_transaction(params: FixedTransferParams) -> Result<String> {
        let account_pubkey = match Pubkey::from_str(&params.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => return Err(Error::InvalidAccountPubkey),
        };
        let mint_pubkey =
            Pubkey::from_str(&"4PYnraBJbdPXeMXdgL5k1m3TCcfNMaEWycvEQu2cteEV").unwrap();
        let receiver_pubkey =
            Pubkey::from_str(&"6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G").unwrap();
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
            1,
        ) {
            Ok(transfer_instruction) => transfer_instruction,
            _ => return Err(Error::InvalidInstruction),
        };
        let transaction_message = Message::new(&[transfer_instruction], None);
        let transaction: Transaction = Transaction::new_unsigned(transaction_message);
        let serialized_transaction = match serialize(&transaction) {
            Ok(serialized_transaction) => serialized_transaction,
            _ => return Err(Error::InvalidInstruction),
        };

        Ok(BASE64_STANDARD.encode(serialized_transaction))
    }
}
