use crate::{ActionAttributesStruct, ActionStruct};
use proc_macro2::TokenStream;

pub fn generate(action_struct: &ActionStruct) -> TokenStream {
    let ActionStruct {
        name, attributes, ..
    } = action_struct;
    let ActionAttributesStruct {
        title,
        description,
        label,
        icon,
    } = attributes;

    quote::quote! {
        impl ToMetadata for #name {
            fn to_metadata(&self) -> ActionMetadata {
                ActionMetadata {
                    icon: #icon,
                    title: #title,
                    description: #description,
                    label: #label,
                }
            }
        }
    }
}
