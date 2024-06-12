use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(action))]
struct ActionStructAttributes {
    icon: String,
    title: String,
    description: String,
    label: String,
}

fn action_derive_macro2(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // parse
    let mut ast: DeriveInput = syn::parse2(item)?;

    // extract struct attributes
    let ActionStructAttributes {
        icon,
        title,
        description,
        label,
    } = deluxe::extract_attributes(&mut ast)?;

    // define impl variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    // generate
    Ok(quote::quote! {
        impl #impl_generics Action for #ident #type_generics #where_clause {
            fn to_metadata() -> ActionMetadata {
                ActionMetadata {
                    icon: #icon,
                    title: #title,
                    description: #description,
                    label: #label,
                }
            }
        }
    })
}

#[proc_macro_derive(Action, attributes(action))]
pub fn action_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    action_derive_macro2(item.into()).unwrap().into()
}
