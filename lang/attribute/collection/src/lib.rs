use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use znap_syn::CollectionMod;

#[proc_macro_attribute]
pub fn collection(_args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as CollectionMod)
        .to_token_stream()
        .into()
}