use std::env::var;

use solana_sdk::signature::Keypair;

#[derive(Debug)]
pub struct Env {
    pub identity: Vec<u8>,
    pub keypair: Keypair,
}

impl Default for Env {
    fn default() -> Self {
        let identity = var("IDENTITY_KEYPAIR")
            .or(var("IDENTITY_KEYPAIR_PATH").map(|path| std::fs::read_to_string(path).unwrap()))
            .map(|i| {
                if i.starts_with('[') {
                    let i = i.trim_start_matches('[').trim_end_matches(']');
                    i.split(',')
                        .map(|b| b.trim().parse::<u8>().unwrap())
                        .collect()
                } else {
                    i.as_bytes().to_vec()
                }
            })
            .expect("Cannot found `IDENTITY_KEYPAIR_PATH` or `IDENTITY_KEYPAIR` env var");

        let keypair = Keypair::from_bytes(&identity).unwrap();

        Self { identity, keypair }
    }
}
