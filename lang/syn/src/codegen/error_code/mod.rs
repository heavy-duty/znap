use crate::ErrorEnum;
use quote::quote;
mod display_variant;
mod name;

pub fn generate(error_enum: &ErrorEnum) -> proc_macro2::TokenStream {
    let error_name = &error_enum.name;
    let display_variants = display_variant::generate(error_enum);
    let name_variants = name::generate(error_enum);

    quote! {
        #name_variants
        #display_variants

        impl From<#error_name> for u32 {
            fn from(e: #error_name) -> u32 {
                e as u32
            }
        }

        impl From<#error_name> for Error {
            fn from(error_code: #error_name) -> Error {
                Error::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    error_code.name(),
                    error_code.to_string(),
                )
            }
        }
    }
}
