use crate::{common::has_action, ActionAttributesStruct};
use deluxe::extract_attributes;
use quote::ToTokens;
use syn::{parse::Result as ParseResult, parse2, ItemStruct};

pub fn parse(action_struct: &ItemStruct) -> ParseResult<Option<ActionAttributesStruct>> {
    if has_action(action_struct) {
        let mut action_derive_input = parse2::<syn::DeriveInput>(action_struct.to_token_stream())?;

        let attributes: ActionAttributesStruct = extract_attributes(&mut action_derive_input)?;

        Ok(Some(attributes))
    } else {
        Ok(None)
    }
}
