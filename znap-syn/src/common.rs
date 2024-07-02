use deluxe::parse2;
use heck::{ToSnekCase, ToUpperCamelCase};
use proc_macro2::Span;
use quote::ToTokens;
use syn::{FnArg, GenericArgument, Ident, ItemFn, ItemStruct, PathArguments, ReturnType, Type};

pub fn extract_attrs_by_name(name: &str, action_struct: &ItemStruct) -> Option<Vec<(Ident, Ident)>> {
    action_struct.attrs.iter().find_map(|attr| {
        if let Ok(meta) = attr.meta.require_list() {
            if let Some(first_segment) = meta.path.segments.first() {
                if first_segment.ident.to_string() == name {
                    let idents: &Vec<Ident> = &meta
                        .tokens
                        .clone()
                        .into_iter()
                        .filter_map(|token| {
                            if let proc_macro2::TokenTree::Ident(ident) = token {
                                let ident_as_token_stream = ident.to_token_stream();

                                let parsed_ident = parse2::<Ident>(ident_as_token_stream).unwrap();

                                return Some(parsed_ident);
                            }

                            return None;
                        })
                        .collect();
                    let chunked_idents = idents.chunks(2);
                    let transformed_chunked_idents: Vec<(Ident, Ident)> = chunked_idents
                        .map(|chunk| {
                            (
                                chunk.first().unwrap().clone(),
                                chunk.last().unwrap().clone(),
                            )
                        })
                        .collect();

                    return Some(transformed_chunked_idents);
                }
            }
        }

        return None;
    })
}



pub fn extract_action_ident(f: &ItemFn) -> Option<&Ident> {
    if let FnArg::Typed(pt) = f.sig.inputs.first()? {
        if let Type::Path(type_path) = pt.ty.as_ref() {
            if let PathArguments::AngleBracketed(inner_path) =
                &type_path.path.segments.first()?.arguments
            {
                if let GenericArgument::Type(inner_type) = inner_path.args.first()? {
                    if let Type::Path(inner_type_path) = inner_type {
                        return inner_type_path.path.segments.first().map(|seg| &seg.ident);
                    }
                }
            }
        }
    }
    None
}

pub fn extract_action_query(f: &ItemFn) -> Option<&Ident> {
    if let FnArg::Typed(pt) = f.sig.inputs.first()? {
        if let Type::Path(type_path) = pt.ty.as_ref() {
            if type_path.path.segments.first()?.ident.to_string() == "Context" {
                if let PathArguments::AngleBracketed(inner_path) =
                    &type_path.path.segments.first()?.arguments
                {
                    if inner_path.args.len() != 2 {
                        return None;
                    }

                    if let GenericArgument::Type(inner_type) = inner_path.args.last()? {
                        if let Type::Path(inner_type_path) = inner_type {
                            return inner_type_path.path.segments.first().map(|seg| &seg.ident);
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn action_name_without_suffix(action_name: &String) -> String {
    let action_name_splitted: Vec<&str> = action_name.split("_").collect();
    let (_, action_name_without_suffix) = action_name_splitted.split_last().unwrap();

    action_name_without_suffix.join("_")
}

pub fn extract_fn_result_type(f: &ItemFn) -> Option<&Ident> {
    if let ReturnType::Type(_, ty) = &f.sig.output {
        if let Type::Path(type_path) = ty.as_ref() {
            if let PathArguments::AngleBracketed(inner_path) =
                &type_path.path.segments.first().unwrap().arguments
            {
                if let GenericArgument::Type(inner_type) = inner_path.args.first().unwrap() {
                    if let Type::Path(inner_type_path) = inner_type {
                        return Some(&inner_type_path.path.segments.first().unwrap().ident);
                    }
                }
            }
        }
    }
    None
}

pub fn create_query(action: &String) -> Ident {
    Ident::new(
        &format!(
            "{}Query",
            action_name_without_suffix(&action.to_snek_case()).to_upper_camel_case()
        ),
        Span::call_site(),
    )
}

pub fn create_params(action: &String) -> Ident {
    Ident::new(
        &format!(
            "{}Params",
            action_name_without_suffix(&action.to_snek_case()).to_upper_camel_case()
        ),
        Span::call_site(),
    )
}

pub fn create_get_handler(action: &String) -> Ident {
    Ident::new(
        &format!(
            "handle_get_{}",
            action_name_without_suffix(&action.to_snek_case())
        ),
        Span::call_site(),
    )
}

pub fn create_post_handler(action: &String) -> Ident {
    Ident::new(
        &format!(
            "handle_post_{}",
            action_name_without_suffix(&action.to_snek_case())
        ),
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
        &format!(
            "{}_create_transaction",
            action_name_without_suffix(&action.to_snek_case())
        ),
        Span::call_site(),
    )
}

pub fn create_metadata(action: &String) -> Ident {
    Ident::new(
        &format!(
            "{}_create_metadata",
            action_name_without_suffix(&action.to_snek_case())
        ),
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
