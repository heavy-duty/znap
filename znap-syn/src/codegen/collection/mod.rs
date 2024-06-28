use crate::CollectionMod;
use quote::quote;
mod create_transaction;
mod create_metadata;
mod handle_get;
mod handle_post;
mod router;
mod display_routes;
mod common;

pub fn generate(collection_mod: &CollectionMod) -> proc_macro2::TokenStream {
    let create_transaction = create_transaction::generate(collection_mod);
    let create_metadata = create_metadata::generate(collection_mod);
    let handle_post = handle_post::generate(collection_mod);
    let handle_get = handle_get::generate(collection_mod);
    let router = router::generate(collection_mod);
    let display_routes = display_routes::generate(collection_mod);

    quote! {
        use znap::base64::prelude::BASE64_STANDARD;
        use znap::base64::Engine;
        use colored::Colorize;

        #create_transaction
        #create_metadata
        #handle_post
        #handle_get
        #router
        #display_routes
    }
}
