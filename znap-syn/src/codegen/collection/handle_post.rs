use proc_macro2::TokenStream;
use quote::quote;
use crate::{codegen::collection::common::create_post_handler, CollectionMod};

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.post_action_fns
        .iter()
        .map(|action_fn| {
            let action = &action_fn.action;            
            let handler = create_post_handler(&action.to_string());
                
            match &action_fn.query {
                Some(query) => {
                    quote! {
                        pub async fn #handler(
                            axum::extract::Query(query): axum::extract::Query<#query>,
                            axum::Json(payload): axum::Json<znap::CreateActionPayload>,
                        ) -> znap::Result<axum::Json<znap::ActionTransaction>> {
                            let context_with_query = znap::PostContextWithQuery::<#action, #query> {
                                payload,
                                action: std::marker::PhantomData,
                                query
                            };
                            let transaction = #action::create_transaction(context_with_query)?;
                            let serialized_transaction = bincode::serialize(&transaction).unwrap();
                            let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);
        
                            Ok(axum::Json(znap::ActionTransaction {
                                transaction: encoded_transaction,
                                message: None
                            }))
                        }
                    }
                },
                _ => {
                    quote! {
                        pub async fn #handler(
                            axum::Json(payload): axum::Json<znap::CreateActionPayload>
                        )  -> znap::Result<axum::Json<znap::ActionTransaction>>{
                            let context = znap::PostContext::<#action> {
                                payload,
                                action: std::marker::PhantomData,
                            };
                            let transaction = #action::create_transaction(context)?;
                            let serialized_transaction = bincode::serialize(&transaction).unwrap();
                            let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);
        
                            Ok(axum::Json(znap::ActionTransaction {
                                transaction: encoded_transaction,
                                message: None
                            }))
                        }
                    }
                }
            }
        })
        .collect();

    quote! {
        #(#impls)*
    }
}

