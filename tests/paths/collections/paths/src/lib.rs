use solana_sdk::{message::Message, transaction::Transaction};
use znap::prelude::*;

#[collection]
pub mod paths {
    use super::*;

    pub fn custom_path(ctx: Context<CustomPathAction>) -> Result<ActionTransaction> {
        Ok(ActionTransaction {
            transaction: Transaction::new_unsigned(Message::new(&[], None)),
            message: Some("send donation to alice".to_string()),
        })
    }

    pub fn default_path(ctx: Context<DefaultPathAction>) -> Result<ActionTransaction> {
        Ok(ActionTransaction {
            transaction: Transaction::new_unsigned(Message::new(&[], None)),
            message: Some("send donation to alice".to_string()),
        })
    }
}

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Custom Path",
    description = "Use a custom path configuration",
    label = "Send"
)]
#[query(amount: u64)]
#[params(receiver_address: String)]
#[action_path(template = "{{prefix}}/v1/test/{{action_name}}")]
pub struct CustomPathAction;

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Default Path",
    description = "Use the default path configuration",
    label = "Send"
)]
#[query(amount: u64)]
#[params(receiver_address: String)]
pub struct DefaultPathAction;
