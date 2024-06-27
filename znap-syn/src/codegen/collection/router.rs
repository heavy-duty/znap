use crate::CollectionMod;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let routes: Vec<TokenStream> = collection_mod
        .action_fns
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
            let cors = tower_http::cors::CorsLayer::new()
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::CONTENT_ENCODING,
                    axum::http::header::ACCEPT_ENCODING,
                ])
                .allow_origin(tower_http::cors::Any);

            axum::Router::new()
                #(#routes)*
                .route_service("/actions.json", tower_http::services::ServeFile::new("actions.json"))
                .layer(cors)
        }
    }
}
