use crate::CollectionMod;
use syn::{parse::Result as ParseResult, ItemMod};
mod actions;
mod get_action_fns;
mod post_action_fns;

pub fn parse(collection_mod: &ItemMod) -> ParseResult<CollectionMod> {
    let post_action_fns = post_action_fns::parse(&collection_mod)?;
    let get_action_fns = get_action_fns::parse(&collection_mod)?;
    let actions = actions::parse(&collection_mod)?;

    Ok(CollectionMod {
        actions,
        post_action_fns,
        get_action_fns,
        name: collection_mod.ident.clone(),
        collection_mod: collection_mod.clone(),
    })
}
