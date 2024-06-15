use crate::ErrorEnum;
use common::extract_error_message;
use proc_macro2::TokenStream;
use quote::quote;
pub mod common;

pub fn generate(error_enum: &ErrorEnum) -> proc_macro2::TokenStream {
    let error_enum_raw_enum = &error_enum.raw_enum;
    let error_enum_ident = &error_enum.name;

    let display_variant_dispatch: Vec<TokenStream> = error_enum_raw_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let error_message = extract_error_message(variant).unwrap();

            quote::quote! {
                #error_enum_ident::#variant_ident => write!(fmt, #error_message)
            }
        })
        .collect();

    let name_variant_dispatch: Vec<proc_macro2::TokenStream> = error_enum_raw_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_ident_name = variant_ident.to_string();
            quote! {
                #error_enum_ident::#variant_ident => #variant_ident_name.to_string()
            }
        })
        .collect();

    quote! {
        impl #error_enum_ident {
            pub fn name(&self) -> String {
                match self {
                    #(#name_variant_dispatch),*
                }
            }
        }

        impl From<#error_enum_ident> for u32 {
            fn from(e: #error_enum_ident) -> u32 {
                e as u32
            }
        }

        impl From<#error_enum_ident> for Error {
            fn from(error_code: #error_enum_ident) -> Error {
                Error::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    error_code.name(),
                    error_code.to_string(),
                )
            }
        }

        impl std::fmt::Display for #error_enum_ident {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                match self {
                    #(#display_variant_dispatch),*
                }
            }
        }
    }
}
