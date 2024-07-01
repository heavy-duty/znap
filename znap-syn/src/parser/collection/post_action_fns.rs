use crate::{common::{extract_action_ident, extract_action_query, extract_fn_result_type}, PostActionFn};
use syn::{
    parse::{Error as ParseError, Result as ParseResult},
    spanned::Spanned,
    Item, ItemFn, ItemMod,
};

pub fn parse(collection_mod: &ItemMod) -> ParseResult<Vec<PostActionFn>> {
    let mod_content = &collection_mod
        .content
        .as_ref()
        .ok_or_else(|| ParseError::new(collection_mod.span(), "collection content not provided"))?
        .1;

    let post_actions_fns = mod_content
        .iter()
        .filter_map(|item| match item {
            Item::Fn(item_fn) => {
                if extract_fn_result_type(&item_fn).unwrap() == "Transaction" {
                    return Some(item_fn);
                }

                None
            }
            _ => None,
        })
        .map(|method: &ItemFn| {
            let action_ident = extract_action_ident(&method).unwrap();
            let query_ident = match extract_action_query(&method) {
                Some(ident) => Some(ident.clone()),
                _ => None,
            };

            Ok(PostActionFn {
                raw_method: method.clone(),
                name: method.sig.ident.clone(),
                action: action_ident.clone(),
                query: query_ident,
            })
        })
        .collect::<ParseResult<Vec<PostActionFn>>>()?;

    Ok(post_actions_fns)
}
