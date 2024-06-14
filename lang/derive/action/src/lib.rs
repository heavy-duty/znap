use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use znap_syn::ActionStruct;

#[proc_macro_derive(Action, attributes(action, query))]
pub fn action_derive_macro(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ActionStruct)
        .to_token_stream()
        .into()
}
