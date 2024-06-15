use crate::ErrorEnum;
use syn::{parse::Result as ParseResult, ItemEnum};

pub fn parse(error_enum: &ItemEnum) -> ParseResult<ErrorEnum> {
    Ok(ErrorEnum {
        name: error_enum.ident.clone(),
        raw_enum: error_enum.clone(),
    })
}
