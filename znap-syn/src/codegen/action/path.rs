use crate::{
    common::{action_name_without_suffix, create_path},
    ActionStruct,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(action_struct: &ActionStruct) -> TokenStream {
    let path = create_path(&action_struct.name.to_string());
    let action_name = action_name_without_suffix(&action_struct.name.to_string());
    let prefix = action_struct
        .attributes
        .as_ref()
        .map(|attr| format!("/{}", attr.prefix))
        .unwrap_or_default();
    let mut segments = vec![prefix, action_name];

    if let Some(params_attrs) = &action_struct.params_attrs {
        segments.extend(
            params_attrs
                .iter()
                .map(|(param_name, _)| format!(":{param_name}"))
                .collect::<Vec<_>>(),
        );

        let action_path = segments.join("/");

        quote! {
            const #path: &str = #action_path;
        }
    } else {
        let action_path = segments.join("/");

        quote! {
            const #path: &str = #action_path;
        }
    }
}
