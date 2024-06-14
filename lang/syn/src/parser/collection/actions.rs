use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::Ident;
use syn::{
    parse::{Error as ParseError, Result as ParseResult},
    Item, ItemFn, ItemMod,
};
use convert_case::{Case, Casing};
use crate::codegen::collection::common::extract_action_ident;
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
            let action_ident = extract_action_ident(&method).unwrap();
            let handle_get_name = format!("handle_get_{}", action_ident.to_string().to_case(Case::Snake));
            let handle_get_ident = Ident::new(&handle_get_name, Span::call_site());
            let handle_post_name = format!("handle_post_{}", action_ident.to_string().to_case(Case::Snake));
            let handle_post_ident = Ident::new(&handle_post_name, Span::call_site());

            Ok(ActionFn {
                raw_method: method.clone(),
                name: method.sig.ident.clone(),
                handle_get_ident,
                handle_post_ident,
                action_ident: action_ident.clone()
            })
        })
        .collect::<ParseResult<Vec<ActionFn>>>()?;

    Ok(actions_fns)
}
