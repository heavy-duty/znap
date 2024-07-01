use crate::{common::create_query, ActionStruct};
use quote::quote;
mod to_metadata;

pub fn generate(action_struct: &ActionStruct) -> proc_macro2::TokenStream {
    let to_metadata = to_metadata::generate(action_struct);
    let action = &action_struct.name;

    println!("{:?}", action_struct.query);

    /* if let Some(_) = &action_struct.query {
        quote! {
            #to_metadata
        }
    } else {
        let query = create_query(&action.to_string());

        quote! {
            #[query]
            struct #query {}

            #to_metadata
        }
    }
 */

quote! {
    #to_metadata
}

}