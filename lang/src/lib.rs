use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract::Query, Json};
pub use base64::prelude::*;
use serde::{Deserialize, Serialize};
use solana_sdk::transaction::Transaction;
pub use std::marker::PhantomData;
pub use znap_attribute_collection::collection;
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

pub trait HandleGetAction {
    fn handle_get_action() -> Result<Json<ActionMetadata>, ActionError>;
}

pub trait HandlePostAction {
    fn handle_post_action(
        payload: Json<CreateActionPayload>,
    ) -> Result<Json<ActionTransaction>, ActionError>;
}

pub trait HandlePostActionWithQuery<T> {
    fn handle_post_action(
        payload: Json<CreateActionPayload>,
        query: Query<T>,
    ) -> Result<Json<ActionTransaction>, ActionError>;
}

#[derive(Debug, Serialize)]
pub enum Error {
    InvalidAccountPubkey,
    InvalidInstruction,
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
