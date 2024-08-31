use solana_sdk::{message::Message, transaction::Transaction};
use znap::prelude::*;

#[collection]
pub mod prefix {
    use super::*;

    pub fn custom_prefix(ctx: Context<CustomPrefixAction>) -> Result<ActionTransaction> {
        Ok(ActionTransaction {
            transaction: Transaction::new_unsigned(Message::new(&[], None)),
            message: Some("custom prefix".to_string()),
        })
    }

    pub fn empty_prefix(ctx: Context<EmptyPrefixAction>) -> Result<ActionTransaction> {
        Ok(ActionTransaction {
            transaction: Transaction::new_unsigned(Message::new(&[], None)),
            message: Some("empty prefix".to_string()),
        })
    }

    pub fn default_prefix(ctx: Context<DefaultPrefixAction>) -> Result<ActionTransaction> {
        Ok(ActionTransaction {
            transaction: Transaction::new_unsigned(Message::new(&[], None)),
            message: Some("default prefix".to_string()),
        })
    }
}

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Custom prefix",
    description = "An action with a custom prefix.",
    label = "Send"
)]
#[action_path(prefix = "v1-api")]
pub struct CustomPrefixAction;

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Empty prefix",
    description = "An action with an empty prefix.",
    label = "Send"
)]
#[action_path()]
pub struct EmptyPrefixAction;

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Default prefix",
    description = "An action with the default prefix.",
    label = "Send"
)]
pub struct DefaultPrefixAction;
