use heck::ToSnekCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;
use crate::CollectionMod;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.post_action_fns
        .iter()
        .map(|action_fn| {
            let action_ident = &action_fn.action_ident;
            let action_name = action_ident.to_string().to_snek_case();
            let handle_ident =
                Ident::new(&format!("handle_post_{}", action_name), Span::call_site());
            
            match &action_fn.action_query_ident {
                Some(action_query_ident) => {
                    quote! {
                        pub async fn #handle_ident(
                            axum::extract::Query(query): axum::extract::Query<#action_query_ident>,
                            axum::Json(payload): axum::Json<znap::CreateActionPayload>,
                        ) -> znap::Result<axum::Json<znap::ActionTransaction>> {
                            let action = #action_ident;
                            let context_with_query = znap::PostContextWithQuery::<#action_ident, #action_query_ident> {
                                payload,
                                action: std::marker::PhantomData,
                                query
                            };
                            let transaction = action.create_transaction(context_with_query)?;
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
                        pub async fn #handle_ident(
                            axum::Json(payload): axum::Json<znap::CreateActionPayload>
                        )  -> znap::Result<axum::Json<znap::ActionTransaction>>{
                            let action = #action_ident {};
                            let context = znap::PostContext::<#action_ident> {
                                payload,
                                action: std::marker::PhantomData,
                            };
                            let transaction = action.create_transaction(context)?;
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

