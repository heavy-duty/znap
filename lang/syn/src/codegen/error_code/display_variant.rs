use crate::ErrorEnum;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(error_enum: &ErrorEnum) -> TokenStream {
    let error_name = &error_enum.name;
    let display_variant: Vec<TokenStream> = error_enum
        .error_variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.name;
            let error_message = &variant.msg;

            quote! {
                #error_name::#variant_ident => write!(fmt, #error_message)
            }
        })
        .collect();

    quote! {
        impl std::fmt::Display for #error_name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                match self {
                    #(#display_variant),*
                }
            }
        }
    }
}
