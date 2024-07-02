use crate::{common::create_params, ActionStruct};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(action_struct: &ActionStruct) -> TokenStream {
    let params = create_params(&action_struct.name.to_string());

    if let Some(params_attrs) = &action_struct.params_attrs {
        let attributes = params_attrs.iter().map(|(name, value)| {
            quote! {
                pub #name: #value
            }
        });

        quote! {
            #[derive(Debug, serde::Deserialize)]
            struct #params {
                #(#attributes),*
            }
        }
    } else {
        quote! {
            #[derive(Debug, serde::Deserialize)]
            struct #params {}
        }
    }
}
