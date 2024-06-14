use crate::{codegen::collection::common::extract_action_ident, CollectionMod};

pub fn generate(collection_mod: &CollectionMod) -> proc_macro2::TokenStream {
    let impls: Vec<proc_macro2::TokenStream> = collection_mod
        .action_fns
        .iter()
        .map(|action_fn| {
            let action_ident = extract_action_ident(&action_fn.raw_method).unwrap();

            quote::quote! {
                impl HandleGetAction for #action_ident {
                    fn handle_get_action() -> Result<axum::Json<ActionMetadata>, Error> {
                        let action = #action_ident;

                        Ok(axum::Json(action.to_metadata()))
                    }
                }
            }
        })
        .collect();

    quote::quote! {
        #(#impls)*
    }
}
