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

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use solana_sdk::transaction::Transaction;
use std::marker::PhantomData;
pub extern crate bincode;
pub extern crate base64;
pub extern crate znap_macros;
pub extern crate colored;

pub mod prelude;

/// Trait used to transform a struct into an Action.
pub trait Action {}

/// Trait used to transform a struct into an error code.
pub trait ErrorCode {}

/// Used to rename Resolve and limit errors to those that occur within the program.
pub type Result<T> = core::result::Result<T, Error>;

/// Allows a struct to capture its internal values and return them as an ActionMetadata interface.
pub trait ToMetadata {
    fn to_metadata(&self) -> ActionMetadata;
}

/// Allows a struct to create a transaction.
pub trait CreateTransaction<T> {
    fn create_transaction(&self, ctx: Context<T>) -> Result<Transaction>;
}

/// Allows a struct to create a transaction that includes query parameters.
pub trait CreateTransactionWithQuery<T, U> {
    fn create_transaction(&self, ctx: ContextWithQuery<T, U>) -> Result<Transaction>;
}

/// Allows access to the methods and other values defined within the Action.
pub struct Context<TAction> {
    pub payload: CreateActionPayload,
    pub action: PhantomData<TAction>,
}

/// Allows access to the methods and other values defined within the Action for requests that include query parameters.
pub struct ContextWithQuery<TAction, TQuery> {
    pub payload: CreateActionPayload,
    pub action: PhantomData<TAction>,
    pub query: TQuery,
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
