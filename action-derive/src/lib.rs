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

    let to_metadata_impl = quote::quote! {
        impl ToMetadata for #ident {
            fn to_metadata(&self) -> ActionMetadata {
                ActionMetadata {
                    icon: #icon,
                    title: #title,
                    description: #description,
                    label: #label,
                }
            }
        }
    };

    // generate
    Ok(quote::quote! {
        #to_metadata_impl
    })
}

#[proc_macro_derive(Action, attributes(action, query))]
pub fn action_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    action_derive_macro2(item.into()).unwrap().into()
}
