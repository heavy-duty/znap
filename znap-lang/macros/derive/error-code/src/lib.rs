use proc_macro::TokenStream;
use znap_syn::prelude::*;
use znap_syn::ErrorEnum;

#[proc_macro_derive(ErrorCode, attributes(error))]
pub fn error_code_derive_macro(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ErrorEnum)
        .to_token_stream()
        .into()
}
