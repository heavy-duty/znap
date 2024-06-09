use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(action))]
struct ActionStructAttributes {
    icon: String,
    title: String,
    description: String,
    label: String,
}

fn actions_derive_macro2(
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
            fn icon(&self) -> &'static str {
                #icon
            }

            fn title(&self) -> &'static str {
                #title
            }

            fn description(&self) -> &'static str {
                #description
            }

            fn label(&self) -> &'static str {
                #label
            }
        }
    })
}

#[proc_macro_derive(Action, attributes(action, query))]
pub fn actions_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    actions_derive_macro2(item.into()).unwrap().into()
}
