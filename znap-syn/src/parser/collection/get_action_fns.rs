use super::common::{extract_action_query, extract_fn_result_type};
use crate::parser::collection::common::extract_action_ident;
use crate::GetActionFn;
use heck::ToSnekCase;
use proc_macro2::Span;
use syn::{
    parse::{Error as ParseError, Result as ParseResult},
    spanned::Spanned,
    Ident, Item, ItemFn, ItemMod,
};

pub fn parse(collection_mod: &ItemMod) -> ParseResult<Vec<GetActionFn>> {
    let mod_content = &collection_mod
        .content
        .as_ref()
        .ok_or_else(|| ParseError::new(collection_mod.span(), "collection content not provided"))?
        .1;

    let get_actions_fns = mod_content
        .iter()
        .filter_map(|item| match item {
            Item::Fn(item_fn) => {
                if extract_fn_result_type(&item_fn).unwrap() == "ActionMetadata" {
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
                Ident::new(&format!("handle_get_{}", action_name), Span::call_site());
            let action_query_ident = match extract_action_query(&method) {
                Some(ident) => Some(ident.clone()),
                _ => None,
            };

            Ok(GetActionFn {
                raw_method: method.clone(),
                name: method.sig.ident.clone(),
                handle_ident,
                action_ident: action_ident.clone(),
                action_query_ident: action_query_ident.clone(),
            })
        })
        .collect::<ParseResult<Vec<GetActionFn>>>()?;

    Ok(get_actions_fns)
}
