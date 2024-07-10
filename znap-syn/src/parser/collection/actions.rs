use crate::common::{extract_action_ident, extract_fn_result_type};
use syn::{
    parse::{Error as ParseError, Result as ParseResult},
    spanned::Spanned,
    Ident, Item, ItemFn, ItemMod,
};

pub fn parse(collection_mod: &ItemMod) -> ParseResult<Vec<Ident>> {
    let mod_content = &collection_mod
        .content
        .as_ref()
        .ok_or_else(|| ParseError::new(collection_mod.span(), "collection content not provided"))?
        .1;

    let post_action_idents = mod_content
        .iter()
        .filter_map(|item| match item {
            Item::Fn(item_fn) => {
                if extract_fn_result_type(item_fn).unwrap() == "ActionTransaction" {
                    return Some(item_fn);
                }

                None
            }
            _ => None,
        })
        .map(|method: &ItemFn| {
            let action_ident = extract_action_ident(method).unwrap();

            Ok(action_ident.clone())
        })
        .collect::<ParseResult<Vec<Ident>>>()?;

    Ok(post_action_idents)
}
