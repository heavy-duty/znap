use crate::QueryStruct;
use syn::{parse::Result as ParseResult, ItemStruct};

pub fn parse(query_struct: &ItemStruct) -> ParseResult<QueryStruct> {
    Ok(QueryStruct {
        name: query_struct.ident.clone(),
        raw_struct: query_struct.clone(),
    })
}
