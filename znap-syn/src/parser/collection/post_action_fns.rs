use super::common::extract_fn_result_type;
use crate::parser::collection::common::{extract_action_ident, extract_action_query};
use crate::PostActionFn;
use heck::ToSnekCase;
use proc_macro2::Span;
use syn::{
    parse::{Error as ParseError, Result as ParseResult},
    spanned::Spanned,
    Ident, Item, ItemFn, ItemMod,
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
            let action_name = action_ident.to_string().to_snek_case();
            let handle_ident =
                Ident::new(&format!("handle_post_{}", action_name), Span::call_site());
            let action_query_ident = match extract_action_query(&method) {
                Some(ident) => Some(ident.clone()),
                _ => None,
            };

            Ok(PostActionFn {
                raw_method: method.clone(),
                name: method.sig.ident.clone(),
                handle_ident,
                action_ident: action_ident.clone(),
                action_query_ident,
            })
        })
        .collect::<ParseResult<Vec<PostActionFn>>>()?;

    Ok(post_actions_fns)
}
