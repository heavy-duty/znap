use proc_macro2::TokenStream;
use quote::quote;
use crate::CollectionMod;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.action_fns
        .iter()
        .map(|action_fn| {
            let action_ident = &action_fn.action_ident;
            let handler_ident = &action_fn.handle_post_ident;
            
            match &action_fn.action_query_ident {
                Some(action_query_ident) => {
                    quote! {
                        pub async fn #handler_ident(
                            axum::extract::Query(query): axum::extract::Query<#action_query_ident>,
                            axum::Json(payload): axum::Json<znap_lang::CreateActionPayload>,
                        ) -> znap_lang::Result<axum::Json<znap_lang::ActionTransaction>> {
                            let action = #action_ident;
                            let context_with_query = znap_lang::ContextWithQuery::<#action_ident, #action_query_ident> {
                                payload,
                                action: std::marker::PhantomData,
                                query
                            };
                            let transaction = action.create_transaction(context_with_query)?;
                            let serialized_transaction = bincode::serialize(&transaction).unwrap();
                            let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);
        
                            Ok(axum::Json(znap_lang::ActionTransaction {
                                transaction: encoded_transaction,
                                message: None
                            }))
                        }
                    }
                },
                _ => {
                    quote! {
                        pub async fn #handler_ident(
                            axum::Json(payload): axum::Json<znap_lang::CreateActionPayload>
                        )  -> znap_lang::Result<axum::Json<znap_lang::ActionTransaction>>{
                            let action = #action_ident {};
                            let context = znap_lang::Context::<#action_ident> {
                                payload,
                                action: std::marker::PhantomData,
                            };
                            let transaction = action.create_transaction(context)?;
                            let serialized_transaction = bincode::serialize(&transaction).unwrap();
                            let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);
        
                            Ok(axum::Json(znap_lang::ActionTransaction {
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

