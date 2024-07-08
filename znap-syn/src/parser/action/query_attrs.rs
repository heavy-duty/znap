use crate::common::extract_attrs_by_name;
use syn::{Ident, ItemStruct};

pub fn parse(action_struct: &ItemStruct) -> Option<Vec<(Ident, Ident)>> {
    extract_attrs_by_name("query", action_struct)
}
