use crate::{ActionFn, CollectionMod};
use syn::{parse::Result as ParseResult, ItemMod};
mod post_action_fns;
mod get_action_fns;
mod action_idents;
pub mod common;

pub fn parse(collection_mod: &ItemMod) -> ParseResult<CollectionMod> {
    let post_action_fns = post_action_fns::parse(&collection_mod)?;
    let get_action_fns = get_action_fns::parse(&collection_mod)?;
    let action_idents = action_idents::parse(&collection_mod)?;
    let action_fns: Vec<ActionFn>  =  vec!();

    Ok(CollectionMod {
        action_idents,
        action_fns,
        post_action_fns,
        get_action_fns,
        name: collection_mod.ident.clone(),
        collection_mod: collection_mod.clone(),
    })
}
