use syn::{Ident, ItemStruct};
use crate::common::extract_attrs_by_name;

pub fn parse(action_struct: &ItemStruct) -> Option<Vec<(Ident, Ident)>> {
    extract_attrs_by_name("params", action_struct)
}
