use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    native_token::LAMPORTS_PER_SOL,
    pubkey,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction::transfer,
    transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

fn add_action_identity_proof(transaction: Transaction) -> Transaction {
    let reference_keypair = Keypair::new();
    let reference_pubkey = reference_keypair.pubkey();
    let identity_keypair = Keypair::new();
    let identity_pubkey = identity_keypair.pubkey();

    let identity_signature = identity_keypair.sign_message(&reference_pubkey.to_bytes());
    let identity_message = format!(
        "solana-action:{}:{}:{}",
        identity_pubkey.to_string(),
        reference_pubkey.to_string(),
        identity_signature.to_string()
    );

    let mut identity_added = false;

    let mut instructions_with_identity: Vec<Instruction> = transaction
        .message
        .instructions
        .iter()
        .map(|instruction| {
            let program_id =
                transaction.message.account_keys[instruction.program_id_index as usize];

            let mut accounts: Vec<AccountMeta> = instruction
                .accounts
                .iter()
                .map(|account_index| {
                    let pubkey = transaction.message.account_keys[*account_index as usize];

                    match transaction
                        .message
                        .is_maybe_writable(*account_index as usize, None)
                    {
                        true => AccountMeta::new(
                            pubkey,
                            transaction.message.is_signer(*account_index as usize),
                        ),
                        false => AccountMeta::new_readonly(
                            pubkey,
                            transaction.message.is_signer(*account_index as usize),
                        ),
                    }
                })
                .collect();

            if !identity_added
                && program_id.to_string() != "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
            {
                accounts.push(AccountMeta::new_readonly(reference_pubkey, false));
                accounts.push(AccountMeta::new_readonly(identity_pubkey, false));

                identity_added = true;
            }

            Instruction {
                program_id,
                data: instruction.data.clone(),
                accounts,
            }
        })
        .collect();

    instructions_with_identity.push(Instruction {
        accounts: vec![],
        data: identity_message.as_bytes().to_vec(),
        program_id: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
    });

    let transaction_message_with_identity = Message::new(&instructions_with_identity, None);
    
    Transaction::new_unsigned(transaction_message_with_identity)
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
            transaction: add_action_identity_proof(transaction),
            message: Some("send donation to alice".to_string()),
        })
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
        label = "Send SOL",
        href = "/api/send_donation?amount={amount}",
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
