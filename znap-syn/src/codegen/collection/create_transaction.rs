use proc_macro2::TokenStream;
use quote::quote;
use crate::CollectionMod;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.post_action_fns
        .iter()
        .map(|action_fn| {
            let action = &action_fn.action;
            let fn_block = &action_fn.raw_method.block;
            
            match &action_fn.query {
                Some(query) => {
                    quote! {
                        impl CreateTransactionWithQuery<#action, #query> for #action {
                            fn create_transaction(&self, ctx: znap::PostContextWithQuery<#action, #query>) -> znap::Result<solana_sdk::transaction::Transaction> {
                                #fn_block
                            }
                        }
                    }
                },
                _ => {
                    quote! {
                        impl CreateTransaction<#action> for #action {
                            fn create_transaction(&self, ctx: znap::PostContext<#action>) -> znap::Result<solana_sdk::transaction::Transaction> {
                                #fn_block
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
