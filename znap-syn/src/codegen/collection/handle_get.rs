use crate::{codegen::collection::common::create_get_handler, CollectionMod};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.actions
        .iter()
        .map(|action| {
            let handler = create_get_handler(&action.to_string());

            match &collection_mod.get_action_fns.iter().find(|get_action_fn| get_action_fn.action == *action) {
                Some(get_action_fn) => {
                    match &get_action_fn.query {
                        Some(query) => {
                            quote! {
                                pub async fn #handler(
                                    axum::extract::Query(query): axum::extract::Query<#query>,
                                ) -> znap::Result<axum::Json<znap::ActionMetadata>> {
                                    let context_with_query = znap::GetContextWithQuery::<#action, #query> {
                                        action: std::marker::PhantomData,
                                        query
                                    };
                                    let metadata = #action::create_metadata(context_with_query)?;
                
                                    Ok(axum::Json(metadata))
                                }
                            }
                        },
                        _ => {
                            quote! {
                                pub async fn #handler() -> znap::Result<axum::Json<znap::ActionMetadata>> {
                                    let context = znap::GetContext::<#action> {
                                        action: std::marker::PhantomData,
                                    };
                                    let metadata = #action::create_metadata(context)?;
                
                                    Ok(axum::Json(metadata))
                                }
                            }
                        }
                    }
                },
                _ => {
                    quote! {
                        pub async fn #handler() -> znap::Result<axum::Json<znap::ActionMetadata>> {
                            let context = znap::GetContext::<#action> {
                                action: std::marker::PhantomData,
                            };
                            let metadata = #action::create_metadata(context)?;
        
                            Ok(axum::Json(metadata))
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
