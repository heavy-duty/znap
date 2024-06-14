use proc_macro2::TokenStream;
use quote::quote;
use crate::CollectionMod;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let routes: Vec<TokenStream> = collection_mod.action_fns
        .iter()
        .map(|action_fn| {
            let handle_get_ident = &action_fn.handle_get_ident;
            let handle_post_ident = &action_fn.handle_post_ident;
            let route_path = &action_fn.route_path;
            
            quote! {
                .route(
                    #route_path,
                    axum::routing::get(#handle_get_ident).post(#handle_post_ident)
                )
            }
        })
        .collect();

    quote! {
        pub fn collection_router() -> axum::Router {
            axum::Router::new()
                #(#routes)*
        }
    }
}

