use proc_macro2::Ident;
use syn::{FnArg, GenericArgument, Item, ItemMod, PathArguments, Type};

fn collection_attribute_macro2(
    input: proc_macro::TokenStream,
) -> Result<proc_macro::TokenStream, syn::Error> {
    // Parse the input tokens into a syntax tree
    let input_module: ItemMod = syn::parse(input)?;

    // Initialize an empty vector to hold function names
    let mut functions = Vec::new();

    // If the module has content, iterate over it to find function items
    if let Some((_, items)) = &input_module.content {
        for item in items {
            if let Item::Fn(item_fn) = item {
                functions.push(item_fn.clone());
            }
        }
    }

    // Generate implementation for create_transaction for the actions
    let trait_impls = generate_trait_impl(&functions);

    // TODO: Handle queries for handlers

    // TODO: Make sure action handlers return a Transaction

    // TODO: Generate Routes and main

    // TODO: Remove original function on behalf of generated one

    // Combine the original module with the generated trait implementation
    let result = quote::quote! {
        #input_module
        // #handlers
        #trait_impls
    };

    Ok(result.into())
}

fn extract_action_ident(f: &syn::ItemFn) -> Option<Ident> {
    let arg = f.sig.inputs.first()?;

    let pattern_type = match arg {
        FnArg::Typed(pt) => pt,
        _ => return None,
    };

    let type_path = match pattern_type.ty.as_ref() {
        Type::Path(tp) => tp,
        _ => return None,
    };

    let inner_path = match &type_path.path.segments.first()?.arguments {
        PathArguments::AngleBracketed(ip) => ip,
        _ => return None,
    };

    let inner_type = match inner_path.args.first()? {
        GenericArgument::Type(it) => it,
        _ => return None,
    };

    let inner_type_path = match inner_type {
        Type::Path(itp) => itp,
        _ => return None,
    };

    Some(inner_type_path.path.segments.first()?.ident.clone())
}

fn generate_trait_impl(functions: &[syn::ItemFn]) -> proc_macro2::TokenStream {
    let impls: Vec<proc_macro2::TokenStream> = functions
        .iter()
        .map(|f| {
            let action_ident = extract_action_ident(&f).unwrap();
            let fn_block = &f.block;

            let create_transaction_impl = quote::quote! {
                impl CreateTransaction<#action_ident> for #action_ident {
                    fn create_transaction(ctx: Context<#action_ident>) -> Result<String, Error> {
                        #fn_block
                    }
                }
            };

            let handle_get_action_impl = quote::quote! {
                impl HandleGetAction for #action_ident {
                    fn handle_get_action() -> Result<axum::Json<ActionMetadata>, Error> {
                        Ok(axum::Json(#action_ident::to_metadata()))
                    }
                }
            };

            // this is without query
            let handle_post_action_impl = quote::quote! {
                impl HandlePostAction for #action_ident {
                    fn handle_post_action(axum::Json(payload): axum::Json<CreateActionPayload>) -> Result<axum::Json<ActionTransaction>, Error> {
                        let context = Context::<#action_ident> {
                            payload,
                            action: #action_ident {},
                        };
                        let transaction = #action_ident::create_transaction(context).unwrap();
    
                        Ok(axum::Json(ActionTransaction {
                            transaction,
                            message: None
                        }))
                    }
                }
            };

            quote::quote! {
                #create_transaction_impl
                #handle_get_action_impl
                #handle_post_action_impl
            }
        })
        .collect();

    quote::quote! {
        #(#impls)*
    }
}

#[proc_macro_attribute]
pub fn collection(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    collection_attribute_macro2(input.into()).unwrap().into()
}
