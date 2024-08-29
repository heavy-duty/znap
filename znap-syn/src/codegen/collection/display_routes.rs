use crate::{common::create_path, CollectionMod};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let collection_ident = &collection_mod.name.to_string();
    let routes: Vec<TokenStream> = collection_mod
        .actions
        .iter()
        .map(|action| {
            let path = create_path(&action.to_string());

            quote! {
                println!("  {}     {}", "GET ".bold(), #path.to_string().italic());
                println!("  {}     {}", "POST".bold(), #path.to_string().italic());
            }
        })
        .collect();

    quote! {
        pub fn display_routes() {
            let collection_name = #collection_ident;
            println!("\n[{}] endpoints: \n", collection_name.cyan());
            #(#routes)*
        }
    }
}
