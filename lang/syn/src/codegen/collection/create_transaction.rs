use proc_macro2::TokenStream;
use crate::CollectionMod;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.action_fns
        .iter()
        .map(|action_fn| {
            let action_ident = &action_fn.action_ident;
            let fn_block = &action_fn.raw_method.block;
            
            match &action_fn.action_query_ident {
                Some(action_query_ident) => {
                    quote::quote! {
                        impl CreateTransactionWithQuery<#action_ident, #action_query_ident> for #action_ident {
                            fn create_transaction(&self, ctx: ContextWithQuery<#action_ident, #action_query_ident>) -> znap_lang::Result<solana_sdk::transaction::Transaction> {
                                #fn_block
                            }
                        }
                    }
                },
                _ => {
                    quote::quote! {
                        impl CreateTransaction<#action_ident> for #action_ident {
                            fn create_transaction(&self, ctx: Context<#action_ident>) -> znap_lang::Result<solana_sdk::transaction::Transaction> {
                                #fn_block
                            }
                        }
                    }
                }
            }
        })
        .collect();

    quote::quote! {
        #(#impls)*
    }
}
