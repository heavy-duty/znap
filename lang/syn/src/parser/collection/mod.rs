use crate::CollectionMod;
use syn::parse::Result as ParseResult;

mod actions;

pub fn parse(collection_mod: syn::ItemMod) -> ParseResult<CollectionMod> {
    let action_fns = actions::parse(&collection_mod)?;
    Ok(CollectionMod {
        action_fns,
        name: collection_mod.ident.clone(),
        collection_mod,
    })
}
