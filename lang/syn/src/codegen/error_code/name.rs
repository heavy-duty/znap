use crate::ErrorEnum;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(error_enum: &ErrorEnum) -> TokenStream {
    let error_name = &error_enum.name;
    let name_variant: Vec<proc_macro2::TokenStream> = error_enum
        .error_variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.name;
            let variant_ident_name = variant_ident.to_string();
            quote! {
                #error_name::#variant_ident => #variant_ident_name.to_string()
            }
        })
        .collect();

    quote! {
        impl #error_name {
            pub fn name(&self) -> String {
                match self {
                    #(#name_variant),*
                }
            }
        }
    }
}
