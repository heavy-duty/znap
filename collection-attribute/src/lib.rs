use proc_macro2::{Ident, Span};
use syn::{Item, ItemMod};

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

    // TODO: Generate implementation for create_transaction for the actions

    // Generate the trait implementation based on the found functions
    let handlers = generate_handlers(&functions);

    // TODO: Connect handler methods with the concrete action implementations

    // TODO: Remove original function on behalf of generated one

    // Combine the original module with the generated trait implementation
    let result = quote::quote! {
        #input_module
        #handlers
    };

    Ok(result.into())
}

fn generate_handlers(functions: &[syn::ItemFn]) -> proc_macro2::TokenStream {
    let impls: Vec<proc_macro2::TokenStream> = functions.iter().map(|f| {
        let fn_name = &f.sig.ident;

        let get_handler_fn_name: String = format!("handle_get_{}", fn_name.to_string());
        let get_handler_ident = &Ident::new(&get_handler_fn_name, Span::call_site());
        let post_handler_fn_name: String = format!("handle_post_{}", fn_name.to_string());
        let post_handler_ident = &Ident::new(&post_handler_fn_name, Span::call_site());

        quote::quote! {
            async fn #get_handler_ident() -> Result<axum::Json<ActionMetadata>, axum::Error> {
                Ok(axum::Json(ActionMetadata { title: "".to_string(), description: "".to_string(), icon: "".to_string(), label: "".to_string() }))
            }
    
            async fn #post_handler_ident(
                axum::Json(payload): axum::Json<CreateActionPayload>,
            ) -> Result<axum::Json<CreateActionResponse>, Error> {
                Ok(axum::Json(CreateActionResponse {
                    transaction: "".to_string(),
                    message: None
                }))
            }
        }
    }).collect();

    quote::quote! {
        #(#impls)*
    }
}

#[proc_macro_attribute]
pub fn collection_attribute_macro(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    collection_attribute_macro2(input.into()).unwrap().into()
}
