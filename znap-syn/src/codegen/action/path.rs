use crate::{
    common::{action_name_without_suffix, create_path},
    ActionStruct,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(action_struct: &ActionStruct) -> (String, TokenStream) {
    let path = create_path(&action_struct.name.to_string());
    let action_name = action_name_without_suffix(&action_struct.name.to_string());
    let route = if let Some(path) = &action_struct.path_attrs {
        match (&path.prefix, &path.template) {
            (Some(prefix), Some(template)) => format!(
                "/{}/{}",
                prefix.trim_matches('/'),
                template
                    .replace("{{action_name}}", &action_name)
                    .replace("{{prefix}}", prefix.trim_matches('/'))
                    .trim_matches('/')
            ),
            (None, Some(template)) => format!(
                "/{}",
                template
                    .replace("{{action_name}}", &action_name)
                    .replace("{{prefix}}", "api")
                    .trim_matches('/')
            ),
            (Some(prefix), None) => format!("/{}/{}", prefix.trim_matches('/'), &action_name),
            _ => format!("/{}", &action_name),
        }
    } else {
        format!("/api/{}", action_name)
    };

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
