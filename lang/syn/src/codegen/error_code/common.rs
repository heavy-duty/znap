use syn::Variant;
use quote::ToTokens;

pub fn extract_error_message(variant: &Variant) -> Option<String> {
    let attr = &variant.attrs[0];
    let filtered: Vec<String> = attr.to_token_stream().into_iter().filter_map(|token_tree| {
        match token_tree {
            proc_macro2::TokenTree::Group(group) => {
                let stream = group.stream();
                let mut stream_iter = stream.into_iter();
                
                let maybe_msg_ident = stream_iter.next();

                match maybe_msg_ident {
                    Some(msg_ident) => {
                        if msg_ident.to_string() != "msg" {
                            panic!("Invalid attribute name")
                        }
                    },
                    _ => panic!("Message is missing")
                };
                
                let msg_literal = stream_iter.next().unwrap();

                Some(msg_literal.to_string().replace('\"', ""))
            },
            _ => None,
        }
    }).collect();

    match filtered.first() {
        Some(message) => Some(message.clone()),
        _ => None
    }
}