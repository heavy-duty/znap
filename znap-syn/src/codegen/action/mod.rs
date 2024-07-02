use crate::ActionStruct;
use quote::quote;
mod query;
mod params;
mod to_metadata;

pub fn generate(action_struct: &ActionStruct) -> proc_macro2::TokenStream {
    let to_metadata = to_metadata::generate(action_struct);
    let query = query::generate(action_struct);
    let params = params::generate(action_struct);

    quote! {
        #to_metadata
        #query
        #params
    }
}
