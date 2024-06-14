use crate::CollectionMod;
use quote::quote;
pub mod common;
mod create_transaction;
mod handle_get;
mod handle_post;

pub fn generate(collection_mod: &CollectionMod) -> proc_macro2::TokenStream {
    let handle_post = handle_post::generate(collection_mod);
    let handle_get = handle_get::generate(collection_mod);
    let create_transaction = create_transaction::generate(collection_mod);

    quote! {
        #handle_post
        #handle_get
        #create_transaction
    }
}
