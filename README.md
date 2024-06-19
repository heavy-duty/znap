<div align="center">
  <h1>Znap</h1>

  <p>
    <strong>Performance-first Rust Framework to build APIs compatible with the Solana Actions Spec.</strong>
  </p>

  <p>
    <a href="https://github.com/heavy-duty/znap/tree/master/examples/my-actions"><img alt="Tutorials" src="https://img.shields.io/badge/docs-tutorials-blue" /></a>
  </p>
</div>

Znap is an innovative Rust-based framework designed to simplify the creation of [Solana Actions and Blinks](#) on the Solana blockchain.

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
use solana_sdk::{message::Message, pubkey, pubkey::Pubkey, transaction::Transaction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{instruction::transfer, ID as TOKEN_PROGRAM_ID};
use std::str::FromStr;
use znap::prelude::*;

#[collection]
pub mod my_actions {
    use super::*;

    pub fn fixed_transfer(ctx: Context<FixedTransferAction>) -> Result<Transaction> {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => return Err(Error::from(ActionError::InvalidAccountPublicKey)),
        };
        let mint_pubkey = pubkey!("FtaDaiPPAy52vKtzdrpMLS3bXvG9LVUYJt6TeG6XxMUi");
        let receiver_pubkey = pubkey!("6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G");
        let source_pubkey = get_associated_token_address(&account_pubkey, &mint_pubkey);
        let destination_pubkey = get_associated_token_address(&receiver_pubkey, &mint_pubkey);
        let transfer_instruction = match transfer(
            &spl_token::ID,
            &source_pubkey,
            &destination_pubkey,
            &account_pubkey,
            &[&account_pubkey],
            1,
        ) {
            Ok(transfer_instruction) => transfer_instruction,
            _ => return Err(Error::from(ActionError::InvalidInstruction)),
        };
        let transaction_message = Message::new(&[transfer_instruction], None);

        Ok(Transaction::new_unsigned(transaction_message))
    }
}

#[derive(Action)]
#[action(
    icon = "https://google.com",
    title = "Fixed transfer",
    description = "Send a fixed transfer to the treasury",
    label = "Send"
)]
pub struct FixedTransferAction;

#[derive(ErrorCode)]
enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
    #[error(msg = "Invalid instruction")]
    InvalidInstruction,
}
```