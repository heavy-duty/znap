use std::env::var;

use solana_sdk::signature::Keypair;

pub struct Env {
    pub identity: Vec<u8>,
    pub keypair: Keypair,
}

impl Default for Env {
    fn default() -> Self {
        let identity = var("IDENTITY_KEYPAIR_PATH")
            .map(|path| std::fs::read(path).unwrap())
            .or(var("IDENTITY_KEYPAIR").map(|v| v.as_bytes().to_vec()))
            .expect("Cannot found `IDENTITY_KEYPAIR_PATH` or `IDENTITY_KEYPAIR` env var");

        let keypair = Keypair::from_bytes(&identity).unwrap();

        Self { identity, keypair }
    }
}
