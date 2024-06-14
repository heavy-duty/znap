use axum::http::StatusCode;
use serde::Deserialize;
use solana_sdk::{message::Message, pubkey::Pubkey, transaction::Transaction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{instruction::transfer, ID as TOKEN_PROGRAM_ID};
use znap_lang::*;

#[collection]
pub mod my_actions {

    use super::*;

    pub fn fixed_transfer(ctx: Context<FixedTransferAction>) -> Result<Transaction, ActionError> {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => {
                return Err(ActionError::new(
                    StatusCode::BAD_REQUEST,
                    "Invalid account public key",
                ))
            }
        };
        let mint_pubkey =
            Pubkey::from_str(&"7jfe8e75i7dwiyYQVEhYUgyqWP3i96mFievsrAaVGw2n").unwrap();
        let receiver_pubkey =
            Pubkey::from_str(&"6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G").unwrap();
        let source_pubkey = get_associated_token_address(&account_pubkey, &mint_pubkey);
        let destination_pubkey = get_associated_token_address(&receiver_pubkey, &mint_pubkey);
        let transfer_instruction = match spl_token::instruction::transfer(
            &spl_token::ID,
            &source_pubkey,
            &destination_pubkey,
            &account_pubkey,
            &[&account_pubkey],
            1,
        ) {
            Ok(transfer_instruction) => transfer_instruction,
            _ => {
                return Err(ActionError::new(
                    StatusCode::BAD_REQUEST,
                    "Invalid instruction",
                ))
            }
        };
        let transaction_message = Message::new(&[transfer_instruction], None);

        Ok(Transaction::new_unsigned(transaction_message))
    }

    pub fn dynamic_transfer(
        ctx: ContextWithQuery<DynamicTransferAction, DynamicTransferQuery>,
    ) -> Result<Transaction, ActionError> {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => {
                return Err(ActionError::new(
                    StatusCode::BAD_REQUEST,
                    "Invalid account public key",
                ))
            }
        };
        let mint_pubkey =
            Pubkey::from_str(&"7jfe8e75i7dwiyYQVEhYUgyqWP3i96mFievsrAaVGw2n").unwrap();
        let receiver_pubkey =
            Pubkey::from_str(&"6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G").unwrap();
        let source_pubkey = get_associated_token_address(&account_pubkey, &mint_pubkey);
        let destination_pubkey = get_associated_token_address(&receiver_pubkey, &mint_pubkey);
        let transfer_instruction = match transfer(
            &TOKEN_PROGRAM_ID,
            &source_pubkey,
            &destination_pubkey,
            &account_pubkey,
            &[&account_pubkey],
            ctx.query.amount,
        ) {
            Ok(transfer_instruction) => transfer_instruction,
            _ => {
                return Err(ActionError::new(
                    StatusCode::BAD_REQUEST,
                    "Invalid instruction",
                ))
            }
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
