use proc_macro::TokenStream;
use znap_syn::prelude::*;
use znap_syn::ActionStruct;

#[proc_macro_derive(Action, attributes(action))]
pub fn action_derive_macro(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ActionStruct)
        .to_token_stream()
        .into()
}
