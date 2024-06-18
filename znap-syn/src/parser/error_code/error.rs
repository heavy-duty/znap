use crate::{ErrorAttributesStruct, ErrorVariant};
use deluxe::extract_attributes;
use syn::{parse::Result as ParseResult, ItemEnum};

pub fn parse(error_enum: &ItemEnum) -> ParseResult<Vec<ErrorVariant>> {
    let variant_attrs: Vec<ErrorVariant> = error_enum
        .variants
        .clone()
        .iter_mut()
        .map(|variant| {
            let variant_name = variant.ident.clone();
            let ErrorAttributesStruct { msg } = extract_attributes(variant).unwrap();

            ErrorVariant {
                msg,
                name: variant_name,
                raw_variant: variant.clone(),
            }
        })
        .collect();

    Ok(variant_attrs)
}
