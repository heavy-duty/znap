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

pub trait Action {}

pub trait ErrorCode {}

pub type Result<T> = core::result::Result<T, Error>;

pub trait ToMetadata {
    fn to_metadata(&self) -> ActionMetadata;
}

pub trait CreateTransaction<T> {
    fn create_transaction(&self, ctx: Context<T>) -> Result<Transaction>;
}

pub trait CreateTransactionWithQuery<T, U> {
    fn create_transaction(&self, ctx: ContextWithQuery<T, U>) -> Result<Transaction>;
}

pub struct Context<TAction> {
    pub payload: CreateActionPayload,
    pub action: PhantomData<TAction>,
}

pub struct ContextWithQuery<TAction, TQuery> {
    pub payload: CreateActionPayload,
    pub action: PhantomData<TAction>,
    pub query: TQuery,
}

#[derive(Debug, Deserialize)]
pub struct CreateActionPayload {
    pub account: String,
}

#[derive(Debug, Serialize)]
pub struct ActionTransaction {
    pub transaction: String,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ActionMetadata {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub label: &'static str,
}

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
