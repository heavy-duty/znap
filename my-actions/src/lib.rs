pub use action_derive::Action;
pub use collection_attribute::collection_attribute_macro;
use serde::{Deserialize, Serialize};

pub trait Action {
    fn icon(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn label(&self) -> &'static str;
}

pub trait CreateTransaction {
    fn create_transaction(&self) -> Result<String, Error>;
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
        Ok(String::from("2"))
    }
}

#[derive(Debug)]
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
        
        let transaction = fixed_transfer_action.create_transaction().unwrap();

        assert_eq!("2", transaction);
    }
}
