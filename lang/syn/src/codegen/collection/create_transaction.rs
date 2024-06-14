use proc_macro2::TokenStream;

use crate::{
    codegen::collection::common::{extract_action_ident, extract_action_query},
    CollectionMod,
};

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.action_fns
        .iter()
        .map(|action_fn| {
            let action_ident = extract_action_ident(&action_fn.raw_method).unwrap();
            let fn_block = &action_fn.raw_method.block;
            
            if let Some(
                action_query_type_ident,
            ) =
                extract_action_query(&action_fn.raw_method)
            {
                quote::quote! {
                    impl CreateTransactionWithQuery<#action_ident, #action_query_type_ident> for #action_ident {
                        fn create_transaction(&self, ctx: ContextWithQuery<#action_ident, #action_query_type_ident>) -> Result<solana_sdk::transaction::Transaction, znap_lang::ActionError> {
                            #fn_block
                        }
                    }
                }
            } else {
                quote::quote! {
                    impl CreateTransaction<#action_ident> for #action_ident {
                        fn create_transaction(&self, ctx: Context<#action_ident>) -> Result<solana_sdk::transaction::Transaction, znap_lang::ActionError> {
                            #fn_block
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
