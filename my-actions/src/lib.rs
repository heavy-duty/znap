pub use action_derive::Action;
pub use collection_attribute::collection;
use serde::{Deserialize, Serialize};

// START OF BOILERPLATE
pub trait Action {
    fn to_metadata() -> ActionMetadata;
}

pub trait CreateTransaction {
    fn create_transaction() -> Result<String, Error>;
}

pub struct Context<T> {
    value: T,
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

#[derive(Debug, Serialize)]
pub struct ActionMetadata {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    label: &'static str,
}
// END OF BOILERPLATE

// START OF ACTUAL CODE
#[collection]
pub mod my_actions {
    use super::*;

    pub fn fixed_transfer2(_ctx: Context<FixedTransferAction>) -> Result<String, Error> {
        Ok(String::from("2"))
    }
}

#[derive(Action)]
#[action(
    icon = "https://google.com",
    title = "Fixed transfer",
    description = "Send a fixed transfer fee to the treasury",
    label = "Send"
)]
pub struct FixedTransferAction;

#[derive(Debug)]
pub enum Error {
    InvalidAccountPubkey,
    InvalidInstruction,
}
// END OF ACTUAL CODE

// START TESTING
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let fixed_transfer_action_metadata = FixedTransferAction::to_metadata();

        assert_eq!("Fixed transfer", fixed_transfer_action_metadata.title);
        
        let fixed_transfer_action_transaction = FixedTransferAction::create_transaction().unwrap();

        assert_eq!("2", fixed_transfer_action_transaction);

    }
}
// END TESTING