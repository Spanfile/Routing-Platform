use syn::{
    parse::{Parse, ParseStream},
    Ident, Result, Token,
};

#[derive(Debug)]
pub enum ArgumentWrapper {
    Vec(String),
    Option(String),
    None(String),
}

impl Parse for ArgumentWrapper {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![<]) {
            let _open_br = input.parse::<Token![<]>()?;
            let argument_type = input.parse::<Ident>()?;
            let _close_br = input.parse::<Token![>]>()?;

            match ident.to_string().as_str() {
                "Vec" => Ok(ArgumentWrapper::Vec(argument_type.to_string())),
                "Option" => Ok(ArgumentWrapper::Option(argument_type.to_string())),
                _ => Err(input.error("unknown argument wrapper type")),
            }
        } else {
            Ok(ArgumentWrapper::None(ident.to_string()))
        }
    }
}
