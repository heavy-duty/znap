use crate::{parser::collection::common::action_name_without_suffix, CollectionMod};
use heck::ToSnekCase;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let collection_ident = &collection_mod.name.to_string();
    let routes: Vec<TokenStream> = collection_mod
        .action_idents
        .iter()
        .map(|action_ident| {
            let action_name = action_ident.to_string().to_snek_case();
            let route_path = format!(
                "/actions/{}_{}",
                collection_mod.name.to_string().to_snek_case(),
                action_name_without_suffix(&action_name).to_snek_case()
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
