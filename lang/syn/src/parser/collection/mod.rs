use crate::CollectionMod;
use syn::{parse::Result as ParseResult, ItemMod};
mod actions;
pub mod common;

pub fn parse(collection_mod: &ItemMod) -> ParseResult<CollectionMod> {
    let action_fns = actions::parse(&collection_mod)?;

    Ok(CollectionMod {
        action_fns,
        name: collection_mod.ident.clone(),
        collection_mod: collection_mod.clone(),
    })
}
