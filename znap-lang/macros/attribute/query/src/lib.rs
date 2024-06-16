use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use znap_syn::QueryStruct;

#[proc_macro_attribute]
pub fn query(_args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as QueryStruct)
        .to_token_stream()
        .into()
}
