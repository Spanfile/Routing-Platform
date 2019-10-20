use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Result, Token,
};

#[derive(Debug)]
pub enum ArgumentType {
    String,
    Integer,
    Float,
    Boolean,
}

#[derive(Debug)]
pub enum ArgumentWrapper {
    Vec(ArgumentType),
    Option(ArgumentType),
    None(ArgumentType),
}

impl Parse for ArgumentWrapper {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![<]) {
            match ident.to_string().as_str() {
                "Vec" => Ok(ArgumentWrapper::Vec(input.parse::<ArgumentType>()?)),
                "Option" => Ok(ArgumentWrapper::Option(input.parse::<ArgumentType>()?)),
                _ => Err(input.error("unknown argument wrapper type")),
            }
        } else {
            Ok(ArgumentWrapper::None(syn::parse::<ArgumentType>(
                ident.to_token_stream().into(),
            )?))
        }
    }
}

impl Parse for ArgumentType {
    fn parse(input: ParseStream) -> Result<Self> {
        let _ = input.parse::<Token![<]>();
        let ident = input.parse::<Ident>()?;
        let _ = input.parse::<Token![>]>();
        match ident.to_string().as_str() {
            "String" => Ok(ArgumentType::String),
            "i64" => Ok(ArgumentType::Integer),
            "f64" => Ok(ArgumentType::Float),
            "bool" => Ok(ArgumentType::Boolean),
            _ => Err(input.error("unknown argument type")),
        }
    }
}
