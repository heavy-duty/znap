use crate::CollectionMod;
use heck::ToSnekCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.action_idents
        .iter()
        .map(|action_ident| {
            let maybe_get_action_fn = collection_mod.get_action_fns.iter().find(|get_action_fn| get_action_fn.action_ident == *action_ident);
            let action_name = action_ident.to_string().to_snek_case();
            let handle_ident =
                Ident::new(&format!("handle_get_{}", action_name), Span::call_site());

            match &maybe_get_action_fn {
                Some(get_action_fn) => {
                    match &get_action_fn.action_query_ident {
                        Some(action_query_ident) => {
                            quote! {
                                pub async fn #handle_ident(
                                    axum::extract::Query(query): axum::extract::Query<#action_query_ident>,
                                ) -> znap::Result<axum::Json<znap::ActionMetadata>> {
                                    let action = #action_ident {};
                                    let context_with_query = znap::GetContextWithQuery::<#action_ident, #action_query_ident> {
                                        action: std::marker::PhantomData,
                                        query
                                    };
                                    let metadata = action.create_metadata(context_with_query)?;
                
                                    Ok(axum::Json(metadata))
                                }
                            }
                        },
                        _ => {
                            quote! {
                                pub async fn #handle_ident() -> znap::Result<axum::Json<znap::ActionMetadata>> {
                                    let action = #action_ident {};
                                    let context = znap::GetContext::<#action_ident> {
                                        action: std::marker::PhantomData,
                                    };
                                    let metadata = action.create_metadata(context)?;
                
                                    Ok(axum::Json(metadata))
                                }
                            }
                        }
                    }
                },
                _ => {
                    let action_name = action_ident.to_string().to_snek_case();
                    let handle_ident =
                        Ident::new(&format!("handle_get_{}", action_name), Span::call_site());

                    quote! {
                        pub async fn #handle_ident() -> znap::Result<axum::Json<znap::ActionMetadata>> {
                            let action = #action_ident {};
                            let context = znap::GetContext::<#action_ident> {
                                action: std::marker::PhantomData,
                            };
                            let metadata = action.create_metadata(context)?;
        
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
