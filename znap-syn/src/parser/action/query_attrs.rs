use syn::{Ident, ItemStruct};
use crate::common::extract_query_attrs;

pub fn parse(action_struct: &ItemStruct) -> Option<Vec<(Ident, Ident)>> {
    extract_query_attrs(action_struct)
}
