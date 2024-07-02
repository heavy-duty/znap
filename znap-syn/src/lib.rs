pub mod codegen;
pub mod parser;
pub mod common;
use codegen::action as action_codegen;
use codegen::collection as collection_codegen;
use codegen::error_code as error_code_codegen;
use deluxe::ExtractAttributes;
use deluxe::ParseMetaItem;
use parser::action as action_parser;
use parser::collection as collection_parser;
use parser::error_code as error_code_parser;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ItemEnum;
use syn::Variant;
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    Ident, ItemFn, ItemMod, ItemStruct,
};

#[derive(Debug)]
pub struct CollectionMod {
    pub actions: Vec<Ident>,
    pub get_action_fns: Vec<GetActionFn>,
    pub post_action_fns: Vec<PostActionFn>,
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
pub struct GetActionFn {
    pub raw_method: ItemFn,
    pub name: Ident,
    pub action: Ident,
    pub query: Option<Ident>,
}

#[derive(Debug)]
pub struct PostActionFn {
    pub raw_method: ItemFn,
    pub name: Ident,
    pub action: Ident,
    pub query: Option<Ident>,
}

#[derive(Debug)]
pub struct ActionStruct {
    pub name: Ident,
    pub raw_struct: ItemStruct,
    pub attributes: ActionAttributesStruct,
    pub query_attrs: Option<Vec<(Ident, Ident)>>,
    pub params_attrs: Option<Vec<(Ident, Ident)>>,
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
    #[deluxe(append, rename = link, default = Vec::new())]
    pub links: Vec<ActionLinkStruct>,
}

#[derive(Debug, ParseMetaItem)]
pub struct ActionLinkStruct {
    label: String,
    href: String,
    #[deluxe(append, rename = parameter, default = Vec::new())]
    parameters: Vec<ActionLinkParameterStruct>,
}

#[derive(Debug, ParseMetaItem)]
pub struct ActionLinkParameterStruct {
    label: String,
    name: String,
    #[deluxe(default = false)]
    required: bool,
}

#[derive(Debug)]
pub struct ErrorEnum {
    pub name: Ident,
    pub raw_enum: ItemEnum,
    pub error_variants: Vec<ErrorVariant>,
}

impl Parse for ErrorEnum {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let error_enum = <ItemEnum as Parse>::parse(input)?;
        error_code_parser::parse(&error_enum)
    }
}

impl From<&ErrorEnum> for TokenStream {
    fn from(error_enum: &ErrorEnum) -> Self {
        error_code_codegen::generate(error_enum)
    }
}

impl ToTokens for ErrorEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

#[derive(Debug, ExtractAttributes)]
#[deluxe(attributes(error))]
pub struct ErrorAttributesStruct {
    pub msg: String,
}

#[derive(Debug)]
pub struct ErrorVariant {
    pub name: Ident,
    pub raw_variant: Variant,
    pub msg: String,
}

