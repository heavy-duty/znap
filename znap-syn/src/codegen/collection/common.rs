use crate::parser::collection::common::action_name_without_suffix;
use heck::ToSnekCase;
use proc_macro2::Span;
use syn::Ident;

pub fn create_get_handler(action: &String) -> Ident {
    Ident::new(
        &format!("handle_get_{}", action.to_snek_case()),
        Span::call_site(),
    )
}

pub fn create_post_handler(action: &String) -> Ident {
    Ident::new(
        &format!("handle_post_{}", action.to_snek_case()),
        Span::call_site(),
    )
}

pub fn create_route_path(action: &String) -> String {
    format!(
        "/api/{}",
        action_name_without_suffix(&action.to_snek_case()).to_snek_case()
    )
}
