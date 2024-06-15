use crate::QueryStruct;
use quote::quote;

pub fn generate(query_struct: &QueryStruct) -> proc_macro2::TokenStream {
    let query_raw_struct = &query_struct.raw_struct;

    quote! {
        #[derive(Debug, serde::Deserialize)]
        #query_raw_struct
    }
}
