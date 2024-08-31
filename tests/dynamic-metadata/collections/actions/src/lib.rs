use solana_sdk::{message::Message, transaction::Transaction};
use znap::prelude::*;

#[collection]
pub mod actions {
    use super::*;

    pub fn dynamic_metadata_action_get(
        ctx: Context<DynamicMetadataAction>,
    ) -> Result<ActionMetadata> {
        let label = "Send";
        let description = "An action with dynamic metadata";
        let title = "Dynamic Metadata Action";
        let icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660";

        Ok(ActionMetadata {
            title: title.to_string(),
            description: description.to_string(),
            icon: icon.to_string(),
            label: label.to_string(),
            disabled: false,
            error: None,
            links: None,
        })
    }

    pub fn dynamic_metadata_action_post(
        ctx: Context<DynamicMetadataAction>,
    ) -> Result<ActionTransaction> {
        Ok(ActionTransaction {
            transaction: Transaction::new_unsigned(Message::new(&[], None)),
            message: Some("action with dynamic metadata".to_string()),
        })
    }
}

#[derive(Action)]
#[query(amount: u64)]
pub struct DynamicMetadataAction;
