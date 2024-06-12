pub use action_derive::Action;
pub use collection_attribute::collection;
use serde::{Deserialize, Serialize};
use solana_sdk::{message::Message, pubkey::Pubkey, transaction::Transaction};
use std::str::FromStr;
use base64::prelude::*;
use bincode::serialize;

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
        let account_pubkey = match Pubkey::from_str(&"4PYnraBJbdPXeMXdgL5k1m3TCcfNMaEWycvEQu2cteEV") {
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
        let transaction = Transaction::new_unsigned(transaction_message);
        let serialized_transaction = match serialize(&transaction) {
            Ok(serialized_transaction) => serialized_transaction,
            _ => return Err(Error::InvalidInstruction),
        };
        let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);
        
        Ok(encoded_transaction)
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

#[derive(Debug, Serialize)]
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

        assert_eq!("AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAQEEMlnIyV1k2VNRqM4x48htBRRy5jUZ2umQgMwoQ53uf4q5cX+QxKq3dF2j8lUSI+G9tMrUBw/nxQWe4oaNVv7qhPxCeH+W3dRh/wUfr48nA/12tCHT4rv2+H/cXKS0IZgdBt324ddloZPZy+FGzut5rBy0he1fWzeROoz1hX7/AKkAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEDBAECAAAJAwEAAAAAAAAA", fixed_transfer_action_transaction);
    }
}
// END TESTING