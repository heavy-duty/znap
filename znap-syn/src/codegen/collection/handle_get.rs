use proc_macro2::TokenStream;
use quote::quote;
use crate::{common::{create_get_context, create_get_handler, create_metadata}, CollectionMod};

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.actions
        .iter()
        .map(|action| {
            let handler = create_get_handler(&action.to_string());
            let context = create_get_context(&action.to_string());
            let create_metadata_fn = create_metadata(&action.to_string());
            
            match &collection_mod.get_action_fns.iter().find(|get_action_fn| get_action_fn.action == *action) {
                Some(get_action_fn) => {
                    let fn_block = &get_action_fn.raw_method.block;

                    match &get_action_fn.query {
                        Some(query) => {
                            quote! {
                                pub struct #context {
                                    query: #query,
                                }
        
                                pub fn #create_metadata_fn(ctx: #context) -> znap::Result<znap::ActionMetadata> {
                                    #fn_block
                                }

                                pub async fn #handler(
                                    axum::extract::Query(query): axum::extract::Query<#query>,
                                ) -> znap::Result<axum::Json<znap::ActionMetadata>> {
                                    let context = #context {
                                        query,
                                    };
                                    let metadata = #create_metadata_fn(context)?;
                
                                    Ok(axum::Json(metadata))
                                }
                            }
                        },
                        _ => {
                            quote! {
                                pub struct #context;
        
                                pub fn #create_metadata_fn(ctx: #context) -> znap::Result<znap::ActionMetadata> {
                                    #fn_block
                                }

                                pub async fn #handler() -> znap::Result<axum::Json<znap::ActionMetadata>> {
                                    let context = #context;
                                    let metadata = #create_metadata_fn(context)?;
                
                                    Ok(axum::Json(metadata))
                                }
                            }
                        }
                    }
                },
                _ => {
                    quote! {
                        pub struct #context;

                        pub fn #create_metadata_fn(ctx: #context) -> znap::Result<znap::ActionMetadata> {
                            let metadata = #action::to_metadata();

                            Ok(metadata)
                        }

                        pub async fn #handler() -> znap::Result<axum::Json<znap::ActionMetadata>> {
                            let context = #context;
                            let metadata = #create_metadata_fn(context)?;
        
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
