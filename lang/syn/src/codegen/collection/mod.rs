use crate::CollectionMod;
use quote::quote;

pub fn generate(collection_mod: &CollectionMod) -> proc_macro2::TokenStream {
    let mod_name = &collection_mod.name;

    quote! {
        // TODO: remove once we allow segmented paths in `Accounts` structs.
        
    }
}