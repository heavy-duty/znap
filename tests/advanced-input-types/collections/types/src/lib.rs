use znap::prelude::*;
use solana_sdk::{ message::Message, transaction::Transaction };

#[collection]
pub mod types {
    use super::*;

    pub fn types(ctx: Context<TypesAction>) -> Result<ActionTransaction> {
        let message = Message::new(&[], None);
        let transaction = Transaction::new_unsigned(message);
        Ok(ActionTransaction {
            transaction,
            message: Some("Restake successfully made!".to_string()),
        })
    }
}
 
#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Advanced Input Types",
    description = "An action with advanced input types.",
    label = "Send",
    link = {
        label = "Send",
        href = "/api/types",
        parameter = { label = "Text", name = "text", type = "text" },
        parameter = { label = "Email", name = "email", type = "email" },
        parameter = { label = "Url", name = "url", type = "url" },
        parameter = { label = "Number", name = "number", type = "number" },
        parameter = { label = "Date", name = "date", type = "date" },
        parameter = { label = "Datetime local", name = "datetime-local", type = "datetime-local" },
        parameter = { label = "Checkbox", name = "checkbox", type = "checkbox" },
        parameter = { label = "Radio", name = "radio", type = "radio", 
            option = {
                label = "Radio 1",
                value = "radio-1",
            },
            option = {
                label = "Radio 2",
                value = "radio-2",
            },
        }, 
        parameter = { label = "Radio empty", name = "radio-empty", type = "radio"},
        parameter = { label = "Textarea", name = "textarea", type = "textarea" },
        parameter = { label = "Select", name = "select", type = "select",
            option = {
                label = "Select 1",
                value = "select-1",
            },
            option = {
                label = "Select 2",
                value = "select-2",
            }, 
        },
        parameter = { label = "Select empty", name = "select-empty", type = "select"},
    },
)]
pub struct TypesAction;
