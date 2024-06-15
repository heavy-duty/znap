use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use znap_syn::ErrorEnum;

#[proc_macro_derive(ErrorCode, attributes(msg))]
pub fn error_code_derive_macro(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ErrorEnum)
        .to_token_stream()
        .into()
}
