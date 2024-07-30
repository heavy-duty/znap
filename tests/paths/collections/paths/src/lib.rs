use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, system_instruction::transfer,
    transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

#[collection]
pub mod paths {
    use super::*;

    pub fn paths(ctx: Context<PathsAction>) -> Result<ActionTransaction> {
        let account_pubkey = Pubkey::from_str(&ctx.payload.account)
            .or_else(|_| Err(Error::from(ActionError::InvalidAccountPublicKey)))?;
        let receiver_pubkey = Pubkey::from_str(&ctx.params.receiver_address)
            .or_else(|_| Err(Error::from(ActionError::InvalidReceiverPublicKey)))?;
        let transfer_instruction = transfer(
            &account_pubkey,
            &receiver_pubkey,
            ctx.query.amount * LAMPORTS_PER_SOL,
        );
        let transaction_message = Message::new(&[transfer_instruction], None);
        let transaction = Transaction::new_unsigned(transaction_message);

        Ok(ActionTransaction {
            transaction,
            message: Some("send donation to alice".to_string()),
        })
    }
}

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Send a Donation to {{params.receiver_address}}",
    description = "Send a donation to {{params.receiver_address}} using the Solana blockchain via a Blink.",
    label = "Send",
    path = "{{prefix}}/v1/test/{{action_name}}",
    link = {
        label = "Send 1 SOL",
        href = "?amount=1",
    },
    link = {
        label = "Send 5 SOL",
        href = "?amount=5",
    },
    link = {
        label = "Send SOL",
        href = "http://localhost:3000/api/send_donation?amount={amount}",
        parameter = { label = "Amount in SOL", name = "amount" }
    },
)]
#[query(amount: u64)]
#[params(receiver_address: String)]
pub struct PathsAction;

#[derive(ErrorCode)]
enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
    #[error(msg = "Invalid receiver public key")]
    InvalidReceiverPublicKey,
}
