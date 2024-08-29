use solana_sdk::{
    message::Message, pubkey, pubkey::Pubkey, system_instruction::transfer,
    transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

#[collection]
pub mod actions {
    use super::*;

    pub fn custom_get_action(ctx: Context<GetAction>) -> Result<ActionMetadata> {
        let label = "Custom Get";
        let description = "Custom Get Action";
        let title = "Custom GET Action";

        Ok(ActionMetadata {
            title: title.to_string(),
            description: description.to_string(),
            icon: ctx.metadata.icon,
            label: label.to_string(),
            disabled: false,
            error: None,
            links: None,
        })
    }

    pub fn get_action(ctx: Context<GetAction>) -> Result<ActionTransaction> {
        /*

        This example creates a transaction to send 1 lamport from the *account* provided to the address
        *6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G*. You can replace this with your actual code.

         */
        let account_pubkey = Pubkey::from_str(&ctx.payload.account)
            .or_else(|_| Err(Error::from(ActionError::InvalidAccountPublicKey)))?;
        let receiver_pubkey = pubkey!("6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G");
        let transfer_instruction = transfer(&account_pubkey, &receiver_pubkey, 1);
        let transaction_message = Message::new(&[transfer_instruction], None);
        let transaction = Transaction::new_unsigned(transaction_message);

        Ok(ActionTransaction {
            transaction,
            message: Some("custom get action".to_string()),
        })
    }
}

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "GET Action",
    description = "Get Action",
    label = "Get"
)]
#[query(amount: u64)]
pub struct GetAction;

#[derive(ErrorCode)]
enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
}
