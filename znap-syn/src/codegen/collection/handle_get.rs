use crate::{
    common::{
        create_get_context, create_get_context_with_metadata, create_get_handler, create_metadata,
        create_params,
    },
    CollectionMod,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(collection_mod: &CollectionMod) -> TokenStream {
    let impls: Vec<TokenStream> = collection_mod.actions
        .iter()
        .map(|action| {
            let handler = create_get_handler(&action.to_string());
            let context = create_get_context(&action.to_string());
            let context_with_metadata = create_get_context_with_metadata(&action.to_string());
            let create_metadata_fn = create_metadata(&action.to_string());
            let params = create_params(&action.to_string());

            match &collection_mod.get_action_fns.iter().find(|get_action_fn| get_action_fn.action == *action) {
                Some(get_action_fn) => {
                    let fn_block = &get_action_fn.raw_method.block;

                    quote! {
                        #[derive(Debug, serde::Serialize, serde::Deserialize)]
                        pub struct #context {
                            params: #params,
                            #[serde::skip]
                            env: znap::Env,
                        }

                        #[derive(Debug, serde::Serialize, serde::Deserialize)]
                        pub struct #context_with_metadata {
                            params: #params,
                            #[serde::skip]
                            env: znap::Env,
                            metadata: znap::ActionMetadata,
                        }

                        pub async fn #create_metadata_fn(ctx: #context_with_metadata) -> znap::Result<znap::ActionMetadata> {
                            #fn_block
                        }

                        pub async fn #handler(
                            axum::extract::Path(params): axum::extract::Path<#params>,
                        ) -> znap::Result<axum::Json<znap::ActionMetadata>> {
                            let raw_metadata = #action::to_metadata();
                            let context = #context {
                                params,
                                env: znap::Env::default(),
                            };
                            let rendered_metadata = znap::render_metadata(&raw_metadata, &context, false, None);
                            let context_with_metadata = #context_with_metadata {
                                params: context.params,
                                env: znap::Env::default(),
                                metadata: rendered_metadata,
                            };
                            let metadata = #create_metadata_fn(context_with_metadata).await?;

                            Ok(axum::Json(metadata))
                        }
                    }
                },
                _ => {
                    quote! {
                        #[derive(Debug, serde::Serialize, serde::Deserialize)]
                        pub struct #context {
                            params: #params,
                            #[serde::skip]
                            env: znap::Env,
                        }

                        pub async fn #handler(
                            axum::extract::Path(params): axum::extract::Path<#params>,
                        ) -> znap::Result<axum::Json<znap::ActionMetadata>> {
                            let raw_metadata = #action::to_metadata();
                            let context = #context {
                                params,
                                env: znap::Env::default(),
                            };
                            let rendered_metadata = znap::render_metadata(&raw_metadata, &context, false, None);

                            Ok(axum::Json(rendered_metadata))
                        }
                    }
                }
            }
        })
        .collect();

    quote! {
        #(#impls)*
    }
}
