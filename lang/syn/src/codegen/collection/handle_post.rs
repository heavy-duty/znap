use proc_macro2::TokenStream;
use quote::quote;
use crate::{codegen::collection::common::extract_action_query, CollectionMod};

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.action_fns
        .iter()
        .map(|action_fn| {
            let action_ident = &action_fn.action_ident;
            let handler_ident = &action_fn.handle_post_ident;
            
            if let Some(
                action_query_type_ident,
            ) =
                extract_action_query(&action_fn.raw_method)
            {
                quote! {
                    pub async fn #handler_ident(
                        axum::extract::Query(query): axum::extract::Query<#action_query_type_ident>,
                        axum::Json(payload): axum::Json<znap_lang::CreateActionPayload>,
                    ) -> Result<axum::Json<znap_lang::ActionTransaction>, znap_lang::ActionError> {
                        let action = #action_ident;
                        let context_with_query = znap_lang::ContextWithQuery::<#action_ident, #action_query_type_ident> {
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
            } else {
                quote! {
                    pub async fn #handler_ident(
                        axum::Json(payload): axum::Json<znap_lang::CreateActionPayload>
                    )  -> Result<axum::Json<znap_lang::ActionTransaction>, znap_lang::ActionError>{
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
        })
        .collect();

    quote! {
        #(#impls)*
    }
}

