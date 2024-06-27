use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey, pubkey::Pubkey,
    system_instruction::transfer, transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

#[collection]
pub mod my_actions {
    use super::*;

    pub fn send_donation(
        ctx: ContextWithQuery<SendDonationAction, SendDonationQuery>,
    ) -> Result<Transaction> {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => return Err(Error::from(ActionError::InvalidAccountPublicKey)),
        };
        let receiver_pubkey = pubkey!("6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G");
        let transfer_instruction = transfer(
            &account_pubkey,
            &receiver_pubkey,
            ctx.query.amount * LAMPORTS_PER_SOL,
        );
        let transaction_message = Message::new(&[transfer_instruction], None);

        Ok(Transaction::new_unsigned(transaction_message))
    }
}

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Alice's website",
    description = "Website to make a donation to Alice",
    label = "Send",
    link = {
        label = "Send 1 SOL",
        href = "/api/send_donation?amount=1",
    },
    link = {
        label = "Send 5 SOL",
        href = "/api/send_donation?amount=5",
    },
    link = {
        label = "Send SOL",
        href = "/api/send_donation?amount={amount}",
        parameter = { label = "Amount in SOL", name = "amount" }
    },
)]
pub struct SendDonationAction;

#[query]
pub struct SendDonationQuery {
    pub amount: u64,
}

#[derive(ErrorCode)]
enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
}
