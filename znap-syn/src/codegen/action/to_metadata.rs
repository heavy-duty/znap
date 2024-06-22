use crate::{ActionAttributesStruct, ActionLinkParameterStruct, ActionLinkStruct, ActionStruct};
use proc_macro2::TokenStream;
use quote::quote;

fn generate_parameter(parameters: &[ActionLinkParameterStruct]) -> TokenStream {
    let parameters: Vec<_> = parameters
        .iter()
        .map(|p| {
            let label = &p.label;
            let name = &p.name;

            quote! {
                ActionLinkParameterMetadata {
                    label: #label,
                    name: #name,
                }
            }
        })
        .collect();
    quote! {
        &[ #(#parameters),* ]
    }
}

fn generate_links(links: &[ActionLinkStruct]) -> TokenStream {
    let links: Vec<_> = links
        .iter()
        .map(|l| {
            let label = &l.label;
            let href = &l.href;
            let params = generate_parameter(&l.parameters);

            quote! {
                ActionLinkMetadata {
                    label: #label,
                    href: #href,
                    parameters: #params,
                }
            }
        })
        .collect();

    quote! {
        &[ #(#links),* ]
    }
}

pub fn generate(action_struct: &ActionStruct) -> TokenStream {
    let ActionStruct {
        name, attributes, ..
    } = action_struct;
    let ActionAttributesStruct {
        title,
        description,
        label,
        icon,
        links,
    } = attributes;

    let links = generate_links(links);

    quote! {
        impl ToMetadata for #name {
            fn to_metadata(&self) -> ActionMetadata {
                ActionMetadata {
                    icon: #icon,
                    title: #title,
                    description: #description,
                    label: #label,
                    links: #links,
                }
            }
        }
    }
}
