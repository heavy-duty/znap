use syn::{FnArg, GenericArgument, Ident, ItemFn, PathArguments, Type};

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
            if type_path.path.segments.first()?.ident == "ContextWithQuery" {
                if let PathArguments::AngleBracketed(inner_path) =
                    &type_path.path.segments.first()?.arguments
                {
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

pub fn action_name_without_suffix(action_name: String) -> String {
    let action_name_splitted: Vec<&str> = action_name
        .split("_")
        .collect();
    let (_, action_name_without_suffix) = action_name_splitted.split_last().unwrap();
    
    format!("/actions/{}", action_name_without_suffix.join("_"))
}
