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
//! ```ignore
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
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::Message;
use solana_sdk::pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKey, Signer};
use solana_sdk::transaction::Transaction;
use std::env;
pub extern crate base64;
pub extern crate bincode;
pub extern crate colored;
pub extern crate tower_http;
pub extern crate znap_macros;

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
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateActionPayload {
    pub account: String,
}

/// Represents the data structure returned by the POST handlers.
#[derive(Debug, Serialize)]
pub struct ActionResponse {
    pub transaction: String,
    pub message: Option<String>,
}

/// Represents the data structure returned by a POST request to an endpoint of the Solana Actions API.
#[derive(Debug, Deserialize, Serialize)]
pub struct ActionTransaction {
    pub transaction: Transaction,
    pub message: Option<String>,
}

/// Represents the data structure returned by a GET request to an endpoint of the Solana Actions API.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ActionMetadata {
    pub icon: String,
    pub title: String,
    pub description: String,
    pub label: String,
    pub links: Option<ActionLinks>,
    pub disabled: bool,
    pub error: Option<ActionError>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ActionError {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ActionLinks {
    pub actions: Vec<LinkedAction>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LinkedAction {
    pub label: String,
    pub href: String,
    pub parameters: Vec<LinkedActionParameter>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LinkedActionParameter {
    pub label: String,
    pub name: String,
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

pub fn add_action_identity_proof(transaction: Transaction) -> Transaction {
    let identity_keypair =
        Keypair::read_from_file(env::var("IDENTITY_KEYPAIR_PATH").unwrap()).unwrap();
    let identity_pubkey = identity_keypair.pubkey();

    let reference_keypair = Keypair::new();
    let reference_pubkey = reference_keypair.pubkey();

    let identity_signature = identity_keypair.sign_message(&reference_pubkey.to_bytes());
    let identity_message = format!(
        "solana-action:{}:{}:{}",
        identity_pubkey, reference_pubkey, identity_signature
    );

    let mut identity_added = false;

    let mut instructions_with_identity: Vec<Instruction> = transaction
        .message
        .instructions
        .iter()
        .map(|instruction| {
            let program_id =
                transaction.message.account_keys[instruction.program_id_index as usize];

            let mut accounts: Vec<AccountMeta> = instruction
                .accounts
                .iter()
                .map(|account_index| {
                    let pubkey = transaction.message.account_keys[*account_index as usize];

                    match transaction
                        .message
                        .is_maybe_writable(*account_index as usize, None)
                    {
                        true => AccountMeta::new(
                            pubkey,
                            transaction.message.is_signer(*account_index as usize),
                        ),
                        false => AccountMeta::new_readonly(
                            pubkey,
                            transaction.message.is_signer(*account_index as usize),
                        ),
                    }
                })
                .collect();

            if !identity_added
                && program_id.to_string() != "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
            {
                accounts.push(AccountMeta::new_readonly(reference_pubkey, false));
                accounts.push(AccountMeta::new_readonly(identity_pubkey, false));

                identity_added = true;
            }

            Instruction {
                program_id,
                data: instruction.data.clone(),
                accounts,
            }
        })
        .collect();

    instructions_with_identity.push(Instruction {
        accounts: vec![],
        data: identity_message.as_bytes().to_vec(),
        program_id: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
    });

    let transaction_message_with_identity = Message::new(&instructions_with_identity, None);

    Transaction::new_unsigned(transaction_message_with_identity)
}

pub fn render_source<T>(source: &str, data: &T) -> String
where
    T: Serialize,
{
    let mut handlebars = Handlebars::new();

    assert!(handlebars
        .register_template_string("template", source)
        .is_ok());
    let output = handlebars.render("template", &data).unwrap();

    handlebars.clear_templates();

    output
}

pub fn render_parameters<T>(
    parameters: &[LinkedActionParameter],
    data: &T,
) -> Vec<LinkedActionParameter>
where
    T: Serialize,
{
    parameters
        .iter()
        .map(|parameter| {
            let name = render_source(&parameter.name, &data);
            let label = render_source(&parameter.label, &data);

            LinkedActionParameter {
                label,
                name,
                required: parameter.required,
            }
        })
        .collect()
}

pub fn render_action_links<T>(links: Option<&ActionLinks>, data: &T) -> Option<ActionLinks>
where
    T: Serialize,
{
    match links {
        Some(ActionLinks { actions }) => Some(ActionLinks {
            actions: actions
                .iter()
                .map(|link| {
                    let label = render_source(&link.label, &data);
                    let href = render_source(&link.href, &data);

                    LinkedAction {
                        label,
                        href,
                        parameters: render_parameters(&link.parameters, &data),
                    }
                })
                .collect(),
        }),
        _ => None,
    }
}

pub fn render_metadata<T>(
    metadata: &ActionMetadata,
    data: &T,
    disabled: bool,
    error: Option<ActionError>,
) -> ActionMetadata
where
    T: Serialize,
{
    let title = render_source(&metadata.title, &data);
    let description = render_source(&metadata.description, &data);
    let label = render_source(&metadata.label, &data);
    let icon = render_source(&metadata.icon, &data);
    let links = render_action_links(metadata.links.as_ref(), &data);

    ActionMetadata {
        title,
        icon,
        description,
        label,
        links,
        disabled,
        error,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub active: bool,
}
