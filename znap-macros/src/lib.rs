use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use znap_syn::{ActionStruct, CollectionMod, ErrorEnum};

#[proc_macro_derive(Action, attributes(action, query, params, action_path))]
pub fn action_derive_macro(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ActionStruct)
        .to_token_stream()
        .into()
}

#[proc_macro_derive(ErrorCode, attributes(error))]
pub fn error_code_derive_macro(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ErrorEnum)
        .to_token_stream()
        .into()
}

#[proc_macro_attribute]
pub fn collection(_args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as CollectionMod)
        .to_token_stream()
        .into()
}
