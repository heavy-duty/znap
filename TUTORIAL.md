# ZNAP Tutorial - **Donate/Pay a user**

## Description

Alice is a well-known influencer on Twitter. She usually shares research and valuable insights with her audience free of charge. She decides she wants to start a donation campaign for herself. She spins up a “Donate to me” Blink using the Actions Stack and shares the link to Sphere on Twitter.The link unfurls into an actionable Blink containing the following information:

1. Her image, name, description
2. 3 buttons: 1 SOL, 5 SOL, 10 SOL
3. 4th button with an input field: “Enter a custom SOL amount”

## Brief review

Solana Actions are APIs that adhere to the Solana Actions Specification. Our Solana Actions must be capable of returning:

1. Action Metadata
2. A transaction to be sent to the Solana blockchain.

## Let's Start!

First, we will create our workspace using `znap-cli`. If you do not have `znap-cli` installed, follow our guide [here](https://github.com/heavy-duty/znap/blob/master/INSTALLATION.md).

Next, let's create a new project with `znap init <project-name>`.

### 1. Let's create our action collection for Alice's campaign

In the root directory of your project, run the following command to create a collection: `znap new alice_campaign`. This command will create a new action collection called 'alice_campaign' inside the `/collections` folder, where we will place our actions for Alice.

You will then see a message like:

```bash
You are about to create a collection named: alice_campaign

  Added:

      + collections/alice_campaign/Cargo.toml
      + collections/alice_campaign/src/lib.rs

  Modified:

      * ./Znap.toml
```

Now, let's open our `lib.rs` file where we will create our actions. The file will look like this:

```rust
use znap::prelude::*;

#[collection]
pub mod alice_campaign {
    use super::*;
}
```

### 2. Let's Import All Necessary Libraries

We will import three necessary libraries:

- `solana-sdk`: The base library for off-chain programs that interact with Solana and its data structures.
- `std`: An essential collection of functionalities for working with Rust.
- `znap`: A framework for building APIs compatible with the Solana Actions specification.

```rust
use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey, pubkey::Pubkey,
    system_instruction::transfer, transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

#[collection]
pub mod alice_campaign {
    use super::*;
}
```

### 3. Set Up Our Solana Action's Metadata

We must set up all the necessary information to comply with the Solana Actions Specification. This is crucial because clients need to correctly retrieve all the metadata (`GET` Request) to create blinks using our Solana actions.

- **Title**: The action's title
- **Icon**: The action's icon
- **Description**: A brief description of the purpose of the action
- **Label**: The text for the main button shown to the user
- **Link(s)**: Buttons or inputs where the user will be able to get a transaction to be sent to the blockchain (`POST` Request)

We will use the macros `#[derive]`, `#[action]`, and the trait `Action` to create our Solana Action's metadata.

```rust
use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey, pubkey::Pubkey,
    system_instruction::transfer, transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

#[collection]
pub mod alice_campaign {
    use super::*;
}

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Alice's website",
    description = "Website to make a donation to Alice",
    label = "Send",
    link = {
        label = "Send 1 SOL",
        href = "/api/send_donation?amount=1",
    },
    link = {
        label = "Send 5 SOL",
        href = "/api/send_donation?amount=5",
    },
    link = {
        label = "Send 10 SOL",
        href = "/api/send_donation?amount=10",
    },
    link = {
        label = "Send SOL",
        href = "/api/send_donation?amount={amount}",
        parameter = { label = "Enter a custom SOL amount", name = "amount" }
    },
)]

pub struct SendDonationAction;
```

### 4. Define Query Parameters and Custom Errors

In this step, we will define the query parameters for our function, which we will create in the next step. Similar to Anchor contexts, our query parameters allow our function to understand what information to work with in order to complete its purpose.

We use the macro `#[query]` to define query parameter structs and their expected data.

```rust
#[query]
pub struct SendDonationQuery {
    pub amount: u64,
}
```

The above struct is our query parameter for our function. This query parameter will allow us to access the amount the user wishes to donate.

Additionally, we will create a custom error message for our function in case the requester's public key is incorrect.

```rust
#[derive(ErrorCode)]
enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
}
```

At this point, you should have something like this:

```rust
use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey, pubkey::Pubkey,
    system_instruction::transfer, transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

#[collection]
pub mod alice_campaign {
    use super::*;
}

#[derive(Action)]
#[action(
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Alice's website",
    description = "Website to make a donation to Alice",
    label = "Send",
    link = {
        label = "Send 1 SOL",
        href = "/api/send_donation?amount=1",
    },
    link = {
        label = "Send 5 SOL",
        href = "/api/send_donation?amount=5",
    },
    link = {
        label = "Send 10 SOL",
        href = "/api/send_donation?amount=10",
    },
    link = {
        label = "Send SOL",
        href = "/api/send_donation?amount={amount}",
        parameter = { label = "Enter a custom SOL amount", name = "amount" }
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

### 5. Create a Transaction to be Sent

In this step, we will create our function `send_donation`, which will generate a transaction to be sent to the blockchain.

First, we declare our function with the parameter it will use, which we defined in the previous step.

```rust
    pub fn send_donation(
        ctx: Context<SendDonationAction, SendDonationQuery>
    ) -> Result<Transaction> {

    }
```

Next, we obtain and validate the applicant's public key. If it is not valid, we return an error.

```rust
    pub fn send_donation(
        ctx: Context<SendDonationAction, SendDonationQuery>
    ) -> Result<Transaction> {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => return Err(Error::from(ActionError::InvalidAccountPublicKey)),
        };
    }
```

If the public key is valid, we set the public key of the receiver (Alice) and create the instructions for our transfer transaction.

To create the instructions, we use `ctx.query.amount` to get the amount specified by the requester (client).

```rust
    pub fn send_donation(
        ctx: Context<SendDonationAction, SendDonationQuery>
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
    }
```

Finally, we create a message with our instructions to indicate that everything went well.

Then, we create and return an unsigned transaction with the instructions we generated.

```rust
    pub fn send_donation(
        ctx: Context<SendDonationAction, SendDonationQuery>
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
```

And that’s it! We did it.

Your final project should look like this:

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
        ctx: Context<SendDonationAction, SendDonationQuery>
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
    icon = "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title = "Alice's website",
    description = "Website to make a donation to Alice",
    label = "Send",
    link = {
        label = "Send 1 SOL",
        href = "/api/send_donation?amount=1",
    },
    link = {
        label = "Send 5 SOL",
        href = "/api/send_donation?amount=5",
    },
    link = {
        label = "Send 10 SOL",
        href = "/api/send_donation?amount=10",
    },
    link = {
        label = "Send SOL",
        href = "/api/send_donation?amount={amount}",
        parameter = { label = "Enter a custom SOL amount", name = "amount" }
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

🎉 CONGRATULATIONS, YOU HAVE CREATED YOUR FIRST SOLANA ACTION WITH ZNAP 🎉

### 6. Let's Test Our Action!

In the root of your terminal, run `znap serve`.

This will start building your project, and once finished, it will start a server for your actions to be consumed.

If everything went well, you should see something like this:

```bash
✨ Znap Server ✨

 Service is running at http://localhost:3000

[my_actions] endpoints:

  GET      /api/send_donation
  POST     /api/send_donation

💡 Press Ctrl+C to stop the server
```

Now, if you consume your endpoint with a `GET` method, you will receive the metadata of your Solana Action.

![GET response](image.png)

And if you consume your endpoint with a `POST` method, setting the query parameter `amount` to 1 or another number, you will receive an unsigned Solana transaction ready to be sent to the blockchain.

![POST response](image-1.png)

