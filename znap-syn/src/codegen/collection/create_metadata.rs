use crate::CollectionMod;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.action_idents
        .iter()
        .map(|action_ident| {
            let maybe_get_action_fn = collection_mod.get_action_fns.iter().find(|get_action_fn| get_action_fn.action_ident == *action_ident);

            match &maybe_get_action_fn {
                Some(get_action_fn) => {
                    let fn_block = &get_action_fn.raw_method.block;

                    match &get_action_fn.action_query_ident {
                        Some(action_query_ident) => {
                            quote! {
                                impl CreateMetadataWithQuery<#action_ident, #action_query_ident> for #action_ident {
                                    fn create_metadata(&self, ctx: znap::GetContextWithQuery<#action_ident, #action_query_ident>) -> znap::Result<znap::ActionMetadata> {
                                        #fn_block
                                    }
                                }
                            }
                        },
                        _ => {
                            quote! {
                                impl CreateMetadata<#action_ident> for #action_ident {
                                    fn create_metadata(&self, ctx: znap::GetContext<#action_ident>) -> znap::Result<znap::ActionMetadata> {
                                        #fn_block
                                    }
                                }
                            }
                        }
                    }
                },
                _ => {
                    quote! {
                        impl CreateMetadata<#action_ident> for #action_ident {
                            fn create_metadata(&self, ctx: znap::GetContext<#action_ident>) -> znap::Result<znap::ActionMetadata> {
                                let action = #action_ident {};
                                let metadata = action.to_metadata();

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
