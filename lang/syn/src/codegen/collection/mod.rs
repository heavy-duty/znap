use crate::CollectionMod;
use quote::quote;
mod create_transaction;
mod handle_get;
mod handle_post;
mod router;

pub fn generate(collection_mod: &CollectionMod) -> proc_macro2::TokenStream {
    let handle_post = handle_post::generate(collection_mod);
    let handle_get = handle_get::generate(collection_mod);
    let create_transaction = create_transaction::generate(collection_mod);
    let router = router::generate(collection_mod);

    quote! {
        #create_transaction
        #handle_post
        #handle_get
        #router
    }
}
