use handlebars::Handlebars;
use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, system_instruction::transfer,
    transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

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
        let metadata = SendDonationAction::to_metadata();

        let mut handlebars = Handlebars::new();

        let title_source = metadata.title.clone();
        assert!(handlebars
            .register_template_string("title", &title_source)
            .is_ok());
        let title = handlebars.render("title", &ctx).unwrap();

        let description_source = metadata.description.clone();
        assert!(handlebars
            .register_template_string("description", &description_source)
            .is_ok());
        let description = handlebars.render("description", &ctx).unwrap();

        let label_source = metadata.label.clone();
        assert!(handlebars
            .register_template_string("label", &label_source)
            .is_ok());
        let label = handlebars.render("label", &ctx).unwrap();

        let icon_source = metadata.icon.clone();
        assert!(handlebars
            .register_template_string("icon", &icon_source)
            .is_ok());
        let icon = handlebars.render("icon", &ctx).unwrap();

        let links = match metadata.links {
            Some(ActionLinks { actions }) => {
                let linked_actions: Vec<LinkedAction> = actions
                    .iter()
                    .enumerate()
                    .map(|(action_index, link)| {
                        let label_source = link.label.clone();
                        assert!(handlebars
                            .register_template_string(
                                &format!("link-{}-label", action_index).to_string(),
                                &label_source
                            )
                            .is_ok());
                        let label = handlebars
                            .render(&format!("link-{}-label", action_index).to_string(), &ctx)
                            .unwrap();

                        let href_source = link.href.clone();
                        assert!(handlebars
                            .register_template_string(
                                &format!("link-{}-href", action_index).to_string(),
                                &href_source
                            )
                            .is_ok());
                        let href = handlebars
                            .render(&format!("link-{}-href", action_index).to_string(), &ctx)
                            .unwrap();

                        let parameters: Vec<LinkedActionParameter> = link
                            .parameters
                            .iter()
                            .enumerate()
                            .map(|(parameter_index, parameter)| {
                                let label_source = parameter.label.clone();
                                assert!(handlebars
                                    .register_template_string(
                                        &format!(
                                            "link-{}-parameter-{}-label",
                                            action_index, parameter_index
                                        )
                                        .to_string(),
                                        &label_source
                                    )
                                    .is_ok());
                                let label = handlebars
                                    .render(
                                        &format!(
                                            "link-{}-parameter-{}-label",
                                            action_index, parameter_index
                                        )
                                        .to_string(),
                                        &ctx,
                                    )
                                    .unwrap();

                                let name_source = parameter.name.clone();
                                assert!(handlebars
                                    .register_template_string(
                                        &format!(
                                            "link-{}-parameter-{}-name",
                                            action_index, parameter_index
                                        )
                                        .to_string(),
                                        &name_source
                                    )
                                    .is_ok());
                                let name = handlebars
                                    .render(
                                        &format!(
                                            "link-{}-parameter-{}-name",
                                            action_index, parameter_index
                                        )
                                        .to_string(),
                                        &ctx,
                                    )
                                    .unwrap();

                                LinkedActionParameter {
                                    label,
                                    name,
                                    required: parameter.required,
                                }
                            })
                            .collect();

                        LinkedAction {
                            label,
                            href,
                            parameters,
                        }
                    })
                    .collect();

                Some(ActionLinks {
                    actions: linked_actions,
                })
            }
            _ => None,
        };

        Ok(ActionMetadata {
            title,
            icon,
            description,
            label,
            links,
            disabled: false,
            error: None,
        })
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
