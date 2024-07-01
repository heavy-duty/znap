use crate::CollectionMod;
use quote::quote;
mod display_routes;
mod handle_get;
mod handle_post;
mod router;

pub fn generate(collection_mod: &CollectionMod) -> proc_macro2::TokenStream {
    let handle_post = handle_post::generate(collection_mod);
    let handle_get = handle_get::generate(collection_mod);
    let router = router::generate(collection_mod);
    let display_routes = display_routes::generate(collection_mod);

    quote! {
        use znap::base64::prelude::BASE64_STANDARD;
        use znap::base64::Engine;
        use colored::Colorize;

        #handle_post
        #handle_get
        #router
        #display_routes
    }
}
