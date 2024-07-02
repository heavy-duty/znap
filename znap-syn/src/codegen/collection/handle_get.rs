use proc_macro2::TokenStream;
use quote::quote;
use crate::{common::{create_get_context, create_get_handler, create_metadata, create_params, create_query}, CollectionMod};

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
                    let query = create_query(&action.to_string());
                    let params = create_params(&action.to_string());

                    quote! {
                        pub struct #context {
                            query: #query,
                            params: #params,
                        }
        
                        pub fn #create_metadata_fn(ctx: #context) -> znap::Result<znap::ActionMetadata> {
                            #fn_block
                        }
        
                        pub async fn #handler(
                            axum::extract::Query(query): axum::extract::Query<#query>,
                            axum::extract::Path(params): axum::extract::Path<#params>,
                        ) -> znap::Result<axum::Json<znap::ActionMetadata>> {
                            let context = #context {
                                query,
                                params,
                            };
                            let metadata = #create_metadata_fn(context)?;
        
                            Ok(axum::Json(metadata))
                        }
                    }
                },
                _ => {
                    quote! {
                        pub async fn #handler() -> znap::Result<axum::Json<znap::ActionMetadata>> {
                            Ok(axum::Json(#action::to_metadata()))
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
