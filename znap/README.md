<div align="center">
  <h1>znap</h1>

  <p>
    <strong>Znap framework's core library to create Solana actions</strong>
  </p>
</div>

`znap` is the core library that encompasses `znap-macros`, znap-syn and other essential components for Solana actions and Blinks programming.

`znap` is responsible for coordinating the different modules and tools needed to parse, transform and generate code in Rust. By integrating `znap-macros` and `znap-syn`, `znap` allows developers to take full advantage of Rust's capabilities to create Solana actions and Blinks.

## How to import znap

1. `cargo add znap`
2. In your lib.rs file import: `use znap::prelude::*`

## Package

| Package                 | Description                                              | Version                                                                                                                          | Docs                                                                                                            |
| :---------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `znap`           | Znap framework's core library to create Solana actions           | [![Crates.io](https://img.shields.io/crates/v/znap?color=blue)](https://crates.io/crates/znap)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap/latest/znap/)                                |

## How to use

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

#[derive(Action)]
#[action(
    icon = "https://google.com",
    title = "Dynamic transfer",
    description = "Send a dynamic transfer to the treasury",
    label = "Send"
)]
pub struct DynamicTransferAction;

#[query]
pub struct DynamicTransferQuery {
    pub amount: u64,
}

#[derive(ErrorCode)]
enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
    #[error(msg = "Invalid instruction")]
    InvalidInstruction,
}
```