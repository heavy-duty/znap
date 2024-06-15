pub mod codegen;
pub mod parser;
use codegen::query as query_codegen;
use codegen::action as action_codegen;
use codegen::collection as collection_codegen;
use deluxe::ExtractAttributes;
use parser::query as query_parser;
use parser::action as action_parser;
use parser::collection as collection_parser;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    Ident, ItemFn, ItemMod, ItemStruct,
};

#[derive(Debug)]
pub struct CollectionMod {
    pub action_fns: Vec<ActionFn>,
    pub name: Ident,
    pub collection_mod: ItemMod,
}

impl Parse for CollectionMod {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let collection_mod = <ItemMod as Parse>::parse(input)?;
        collection_parser::parse(&collection_mod)
    }
}

impl From<&CollectionMod> for TokenStream {
    fn from(collection_mod: &CollectionMod) -> Self {
        collection_codegen::generate(collection_mod)
    }
}

impl ToTokens for CollectionMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

#[derive(Debug)]
pub struct ActionFn {
    pub raw_method: ItemFn,
    pub name: Ident,
    pub handle_get_ident: Ident,
    pub handle_post_ident: Ident,
    pub action_ident: Ident,
    pub action_query_ident: Option<Ident>,
    pub route_path: String,
}

#[derive(Debug)]
pub struct ActionStruct {
    pub name: Ident,
    pub raw_struct: ItemStruct,
    pub attributes: ActionAttributesStruct,
}

impl Parse for ActionStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let action_struct = <ItemStruct as Parse>::parse(input)?;
        action_parser::parse(&action_struct)
    }
}

impl From<&ActionStruct> for TokenStream {
    fn from(action_struct: &ActionStruct) -> Self {
        action_codegen::generate(action_struct)
    }
}

impl ToTokens for ActionStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

#[derive(Debug, ExtractAttributes)]
#[deluxe(attributes(action))]
pub struct ActionAttributesStruct {
    pub icon: String,
    pub title: String,
    pub description: String,
    pub label: String,
}

#[derive(Debug)]
pub struct QueryStruct {
    pub name: Ident,
    pub raw_struct: ItemStruct,
}

impl Parse for QueryStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let query_struct = <ItemStruct as Parse>::parse(input)?;
        query_parser::parse(&query_struct)
    }
}

impl From<&QueryStruct> for TokenStream {
    fn from(query_struct: &QueryStruct) -> Self {
        query_codegen::generate(query_struct)
    }
}

impl ToTokens for QueryStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}