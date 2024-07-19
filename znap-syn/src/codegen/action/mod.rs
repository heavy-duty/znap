use crate::ActionStruct;
use quote::quote;
mod name;
mod params;
mod path;
mod query;
mod to_metadata;

pub fn generate(action_struct: &ActionStruct) -> proc_macro2::TokenStream {
    let (route, path) = path::generate(action_struct);
    let to_metadata = to_metadata::generate(action_struct, &route);
    let query = query::generate(action_struct);
    let params = params::generate(action_struct);
    let name = name::generate(action_struct);

    quote! {
        #to_metadata
        #query
        #params
        #path
        #name
    }
}
