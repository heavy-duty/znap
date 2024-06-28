use crate::CollectionMod;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.actions
        .iter()
        .map(|action| {
            match &collection_mod.get_action_fns.iter().find(|get_action_fn| get_action_fn.action == *action) {
                Some(get_action_fn) => {
                    let fn_block = &get_action_fn.raw_method.block;

                    match &get_action_fn.query {
                        Some(query) => {
                            quote! {
                                impl CreateMetadataWithQuery<#action, #query> for #action {
                                    fn create_metadata(ctx: znap::GetContextWithQuery<#action, #query>) -> znap::Result<znap::ActionMetadata> {
                                        #fn_block
                                    }
                                }
                            }
                        },
                        _ => {
                            quote! {
                                impl CreateMetadata<#action> for #action {
                                    fn create_metadata(ctx: znap::GetContext<#action>) -> znap::Result<znap::ActionMetadata> {
                                        #fn_block
                                    }
                                }
                            }
                        }
                    }
                },
                _ => {
                    quote! {
                        impl CreateMetadata<#action> for #action {
                            fn create_metadata(ctx: znap::GetContext<#action>) -> znap::Result<znap::ActionMetadata> {
                                let metadata = #action::to_metadata();

                                Ok(metadata)
                            }
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
