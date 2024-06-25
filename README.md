<div align="center">
  
  [![Watch the demo](https://res.cloudinary.com/andresmgsl/image/upload/q_auto/f_auto/w_450/v1718845551/ZNAP_cuckvf.png)](https://youtu.be/pmuwP9fWa3M)
  <h1>Znap</h1>
  
  [Watch the demo](https://youtu.be/pmuwP9fWa3M)
  <p>
    <strong>Performance-first Rust Framework to build APIs compatible with the Solana Actions Spec.</strong>
  </p>

  <p>
    <a href="https://github.com/heavy-duty/znap/tree/master/examples/my-actions"><img alt="Tutorials" src="https://img.shields.io/badge/docs-tutorials-blue" /></a>
  </p>
</div>

Znap is an innovative Rust-based framework designed to simplify the creation of [Solana Actions](#) on the Solana blockchain.

- Rust eDSL for writing Solana actions
- Macro collection
- CLI and workspace management for developing complete Solana actions

If you're familiar with developing using the Anchor framework, then the experience will be familiar.

## Getting Started

1. `cargo install znap-cli`
2. `znap init <my-project-name>`
3. `cd <my-project-name>`
4. `znap new <collection-name>`

## Packages

| Package                 | Description                                              | Version                                                                                                                          | Docs                                                                                                            |
| :---------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `znap`           | Znap framework's core library to create Solana actions           | [![Crates.io](https://img.shields.io/crates/v/znap?color=blue)](https://crates.io/crates/znap)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap/latest/znap/)                                |
| `znap-syn`           | Parsing and generating code for macros in Rust           | [![Crates.io](https://img.shields.io/crates/v/znap-syn?color=blue)](https://crates.io/crates/znap-syn)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap-syn/latest/znap_syn/)                                |
| `znap-macros`           | Macro collection for creating Solana actions           | [![Crates.io](https://img.shields.io/crates/v/znap-macros?color=blue)](https://crates.io/crates/znap-macros)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap-macros/latest/znap_macros/)                                |
| `znap-cli`           | Znap CLI to interact with a znap workspace.           | [![Crates.io](https://img.shields.io/crates/v/znap-cli?color=blue)](https://crates.io/crates/znap-cli)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap-cli/latest/znap_cli/)                                |

## Example

```rust
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
    icon = "https://<icon-url>",
    title = "Alice's website",
    description = "Website to make a donation to Alice",
    label = "Send",
    link = {
        label = "Send 1 SOL",
        href = "https://<api-url>?amount=1",
    },
    link = {
        label = "Send 5 SOL",
        href = "https://<api-url>?amount=5",
    },
    link = {
        label = "Custom Donation",
        href = "https://<api-url>?amount={amount}",
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
```
