use crate::ErrorEnum;
use syn::{parse::Result as ParseResult, ItemEnum};
mod error;

pub fn parse(error_enum: &ItemEnum) -> ParseResult<ErrorEnum> {
    let error_variants = error::parse(error_enum)?;

    Ok(ErrorEnum {
        name: error_enum.ident.clone(),
        raw_enum: error_enum.clone(),
        error_variants,
    })
}
