use syn::{
    parse::{Error as ParseError, Result as ParseResult},
    Item, ItemFn, ItemMod,
};
use syn::spanned::Spanned;

use crate::ActionFn;

pub fn parse(collection_mod: &ItemMod) -> ParseResult<Vec<ActionFn>> {
    let mod_content = &collection_mod
        .content
        .as_ref()
        .ok_or_else(|| ParseError::new(collection_mod.span(), "collection content not provided"))?
        .1;

    let actions_fns = mod_content
        .iter()
        .filter_map(|item| match item {
            Item::Fn(item_fn) => Some(item_fn),
            _ => None,
        })
        .map(|method: &ItemFn| {
            Ok(ActionFn {
                raw_method: method.clone(),
                ident: method.sig.ident.clone(),
            })
        })
        .collect::<ParseResult<Vec<ActionFn>>>()?;

    Ok(actions_fns)
}
