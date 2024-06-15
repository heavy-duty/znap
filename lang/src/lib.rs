use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
pub use base64::prelude::*;
pub use bincode;
use serde::{Deserialize, Serialize};
use solana_sdk::transaction::Transaction;
use std::marker::PhantomData;
pub use std::str::FromStr;
pub use znap_attribute_collection::collection;
pub use znap_attribute_query::query;
pub use znap_derive_action::Action;

pub trait Action {}

pub trait ToMetadata {
    fn to_metadata(&self) -> ActionMetadata;
}

pub trait CreateTransaction<T> {
    fn create_transaction(&self, ctx: Context<T>) -> Result<Transaction, ActionError>;
}

pub trait CreateTransactionWithQuery<T, U> {
    fn create_transaction(&self, ctx: ContextWithQuery<T, U>) -> Result<Transaction, ActionError>;
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
pub struct ActionError {
    code: StatusCode,
    message: String,
}

impl ActionError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for ActionError {
    fn into_response(self) -> Response {
        (
            self.code,
            Json(ErrorResponse {
                error: self.message.clone(),
            }),
        )
            .into_response()
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}
