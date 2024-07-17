use crate::{
    common::{action_name_without_suffix, create_path},
    ActionStruct,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(action_struct: &ActionStruct) -> (String, TokenStream) {
    let path = create_path(&action_struct.name.to_string());
    let action_name = action_name_without_suffix(&action_struct.name.to_string());
    let route = action_struct
        .attributes
        .as_ref()
        .map(|attr| {
            let path = if !attr.custom_path.contains("{{prefix}}") {
                format!(
                    "/{}/{}",
                    attr.prefix.trim_matches('/'),
                    attr.custom_path
                        .replace("{{action_name}}", &action_name)
                        .trim_matches('/')
                )
            } else {
                format!(
                    "/{}",
                    attr.custom_path
                        .replace("{{prefix}}", &attr.prefix)
                        .replace("{{action_name}}", &action_name)
                        .trim_matches('/')
                )
            };
            if !path.contains(&action_name) {
                format!("{path}/{action_name}")
            } else {
                path
            }
        })
        .unwrap_or_default();
    let mut segments = vec![route.clone()];

    if let Some(params_attrs) = &action_struct.params_attrs {
        segments.extend(
            params_attrs
                .iter()
                .map(|(param_name, _)| format!(":{param_name}"))
                .collect::<Vec<_>>(),
        );

        let action_path = segments.join("/");

        (
            route,
            quote! {
                const #path: &str = #action_path;
            },
        )
    } else {
        let action_path = segments.join("/");

        (
            route,
            quote! {
                const #path: &str = #action_path;
            },
        )
    }
}
