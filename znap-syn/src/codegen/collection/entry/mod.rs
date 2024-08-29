use std::path::PathBuf;

use quote::quote;
use znap_common::Config;

use crate::common::{create_get_handler, create_path, create_post_handler};
use crate::CollectionMod;

pub fn generate(collection_mod: &CollectionMod) -> proc_macro2::TokenStream {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(manifest_path);
    let znap_metadata = znap_common::get_config(Some(path));

    let collections = collection_mod
        .actions
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    if std::env::var("ZNAP_DEPLOY_SHUTTLE").is_ok() {
        generate_shuttle(&znap_metadata)
    } else {
        generate_axum(&znap_metadata, &collections)
    }
}

fn generate_shuttle(config: &Config) -> proc_macro2::TokenStream {
    let identity_keypair = config.identity.as_ref().unwrap();
    let rpc_url = config.rpc_url.as_ref().unwrap();

    quote! {
    use shuttle_runtime::SecretStore;

    mod collections;

    #[shuttle_runtime::main]
    async fn main(
        #[shuttle_runtime::Secrets] secrets: SecretStore,
    ) -> shuttle_axum::ShuttleAxum {
        std::env::set_var("IDENTITY_KEYPAIR", #identity_keypair);
        std::env::set_var("RPC_URL", #rpc_url);

        Ok(collections::router().into())
    }
        }
}

fn generate_axum(config: &Config, collections: &[String]) -> proc_macro2::TokenStream {
    let action = config.collections.as_deref().unwrap().first().unwrap();
    let action_host = &action.address;
    let action_port = action.port;

    // let route_fn = generate_axum_routes(collections);

    quote! {
        use tokio::net::TcpListener;
        use std::env;

        fn main() -> Result<(), axum::Error> {
            let body = async {
                display_routes();
                let listener = TcpListener::bind(format!("{}:{}", #action_host, #action_port))
                    .await
                    .unwrap();
                axum::serve(listener, router()).await.unwrap();
                Ok(())
            };
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed building the Runtime")
                .block_on(body)
        }
    }
}

fn generate_axum_routes(collections: &[String]) -> proc_macro2::TokenStream {
    let routes: Vec<proc_macro2::TokenStream> = collections
        .iter()
        .map(|action| {
            let get_handler = create_get_handler(action);
            let post_handler = create_post_handler(action);
            let path = create_path(action);

            quote! {
                .route(
                    #path,
                    axum::routing::get(#get_handler).post(#post_handler)
                )
            }
        })
        .collect();

    let display_routes = generate_display_path(collections);

    quote! {
        pub async fn handle_status_request() -> znap::Result<axum::Json<znap::Status>> {
            Ok(axum::Json(znap::Status { active: true }))
        }

        #display_routes

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

pub fn generate_display_path(collections: &[String]) -> proc_macro2::TokenStream {
    let routes: Vec<proc_macro2::TokenStream> = collections
        .iter()
        .map(|action| {
            let path = create_path(action);

            quote! {
                println!("  {}\t{}", "GET ", #path);
                println!("  {}\t{}", "POST", #path);
            }
        })
        .collect();

    quote! {
        pub fn display_routes() {
            println!("\n[{}] endpoints: \n", "my_actions");
            #(#routes)*
        }
    }
}
