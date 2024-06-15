use crate::CollectionMod;
use proc_macro2::TokenStream;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod
        .action_fns
        .iter()
        .map(|action_fn| {
            let action_ident = &action_fn.action_ident;
            let handler_ident = &action_fn.handle_get_ident;
            
            quote::quote! {
                pub async fn #handler_ident() -> znap_lang::Result<axum::Json<ActionMetadata>> {
                    let action = #action_ident;

                    Ok(axum::Json(action.to_metadata()))
                }
            }
        })
        .collect();

    quote::quote! {
        #(#impls)*
    }
}
