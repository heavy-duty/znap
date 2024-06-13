use action::*;
use action_derive::Action;
use base64::prelude::*;
use collection_attribute::collection;
use serde::Deserialize;
use solana_sdk::{message::Message, pubkey::Pubkey, transaction::Transaction};
use std::{marker::PhantomData, str::FromStr};

// START OF ACTUAL CODE
#[collection]
pub mod my_actions {
    use super::*;

    pub fn fixed_transfer(ctx: Context<FixedTransferAction>) -> Result<Transaction, Error> {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
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

        Ok(Transaction::new_unsigned(transaction_message))
    }

    pub fn dynamic_transfer(
        ctx: ContextWithQuery<DynamicTransferAction, DynamicTransferQuery>,
    ) -> Result<Transaction, Error> {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
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
            ctx.query.amount,
        ) {
            Ok(transfer_instruction) => transfer_instruction,
            _ => return Err(Error::InvalidInstruction),
        };
        let transaction_message = Message::new(&[transfer_instruction], None);

        Ok(Transaction::new_unsigned(transaction_message))
    }
}

#[derive(Action)]
#[action(
    icon = "https://google.com",
    title = "Fixed transfer",
    description = "Send a fixed transfer to the treasury",
    label = "Send"
)]
pub struct FixedTransferAction;

#[derive(Action)]
#[action(
    icon = "https://google.com",
    title = "Dynamic transfer",
    description = "Send a dynamic transfer to the treasury",
    label = "Send"
)]
pub struct DynamicTransferAction;

#[derive(Deserialize)]
pub struct DynamicTransferQuery {
    pub amount: u64,
}
// END OF ACTUAL CODE

// START TESTING
#[cfg(test)]
mod tests {
    use axum::{extract::Query, http::Uri, Json};

    use super::*;

    #[test]
    fn it_handles_fixed_transfer_action() {
        let action = FixedTransferAction {};
        let action_metadata = action.to_metadata();

        assert_eq!("Fixed transfer", action_metadata.title);

        let action_transaction =
            FixedTransferAction::handle_post_action(Json::from(CreateActionPayload {
                account: "4PYnraBJbdPXeMXdgL5k1m3TCcfNMaEWycvEQu2cteEV".to_string(),
            }))
            .unwrap();

        assert_eq!("AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAQEEMlnIyV1k2VNRqM4x48htBRRy5jUZ2umQgMwoQ53uf4q5cX+QxKq3dF2j8lUSI+G9tMrUBw/nxQWe4oaNVv7qhPxCeH+W3dRh/wUfr48nA/12tCHT4rv2+H/cXKS0IZgdBt324ddloZPZy+FGzut5rBy0he1fWzeROoz1hX7/AKkAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEDBAECAAAJAwEAAAAAAAAA", action_transaction.transaction);
    }

    #[test]
    fn it_handles_dynamic_transfer_action() {
        let action = DynamicTransferAction {};
        let action_metadata = action.to_metadata();

        // DynamicTransferAction.create_transaction(ctx);
        assert_eq!("Dynamic transfer", action_metadata.title);

        let payload = Json::from(CreateActionPayload {
            account: "4PYnraBJbdPXeMXdgL5k1m3TCcfNMaEWycvEQu2cteEV".to_string(),
        });
        let uri = Uri::from_str("http://example.com/path?amount=5").unwrap();
        let query: Query<DynamicTransferQuery> = Query::try_from_uri(&uri).unwrap();
        let action_transaction = DynamicTransferAction::handle_post_action(payload, query).unwrap();

        assert_eq!("AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAQEEMlnIyV1k2VNRqM4x48htBRRy5jUZ2umQgMwoQ53uf4q5cX+QxKq3dF2j8lUSI+G9tMrUBw/nxQWe4oaNVv7qhPxCeH+W3dRh/wUfr48nA/12tCHT4rv2+H/cXKS0IZgdBt324ddloZPZy+FGzut5rBy0he1fWzeROoz1hX7/AKkAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEDBAECAAAJAwUAAAAAAAAA", action_transaction.transaction);
    }
}
// END TESTING
