use crate::{common::create_query, ActionStruct};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(action_struct: &ActionStruct) -> TokenStream {
    let query = create_query(&action_struct.name.to_string());

    if let Some(query_attrs) = &action_struct.query_attrs {
        let attributes = query_attrs.iter().map(|(name, value)| {
            quote! {
                pub #name: #value
            }
        });

        quote! {
            #[derive(Debug, serde::Deserialize)]
            struct #query {
                #(#attributes),*
            }
        }
    } else {
        quote! {
            #[derive(Debug, serde::Deserialize)]
            struct #query {}
        }
    }
}
