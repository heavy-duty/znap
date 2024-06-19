use proc_macro2::TokenStream;
use quote::quote;
use crate::CollectionMod;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let collection_ident = &collection_mod.name.to_string();
    let routes: Vec<TokenStream> = collection_mod.action_fns
        .iter()
        .map(|action_fn| {
            let route_path = &action_fn.route_path;
            
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

