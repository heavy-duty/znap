use syn::{FnArg, GenericArgument, Ident, PathArguments, Type};

pub fn extract_action_ident(f: &syn::ItemFn) -> Option<&Ident> {
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

pub fn extract_action_query(f: &syn::ItemFn) -> Option<&Ident> {
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
