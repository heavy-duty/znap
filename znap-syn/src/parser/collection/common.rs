use syn::{FnArg, GenericArgument, Ident, ItemFn, PathArguments, ReturnType, Type};

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
            if type_path.path.segments.first()?.ident.to_string().contains("Context") {
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
                        return Some(&inner_type_path.path.segments.first().unwrap().ident)
                    }
                }
            }
        }
    }
    None
}
