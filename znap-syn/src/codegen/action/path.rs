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
            let path = if !attr.path.as_ref().is_some_and(|p| p.contains("{{prefix}}")) {
                format!(
                    "/{}/{}",
                    attr.prefix.as_deref().unwrap_or("api").trim_matches('/'),
                    attr.path
                        .as_deref()
                        .unwrap_or("{{action_name}}")
                        .replace("{{action_name}}", &action_name)
                        .trim_matches('/')
                )
            } else {
                format!(
                    "/{}",
                    attr.path
                        .as_deref()
                        .unwrap_or("{{prefix}}/{{action_name}}")
                        .replace("{{prefix}}", attr.prefix.as_deref().unwrap_or("api"))
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

    if let Some(params_attrs) = action_struct.params_attrs.as_ref() {
        let mut segments = vec![route.clone()];
        segments.extend(
            params_attrs
                .iter()
                .filter_map(|(param_name, _)| {
                    if !route.contains(&param_name.to_string()) {
                        return Some(format!(":{param_name}"));
                    }
                    None
                })
                .collect::<Vec<_>>(),
        );

        let action_path = segments.join("/");

        (
            action_path.clone(),
            quote! {
                const #path: &str = #action_path;
            },
        )
    } else {
        (
            route.clone(),
            quote! {
                const #path: &str = #route;
            },
        )
    }
}
