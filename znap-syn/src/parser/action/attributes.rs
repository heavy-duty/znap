use deluxe::extract_attributes;
use quote::ToTokens;
use syn::{parse::Result as ParseResult, parse2, ItemStruct};

use crate::ActionAttributesStruct;

pub fn parse(action_struct: &ItemStruct) -> ParseResult<ActionAttributesStruct> {
    let mut action_derive_input = parse2::<syn::DeriveInput>(action_struct.to_token_stream())?;
    let attributes: ActionAttributesStruct = extract_attributes(&mut action_derive_input)?;

    Ok(attributes)
}
