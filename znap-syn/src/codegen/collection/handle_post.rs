use proc_macro2::TokenStream;
use quote::quote;
use crate::{common::{create_params, create_post_context, create_post_handler, create_query, create_transaction}, CollectionMod};

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
                pub struct #context {
                    query: #query,
                    params: #params,
                    payload: znap::CreateActionPayload
                }

                pub fn #create_transaction_fn(ctx: #context) -> znap::Result<solana_sdk::transaction::Transaction> {
                    #fn_block
                }

                pub async fn #handler(
                    axum::extract::Query(query): axum::extract::Query<#query>,
                    axum::extract::Path(params): axum::extract::Path<#params>,
                    axum::Json(payload): axum::Json<znap::CreateActionPayload>,
                ) -> znap::Result<axum::Json<znap::ActionTransaction>> {
                    let context = #context {
                        payload,
                        query,
                        params,
                    };
                    let transaction = #create_transaction_fn(context)?;
                    let serialized_transaction = bincode::serialize(&transaction).unwrap();
                    let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);

                    Ok(axum::Json(znap::ActionTransaction {
                        transaction: encoded_transaction,
                        message: None
                    }))
                }
            }
        })
        .collect();

    quote! {
        #(#impls)*
    }
}

