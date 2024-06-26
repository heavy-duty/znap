use crate::{ActionAttributesStruct, ActionLinkParameterStruct, ActionLinkStruct, ActionStruct};
use proc_macro2::TokenStream;
use quote::quote;

fn generate_parameter(parameters: &[ActionLinkParameterStruct]) -> TokenStream {
    let parameters: Vec<_> = parameters
        .iter()
        .map(|p| {
            let label = &p.label;
            let name = &p.name;
            let required = p.required;

            quote! {
                LinkedActionParameter {
                    label: #label,
                    name: #name,
                    required: #required,
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
                LinkedAction {
                    label: #label,
                    href: #href,
                    parameters: #params,
                }
            }
        })
        .collect();

    if links.len() == 0 {
        quote! {
            &None
        }
    } else {
        quote! {
            &Some(
                ActionLinks {
                    actions: &[ #(#links),* ]
                }
            )        
        }
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
            fn to_metadata() -> ActionMetadata {
                ActionMetadata {
                    icon: #icon,
                    title: #title,
                    description: #description,
                    label: #label,
                    links: #links,
                    disabled: false,
                    error: None,
                }
            }
        }
    }
}
