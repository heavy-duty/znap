//! [![Watch the demo](https://res.cloudinary.com/andresmgsl/image/upload/q_auto/f_auto/w_450/v1718845551/ZNAP_cuckvf.png)](https://youtu.be/pmuwP9fWa3M)
//!
//! Performance-first Rust Framework to build APIs compatible with the Solana Actions Spec.
//!
//! ## Features
//! - Creating Solana Actions Metadata interfaces
//! - Creating Solana Actions POST Requests Transactions with and without query params.
//! - Creating Solana Actions GET Requests Transactions with and without query params.
//!
//! ## How to import `znap`
//!
//! 1. `cargo add znap`
//! 2. In your lib.rs file import: `use znap::prelude::*`
//!
//! ## Znap ecosystem
//! - [`znap`](https://docs.rs/znap/latest/znap/)
//! - [`znap-syn`](https://docs.rs/znap-syn/latest/znap_syn/)
//! - [`znap-macros`](https://docs.rs/znap-macros/latest/znap_macros/)
//! - [`znap-cli`](https://docs.rs/znap-cli/latest/znap_cli/)
//! 
//! ## Example
//!
//! ```rust
//! use solana_sdk::{message::Message, pubkey, pubkey::Pubkey, transaction::Transaction};
//! use spl_associated_token_account::get_associated_token_address;
//! use spl_token::{instruction::transfer, ID as TOKEN_PROGRAM_ID};
//! use std::str::FromStr;
//! use znap::prelude::*;
//!
//! #[collection]
//! pub mod my_actions {
//!     use super::*;
//!
//!     pub fn fixed_transfer(ctx: Context<FixedTransferAction>) -> Result<Transaction> {
//!         let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
//!             Ok(account_pubkey) => account_pubkey,
//!             _ => return Err(Error::from(ActionError::InvalidAccountPublicKey)),
//!         };
//!         let mint_pubkey = pubkey!("FtaDaiPPAy52vKtzdrpMLS3bXvG9LVUYJt6TeG6XxMUi");
//!         let receiver_pubkey = pubkey!("6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G");
//!         let source_pubkey = get_associated_token_address(&account_pubkey, &mint_pubkey);
//!         let destination_pubkey = get_associated_token_address(&receiver_pubkey, &mint_pubkey);
//!         let transfer_instruction = match transfer(
//!             &TOKEN_PROGRAM_ID,
//!             &source_pubkey,
//!             &destination_pubkey,
//!             &account_pubkey,
//!             &[&account_pubkey],
//!             1,
//!         ) {
//!             Ok(transfer_instruction) => transfer_instruction,
//!             _ => return Err(Error::from(ActionError::InvalidInstruction)),
//!         };
//!         let transaction_message = Message::new(&[transfer_instruction], None);
//!
//!         Ok(Transaction::new_unsigned(transaction_message))
//!     }
//! }
//!
//! #[derive(Action)]
//! #[action(
//!     icon = "https://google.com",
//!     title = "Fixed transfer",
//!     description = "Send a fixed transfer to the treasury",
//!     label = "Send"
//! )]
//! pub struct FixedTransferAction;
//!
//! #[derive(ErrorCode)]
//! enum ActionError {
//!     #[error(msg = "Invalid account public key")]
//!     InvalidAccountPublicKey,
//!     #[error(msg = "Invalid instruction")]
//!     InvalidInstruction,
//! }
//! ```

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
pub extern crate bincode;
pub extern crate base64;
pub extern crate znap_macros;
pub extern crate colored;
pub extern crate tower_http;

pub mod prelude;

/// Trait used to transform a struct into an Action.
pub trait Action {}

/// Trait used to transform a struct into an error code.
pub trait ErrorCode {}

/// Used to rename Resolve and limit errors to those that occur within the program.
pub type Result<T> = core::result::Result<T, Error>;

/// Allows a struct to capture its internal values and return them as an ActionMetadata interface.
pub trait ToMetadata {
    fn to_metadata() -> ActionMetadata;
}

/// Data structure required to make a POST request to an endpoint of the Solana Actions API.
#[derive(Debug, Deserialize)]
pub struct CreateActionPayload {
    pub account: String,
}

/// Represents the data structure returned by a POST request to an endpoint of the Solana Actions API.
#[derive(Debug, Serialize)]
pub struct ActionTransaction {
    pub transaction: String,
    pub message: Option<String>,
}

/// Represents the data structure returned by a GET request to an endpoint of the Solana Actions API.
#[derive(Debug, Serialize, PartialEq)]
pub struct ActionMetadata {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub label: &'static str,
    pub links: &'static Option<ActionLinks>,
    pub disabled: bool,
    pub error: Option<ActionError>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ActionError {
    pub message: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ActionLinks {
    pub actions: &'static [LinkedAction],
}

#[derive(Debug, Serialize, PartialEq)]
pub struct LinkedAction {
    pub label: &'static str,
    pub href: &'static str,
    pub parameters: &'static [LinkedActionParameter],
}

#[derive(Debug, Serialize, PartialEq)]
pub struct LinkedActionParameter {
    pub label: &'static str,
    pub name: &'static str,
    pub required: bool,
}

/// Error occurred during the processing of the request.
#[derive(Debug)]
pub struct Error {
    pub code: StatusCode,
    pub name: String,
    pub message: String,
}

impl Error {
    pub fn new(code: StatusCode, name: String, message: impl Into<String>) -> Self {
        Self {
            code,
            name,
            message: message.into(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.code,
            Json(ErrorResponse {
                name: self.name.clone(),
                message: self.message.clone(),
            }),
        )
            .into_response()
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    name: String,
    message: String,
}
