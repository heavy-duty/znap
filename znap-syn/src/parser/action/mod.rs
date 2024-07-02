use crate::ActionStruct;
use syn::{parse::Result as ParseResult, ItemStruct};
mod attributes;
mod query_attrs;

pub fn parse(action_struct: &ItemStruct) -> ParseResult<ActionStruct> {
    let action_attributes = attributes::parse(&action_struct)?;
    let query_attrs = query_attrs::parse(&action_struct);

    Ok(ActionStruct {
        name: action_struct.ident.clone(),
        raw_struct: action_struct.clone(),
        attributes: action_attributes,
        query_attrs
    })
}
