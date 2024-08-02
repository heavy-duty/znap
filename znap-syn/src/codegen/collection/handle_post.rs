use crate::{
    common::{
        create_params, create_post_context, create_post_handler, create_query, create_transaction,
    },
    CollectionMod,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.post_action_fns
        .iter()
        .map(|action_fn| {
            let action = &action_fn.action;
            let handler = create_post_handler(&action.to_string());
            let context = create_post_context(&action.to_string());
            let create_transaction_fn = create_transaction(&action.to_string());
            let fn_block = &action_fn.raw_method.block;
            let query = create_query(&action.to_string());
            let params = create_params(&action.to_string());

            quote! {
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #context {
                    query: #query,
                    params: #params,
                    payload: znap::CreateActionPayload,
                    #[serde(skip)]
                    env: znap::env::Env,
                }

                pub async fn #create_transaction_fn(ctx: &#context) -> znap::Result<znap::ActionTransaction> {
                    #fn_block
                }

                pub async fn #handler(
                    axum::extract::Query(query): axum::extract::Query<#query>,
                    axum::extract::Path(params): axum::extract::Path<#params>,
                    axum::Json(payload): axum::Json<znap::CreateActionPayload>,
                ) -> znap::Result<axum::Json<znap::ActionResponse>> {
                    let context = #context {
                        payload,
                        query,
                        params,
                        env: znap::env::Env::default(),
                    };
                    let znap::ActionTransaction { transaction, message } = #create_transaction_fn(&context).await?;
                    let mut transaction_with_identity = znap::add_action_identity_proof(transaction, &context.env.keypair);
                    let rpc_client = solana_client::rpc_client::RpcClient::new_with_commitment(
                        context.env.rpc_url,
                        solana_sdk::commitment_config::CommitmentConfig::confirmed(),
                    );
                    let recent_blockhash = rpc_client
                        .get_latest_blockhash()
                        .or_else(|_| Err(Error::new(
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            "FetchBlockhashRequestFailed".to_string(),
                            "The server could not fetch the blockhash".to_string(),
                        )))?;
                    transaction_with_identity.message.recent_blockhash = recent_blockhash;
                    let serialized_transaction = bincode::serialize(&transaction_with_identity).unwrap();
                    let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);

                    Ok(axum::Json(znap::ActionResponse {
                        transaction: encoded_transaction,
                        message
                    }))
                }
            }
        })
        .collect();

    quote! {
        #(#impls)*
    }
}
