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
    icon = "https://arweave.net/uidd6qXKV9Msx8LavjhNdCis_UmFm_RhcmeQxi1fZJE",
    title = "Liquid stake SOL",
    description = "Stake with SolanaHub! Support public good and gain access to stake boost through our Loyalty League. 🚀",
    label = "Stake boost",
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