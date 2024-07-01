use crate::parser::collection::common::action_name_without_suffix;
use heck::{ToSnekCase, ToUpperCamelCase};
use proc_macro2::Span;
use syn::Ident;

pub fn create_get_handler(action: &String) -> Ident {
    Ident::new(
        &format!("handle_get_{}", action_name_without_suffix(&action.to_snek_case())),
        Span::call_site(),
    )
}

pub fn create_post_handler(action: &String) -> Ident {
    Ident::new(
        &format!("handle_post_{}", action_name_without_suffix(&action.to_snek_case())),
        Span::call_site(),
    )
}

pub fn create_route_path(action: &String) -> String {
    format!(
        "/api/{}",
        action_name_without_suffix(&action.to_snek_case()).to_snek_case()
    )
}

pub fn create_post_context(action: &String) -> Ident {
    Ident::new(
        &format!(
            "{}PostContext",
            action_name_without_suffix(&action.to_snek_case()).to_upper_camel_case()
        ),
        Span::call_site(),
    )
}

pub fn create_transaction(action: &String) -> Ident {
    Ident::new(
        &format!("{}_create_transaction", action_name_without_suffix(&action.to_snek_case())),
        Span::call_site(),
    )
}

pub fn create_metadata(action: &String) -> Ident {
    Ident::new(
        &format!("{}_create_metadata", action_name_without_suffix(&action.to_snek_case())),
        Span::call_site(),
    )
}

pub fn create_get_context(action: &String) -> Ident {
    Ident::new(
        &format!(
            "{}GetContext",
            action_name_without_suffix(&action.to_snek_case()).to_upper_camel_case()
        ),
        Span::call_site(),
    )
}
