use error::ActionError;
use hat_api::{fetch_hat, Hat};
use solana_sdk::{message::Message, pubkey, pubkey::Pubkey, transaction::Transaction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{instruction::transfer_checked, ID as TOKEN_PROGRAM_ID};
use std::str::FromStr;
use znap::prelude::*;

mod error;
mod hat_api;

const DESTINATION_PUBLIC_KEY: Pubkey = pubkey!("Fpb6uVk3tWrQ93og9WZm581s9Wge5BJPFAkbjS6nLzNJ");
const MINT_PUBLIC_KEY: Pubkey = pubkey!("5R5kzomKtVjciTSHEaSZ6RcgEGCzjZeQ7NnstVModK6Q");
const MINT_DECIMALS: u8 = 6;

#[collection]
pub mod my_actions {
    use super::*;

    fn buy_hat(ctx: Context<BuyHatAction>) -> Result<ActionTransaction> {
        let account_pubkey = Pubkey::from_str(&ctx.payload.account)
            .or_else(|_| Err(Error::from(ActionError::InvalidAccountPublicKey)))?;

        let Hat { price, title, .. } = fetch_hat(&ctx.params.hat_id).await?;
        let sender_pubkey = get_associated_token_address(&account_pubkey, &MINT_PUBLIC_KEY);
        let receiver_pubkey =
            get_associated_token_address(&DESTINATION_PUBLIC_KEY, &MINT_PUBLIC_KEY);
        let amount = price * 10_u64.pow(MINT_DECIMALS.into());

        let transfer_checked_instruction = transfer_checked(
            &TOKEN_PROGRAM_ID,
            &sender_pubkey,
            &MINT_PUBLIC_KEY,
            &receiver_pubkey,
            &account_pubkey,
            &[&account_pubkey],
            amount,
            MINT_DECIMALS,
        )
        .or_else(|_| Err(Error::from(ActionError::InvalidTransferInstruction)))?;

        let transaction_message = Message::new(&[transfer_checked_instruction], None);
        let transaction = Transaction::new_unsigned(transaction_message);
        let message = format!("Buy a gently used {} for only ${}", title, price);

        Ok(ActionTransaction {
            transaction,
            message: Some(message),
        })
    }

    fn get_buy_hat(ctx: Context<BuyHatAction>) -> Result<ActionMetadata> {
        let hat = fetch_hat(&ctx.params.hat_id).await?;
        let label = "Buy Now!";
        let description = format!("Buy a gently used {} for only ${}", hat.title, hat.price);

        Ok(ActionMetadata {
            title: hat.title,
            description: description.to_string(),
            icon: hat.image_url,
            label: label.to_string(),
            disabled: false,
            error: None,
            links: None,
        })
    }
}

#[derive(Action)]
#[params(hat_id: String)]
pub struct BuyHatAction;
