use super::common::action_name_without_suffix;
use crate::parser::collection::common::{extract_action_ident, extract_action_query};
use crate::ActionFn;
use heck::ToSnekCase;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::Ident;
use syn::{
    parse::{Error as ParseError, Result as ParseResult},
    Item, ItemFn, ItemMod,
};

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
            let action_ident = extract_action_ident(&method).unwrap();
            let action_name = action_ident.to_string().to_snek_case();
            let handle_get_ident =
                Ident::new(&format!("handle_get_{}", action_name), Span::call_site());
            let handle_post_ident =
                Ident::new(&format!("handle_post_{}", action_name), Span::call_site());
            let action_query_ident = match extract_action_query(&method) {
                Some(ident) => Some(ident.clone()),
                _ => None,
            };
            let route_path = format!(
                "/api/{}",
                action_name_without_suffix(action_name).to_snek_case()
            );

            Ok(ActionFn {
                raw_method: method.clone(),
                name: method.sig.ident.clone(),
                handle_get_ident,
                handle_post_ident,
                action_ident: action_ident.clone(),
                action_query_ident,
                route_path,
            })
        })
        .collect::<ParseResult<Vec<ActionFn>>>()?;

    Ok(actions_fns)
}
