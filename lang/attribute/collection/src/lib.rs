use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn collection(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    parse_macro_input!(input as znap_syn::CollectionMod)
        .to_token_stream()
        .into()
}
