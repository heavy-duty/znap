use proc_macro2::Ident;
use syn::{Block, FnArg, GenericArgument, Item, ItemMod, PathArguments, Type};

fn extract_action_ident(f: &syn::ItemFn) -> Option<&Ident> {
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

fn extract_action_query(f: &syn::ItemFn) -> Option<&Ident> {
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

fn generate_impls_with_query(
    action_ident: &Ident,
    fn_block: &Box<Block>,
    action_query_type_ident: &Ident,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    (
        quote::quote! {
            impl CreateTransactionWithQuery<#action_ident, #action_query_type_ident> for #action_ident {
                fn create_transaction(&self, ctx: ContextWithQuery<#action_ident, #action_query_type_ident>) -> Result<solana_sdk::transaction::Transaction, Error> {
                    #fn_block
                }
            }
        },
        quote::quote! {
            impl HandlePostActionWithQuery<#action_query_type_ident> for #action_ident {
                fn handle_post_action(
                    axum::Json(payload): axum::Json<CreateActionPayload>,
                    axum::extract::Query(query): axum::extract::Query<#action_query_type_ident>
                ) -> Result<axum::Json<ActionTransaction>, Error> {
                    let action = #action_ident;
                    let context_with_query = ContextWithQuery::<#action_ident, #action_query_type_ident> {
                        payload,
                        action: PhantomData,
                        query
                    };
                    let transaction = action.create_transaction(context_with_query).unwrap();
                    let serialized_transaction = bincode::serialize(&transaction).unwrap();
                    let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);

                    Ok(axum::Json(ActionTransaction {
                        transaction: encoded_transaction,
                        message: None
                    }))
                }
            }
        },
    )
}

fn generate_impls_without_query(
    action_ident: &Ident,
    fn_block: &Box<Block>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    (
        quote::quote! {
            impl CreateTransaction<#action_ident> for #action_ident {
                fn create_transaction(&self, ctx: Context<#action_ident>) -> Result<solana_sdk::transaction::Transaction, Error> {
                    #fn_block
                }
            }
        },
        quote::quote! {
            impl HandlePostAction for #action_ident {
                fn handle_post_action(axum::Json(payload): axum::Json<CreateActionPayload>) -> Result<axum::Json<ActionTransaction>, Error> {
                    let action = #action_ident {};
                    let context = Context::<#action_ident> {
                        payload,
                        action: PhantomData,
                    };
                    let transaction = action.create_transaction(context).unwrap();
                    let serialized_transaction = bincode::serialize(&transaction).unwrap();
                    let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);

                    Ok(axum::Json(ActionTransaction {
                        transaction: encoded_transaction,
                        message: None
                    }))
                }
            }
        },
    )
}

fn generate_handle_get_action_impl(action_ident: &Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl HandleGetAction for #action_ident {
            fn handle_get_action() -> Result<axum::Json<ActionMetadata>, Error> {
                let action = #action_ident;

                Ok(axum::Json(action.to_metadata()))
            }
        }
    }
}

fn generate_trait_impl(functions: &[syn::ItemFn]) -> proc_macro2::TokenStream {
    let impls: Vec<proc_macro2::TokenStream> = functions
        .iter()
        .map(|f| {
            let action_ident = extract_action_ident(&f).unwrap();
            let fn_block = &f.block;
            let handle_get_action_impl = generate_handle_get_action_impl(action_ident);

            let (create_transaction_impl, handle_post_action_impl) =
                if let Some(action_query_type_ident) = extract_action_query(&f) {
                    generate_impls_with_query(action_ident, fn_block, action_query_type_ident)
                } else {
                    generate_impls_without_query(action_ident, fn_block)
                };

            quote::quote! {
                #create_transaction_impl
                #handle_post_action_impl
                #handle_get_action_impl
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
    // Parse the input tokens into a syntax tree
    let input_module: ItemMod = syn::parse(input).unwrap();

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

    // Combine the original module with the generated trait implementation
    let result = quote::quote! {
        #input_module
        #trait_impls
    };

    result.into()
}
