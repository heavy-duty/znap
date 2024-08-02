use std::env::var;

use solana_sdk::signature::Keypair;

#[derive(Debug)]
pub struct Env {
    pub identity: Vec<u8>,
    pub keypair: Keypair,
    pub rpc_url: String,
}

impl Default for Env {
    fn default() -> Self {
        let keypair = var("IDENTITY_KEYPAIR")
            .or(var("IDENTITY_KEYPAIR_PATH").map(|path| std::fs::read_to_string(path).unwrap()))
            .map(|i| {
                if i.starts_with('[') {
                    let i = i.trim_start_matches('[').trim_end_matches(']');
                    let b = i
                        .split(',')
                        .map(|b| b.trim().parse::<u8>().unwrap())
                        .collect::<Vec<_>>();
                    Keypair::from_bytes(&b).unwrap()
                } else {
                    Keypair::from_base58_string(&i)
                }
            })
            .expect("Cannot found `IDENTITY_KEYPAIR_PATH` or `IDENTITY_KEYPAIR` env var");

        let identity = keypair.to_bytes().to_vec();

        let rpc_url = var("RPC_URL").expect("Cannot found `RPC_URL` env var");

        Self {
            identity,
            keypair,
            rpc_url,
        }
    }
}
