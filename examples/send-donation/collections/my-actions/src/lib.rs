use handlebars::Handlebars;
use serde::Serialize;
use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, system_instruction::transfer,
    transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

fn render_source<T>(source: &String, data: &T) -> String
where
    T: Serialize,
{
    let mut handlebars = Handlebars::new();

    assert!(handlebars
        .register_template_string(&"template", &source)
        .is_ok());
    let output = handlebars.render(&"template", &data).unwrap();

    handlebars.clear_templates();

    output
}

fn render_parameters<T>(
    parameters: &Vec<LinkedActionParameter>,
    data: &T,
) -> Vec<LinkedActionParameter>
where
    T: Serialize,
{
    parameters
        .iter()
        .map(|parameter| {
            let name = render_source(&parameter.name, &data);
            let label = render_source(&parameter.label, &data);

            LinkedActionParameter {
                label,
                name,
                required: parameter.required,
            }
        })
        .collect()
}

fn render_action_links<T>(links: &Option<ActionLinks>, data: &T) -> Option<ActionLinks>
where
    T: Serialize,
{
    match links {
        Some(ActionLinks { actions }) => Some(ActionLinks {
            actions: actions
                .iter()
                .map(|link| {
                    let label = render_source(&link.label, &data);
                    let href = render_source(&link.href, &data);

                    LinkedAction {
                        label,
                        href,
                        parameters: render_parameters(&link.parameters, &data),
                    }
                })
                .collect(),
        }),
        _ => None,
    }
}

fn render_metadata<T>(
    metadata: &ActionMetadata,
    data: &T,
    disabled: bool,
    error: Option<znap::ActionError>,
) -> ActionMetadata
where
    T: Serialize,
{
    let title = render_source(&metadata.title, &data);
    let description = render_source(&metadata.description, &data);
    let label = render_source(&metadata.label, &data);
    let icon = render_source(&metadata.icon, &data);
    let links = render_action_links(&metadata.links, &data);

    ActionMetadata {
        title,
        icon,
        description,
        label,
        links,
        disabled,
        error,
    }
}

#[collection]
pub mod my_actions {
    use super::*;

    pub fn send_donation(ctx: Context<SendDonationAction>) -> Result<ActionTransaction> {
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

    pub fn get_send_donation(ctx: Context<SendDonationAction>) -> Result<ActionMetadata> {
        Ok(render_metadata(&SendDonationAction::to_metadata(), &ctx, false, None))
    }
}

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Send a Donation to {{params.receiver_address}}",
    description = "Send a donation to {{params.receiver_address}} using the Solana blockchain via a Blink.",
    label = "Send",
    link = {
        label = "Send 1 SOL",
        href = "/api/send_donation/{{params.receiver_address}}?amount=1",
    },
    link = {
        label = "Send 5 SOL",
        href = "/api/send_donation/{{params.receiver_address}}?amount=5",
    },
    link = {
        label = "Send SOL",
        href = "/api/send_donation/{{params.receiver_address}}?amount={amount}",
        parameter = { label = "Amount in SOL", name = "amount" }
    },
)]
#[query(amount: u64)]
#[params(receiver_address: String)]
pub struct SendDonationAction;

#[derive(ErrorCode)]
enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
    #[error(msg = "Invalid receiver public key")]
    InvalidReceiverPublicKey,
}
