use crate::{
    common::{create_get_handler, create_path, create_post_handler},
    CollectionMod,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let routes: Vec<TokenStream> = collection_mod
        .actions
        .iter()
        .map(|action| {
            let get_handler = create_get_handler(&action.to_string());
            let post_handler = create_post_handler(&action.to_string());
            let path = create_path(&action.to_string());

            quote! {
                .route(
                    #path,
                    axum::routing::get(#get_handler).post(#post_handler)
                )
            }
        })
        .collect();

    quote! {
        pub async fn handle_status_request() -> znap::Result<axum::Json<znap::Status>> {
            Ok(axum::Json(znap::Status { active: true }))
        }

        pub fn router() -> axum::Router {
            let cors = tower_http::cors::CorsLayer::new()
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::OPTIONS,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::CONTENT_ENCODING,
                    axum::http::header::ACCEPT_ENCODING,
                ])
                .allow_origin(tower_http::cors::Any);

            axum::Router::new()
                #(#routes)*
                .route("/status", axum::routing::get(handle_status_request))
                .route_service("/actions.json", tower_http::services::ServeFile::new("actions.json"))
                .layer(cors)
        }
    }
}
