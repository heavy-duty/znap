use proc_macro::TokenStream;
use znap_syn::prelude::*;
use znap_syn::QueryStruct;

#[proc_macro_attribute]
pub fn query(_args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as QueryStruct)
        .to_token_stream()
        .into()
}
