use crate::ActionStruct;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(action_struct: &ActionStruct) -> TokenStream {
    let action = &action_struct.name;
    let action_str = &action.to_string();

    quote! {
        impl #action {
            pub fn name() -> &'static str {
                #action_str
            }
        }
    }
}
