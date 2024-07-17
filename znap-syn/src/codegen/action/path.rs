use crate::{
    common::{action_name_without_suffix, create_path},
    ActionStruct,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(action_struct: &ActionStruct) -> TokenStream {
    let path = create_path(&action_struct.name.to_string());
    let action_name = action_name_without_suffix(&action_struct.name.to_string());

    if let Some(params_attrs) = &action_struct.params_attrs {
        let mut segments: Vec<String> = vec!["/api".to_string(), action_name];

        for (param_name, _) in params_attrs {
            segments.push(format!(":{}", param_name));
        }

        let action_path = segments.join("/");

        quote! {
            const #path: &str = #action_path;
        }
    } else {
        let action_path = format!("/api/{}", action_name);

        quote! {
            const #path: &str = #action_path;
        }
    }
}
