pub use action_derive::Action;
use base64::prelude::*;
use bincode::serialize;
pub use collection_attribute::collection_attribute_macro;
use serde::{Deserialize, Serialize};
use solana_sdk::{message::Message, pubkey::Pubkey, transaction::Transaction};
use std::str::FromStr;


pub trait Action {
    fn icon(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn label(&self) -> &'static str;
}

pub trait CreateTransaction {
    fn create_transaction(&self) -> &'static str;
}

pub struct Context<T> {
    value: T,
}

#[derive(Action)]
#[action(
    icon = "https://google.com",
    title = "Fixed transfer",
    description = "Send a fixed transfer fee to the treasury",
    label = "Send"
)]
pub struct FixedTransferAction;

#[derive(Debug, Serialize)]
pub struct ActionMetadata {
    icon: String,
    title: String,
    description: String,
    label: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateActionPayload {
    account: String,
}

#[derive(Debug, Serialize)]
pub struct CreateActionResponse {
    transaction: String,
    message: Option<String>,
}

#[collection_attribute_macro]
pub mod my_actions {
    use super::*;

    pub fn fixed_transfer2(_ctx: Context<FixedTransferAction>) -> Result<String, Error> {
        

        Ok("".to_string())
    }
}

pub enum Error {
    InvalidAccountPubkey,
    InvalidInstruction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let fixed_transfer_action = FixedTransferAction {};
        
        assert_eq!("Fixed transfer", fixed_transfer_action.title());
        
    }
}
