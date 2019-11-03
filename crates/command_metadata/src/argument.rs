use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    GenericArgument, PathArguments, Result, Type,
};

#[derive(Debug)]
pub enum ArgumentWrapper {
    Vec(String),
    Option(String),
    None(String),
}

impl Parse for ArgumentWrapper {
    fn parse(input: ParseStream) -> Result<Self> {
        let typ = input.parse::<Type>()?;

        match typ {
            Type::Path(path) => {
                if let Some(last) = path.path.segments.last() {
                    if let PathArguments::AngleBracketed(generic_args) = &last.arguments {
                        if generic_args.args.len() != 1 {
                            Err(input.error("expected exactly one generic argument"))
                        } else {
                            if let Some(GenericArgument::Type(generic_type)) =
                                generic_args.args.first()
                            {
                                let type_string = generic_type.to_token_stream().to_string();
                                match last.ident.to_string().as_ref() {
                                    "Vec" => Ok(ArgumentWrapper::Vec(type_string)),
                                    "Option" => Ok(ArgumentWrapper::Option(type_string)),
                                    _ => Err(input.error("invalid wrapper type")),
                                }
                            } else {
                                Err(input.error("invalid generic parameter type"))
                            }
                        }
                    } else {
                        Ok(ArgumentWrapper::None(last.ident.to_string()))
                    }
                } else {
                    Err(input.error("empty path"))
                }
            }
            _ => Err(input.error("unknown pattern")),
        }
    }
}
