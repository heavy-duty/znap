```rust
use solana_sdk::{message::Message, pubkey, pubkey::Pubkey, transaction::Transaction};
use spl_associated_token_account::get_associated_token_address;
use solana_program::system_instruction::transfer;
use std::str::FromStr;
use znap::prelude::*; 

#[collection]
pub mod my_actions {
    use super::*;

    pub fn fixed_donations_transfer(ctx: Context<FixedDonationsTransferAction>) -> Result<Transaction> {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => return Err(Error::from(ActionError::InvalidAccountPublicKey)),
        };

        let receiver_pubkey = pubkey!("6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G");

        let transfer_instruction = match transfer(&account_pubkey, &receiver_pubkey, ctx.query.param) {
            Ok(transfer_instruction) => transfer_instruction,
            _ => return Err(Error::from(ActionError::InvalidInstruction)),
        };
        let transaction_message = Message::new(&[transfer_instruction], None);

        Ok(Transaction::new_unsigned(transaction_message))
    }

    pub fn dynamic_donation_transfer(
        ctx: ContextWithQuery<DynamicDonationTransferAction, Donation>,
    ) -> Result<Transaction> {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => return Err(Error::from(ActionError::InvalidAccountPublicKey)),
        };

        let transfer_instruction = match transfer(&account_pubkey, &receiver_pubkey, ctx.query.amount) {
            Ok(transfer_instruction) => transfer_instruction,
            _ => return Err(Error::from(ActionError::InvalidInstruction)),
        };
        let transaction_message = Message::new(&[transfer_instruction], None);

        Ok(Transaction::new_unsigned(transaction_message))
    }
}

#[derive(Action)]
#[action(
    icon = "https://<icon-url>",
    title = "Alice's website",
    description = "Website to make a donation to Alice",
    label = "Send",
    link = {
        label = "1 SOL",
        href = "https://<api-url>?q={param}",
        parameter = { label = "1", name = "param" }
    },
    link = {
        label = "5 SOL",
        href = "https://<api-url>?q={param}",
        parameter = { label = "5", name = "param" }
    },
    link = {
        label = "5 SOL",
        href = "https://<api-url>?q={param}",
        parameter = { label = "10", name = "param" }
    },
)]
pub struct FixedDonationsTransferAction;

#[derive(Action)]
#[action(
    icon = "https://<icon-url>",
    title = "Alice's website",
    description = "Website to make a donation to Alice",
    label = "Send",
    link = { label = "Enter a custom SOL amount", href = "https://<api-url>" },
)]
pub struct DynamicDonationTransferAction;

#[query]
pub struct DynamicDonationTransferQuery {
    pub amount: u64,
}

#[derive(ErrorCode)]
enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
    #[error(msg = "Invalid instruction")]
    InvalidInstruction,
}
```