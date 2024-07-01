use heck::ToSnekCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{common::action_name_without_suffix, CollectionMod};

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let collection_ident = &collection_mod.name.to_string();
    let routes: Vec<TokenStream> = collection_mod
        .actions
        .iter()
        .map(|action| {
            let route_path = format!(
                "/api/{}",
                action_name_without_suffix(&action.to_string().to_snek_case()).to_snek_case()
            );

            quote! {
                println!("  {}     {}", "GET ".bold(), #route_path.italic());
                println!("  {}     {}", "POST".bold(), #route_path.italic());
            }
        })
        .collect();

    quote! {
        pub fn display_collection_routes() {
            let collection_name = #collection_ident;
            println!("\n[{}] endpoints: \n", collection_name.cyan());
            #(#routes)*
        }
    }
}
