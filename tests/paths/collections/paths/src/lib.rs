use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, system_instruction::transfer,
    transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

#[collection]
pub mod paths {
    use super::*;

    pub fn custom_path_single_parameter(
        ctx: Context<CustomPathSingleParameterAction>,
    ) -> Result<ActionTransaction> {
        Ok(ActionTransaction {
            transaction: Transaction::new_unsigned(Message::new(&[], None)),
            message: Some("send donation to alice".to_string()),
        })
    }

    pub fn default_path_multi_parameter(
        ctx: Context<DefaultPathMultiParameterAction>,
    ) -> Result<ActionTransaction> {
        Ok(ActionTransaction {
            transaction: Transaction::new_unsigned(Message::new(&[], None)),
            message: Some("send donation to alice".to_string()),
        })
    }
}

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Custom Path with Single Paramater",
    description = "Use a custom path configuration with a single parameter",
    label = "Send",
    path = "{{prefix}}/v1/test/{{action_name}}"
)]
#[query(amount: u64)]
#[params(receiver_address: String)]
pub struct CustomPathSingleParameterAction;

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Default Path with Multiple Paramaters",
    description = "Use the default path configuration with multiple parameters",
    label = "Send"
)]
#[query(amount: u64)]
#[params(mint_address: String, receiver_address: String)]
pub struct DefaultPathMultiParameterAction;
