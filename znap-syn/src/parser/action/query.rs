use syn::{Ident, ItemStruct};
use crate::common::extract_query_type;

pub fn parse(action_struct: &ItemStruct) -> Option<Ident> {
    let query = extract_query_type(action_struct);

    query
}
