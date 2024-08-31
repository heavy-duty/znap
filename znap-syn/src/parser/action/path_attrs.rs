use crate::common::has_path;
use crate::PathAttributesStruct;
use deluxe::extract_attributes;
use quote::ToTokens;
use syn::{parse::Result as ParseResult, parse2, ItemStruct};

pub fn parse(path_struct: &ItemStruct) -> ParseResult<Option<PathAttributesStruct>> {
    if has_path(path_struct) {
        let mut path_derive_input = parse2::<syn::DeriveInput>(path_struct.to_token_stream())?;

        let attributes: PathAttributesStruct = extract_attributes(&mut path_derive_input)?;

        Ok(Some(attributes))
    } else {
        Ok(None)
    }
}
