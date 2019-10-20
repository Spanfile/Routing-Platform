extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rp_common::ShellMode;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, AttributeArgs, Ident, ItemEnum, ItemStruct, Result, Token, Type,
};

#[derive(Debug, FromMeta)]
struct CommandMacroArgs {
    #[darling(multiple, rename = "alias")]
    extra_aliases: Vec<String>,
    #[darling(map = "str_to_shellmode", default)]
    required_shell_mode: Option<ShellMode>,
}

#[derive(Debug)]
enum ArgumentType {
    String,
    Integer,
    Float,
    Boolean,
}

#[derive(Debug)]
enum ArgumentWrapper {
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

#[proc_macro_attribute]
pub fn command(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemStruct);

    let args = match CommandMacroArgs::from_list(&attr) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let name = item.ident.clone();
    let mut aliases: Vec<String> = vec![item.ident.to_string().to_ascii_lowercase()];
    aliases.extend(args.extra_aliases);
    let mode_tokens = shellmode_to_tokens(args.required_shell_mode);

    let mut initialisers = Vec::new();
    let mut arg_index = 0;

    for field in item.fields.iter() {
        let ident = field.ident.clone();

        let getter = if let Type::Path(type_path) = &field.ty {
            let argument = syn::parse::<ArgumentWrapper>(type_path.path.to_token_stream().into())
                .expect("failed to parse argument wrapper");

            let g = match argument {
                ArgumentWrapper::Vec(_t) => quote!(args),
                ArgumentWrapper::Option(_t) => quote!(args.get(#arg_index)),
                ArgumentWrapper::None(ArgumentType::String) => quote!(args.get(#arg_index)?),
                ArgumentWrapper::None(ArgumentType::Integer) => {
                    quote!(args.get(#arg_index).map(|v| v.parse::<i64>)?)
                }
                ArgumentWrapper::None(ArgumentType::Float) => {
                    quote!(args.get(#arg_index).map(|v| v.parse::<f64>)?)
                }
                ArgumentWrapper::None(ArgumentType::Boolean) => {
                    quote!(args.get(#arg_index).map(|v| v.parse::<bool>)?)
                }
            };

            arg_index += 1;
            g
        } else {
            panic!();
        };

        initialisers.push(quote!(
            #ident: #getter
        ));
    }

    quote!(
        #item
        impl CommandMetadata for #name {
            fn from_args(args: Vec<String>) -> anyhow::Result<Self> {
                Ok(Self {
                    #(#initialisers),*
                })
            }

            fn aliases(&self) -> Vec<&str> {
                let mut aliases = Vec::new();
                #(aliases.push(#aliases);)*
                aliases
            }

            fn required_shell_mode(&self) -> Option<ShellMode> {
                #mode_tokens
            }
        }
    )
    .into()
}

#[proc_macro_derive(CommandEnum)]
pub fn command_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemEnum);
    let name = item.ident.clone();

    let mut alias_arms = Vec::new();
    let mut shell_mode_arms = Vec::new();

    for variant in item.variants.iter() {
        let ident = variant.ident.clone();
        alias_arms.push(quote!(
            #name::#ident(cmd) => cmd.aliases(),
        ));

        let ident = variant.ident.clone();
        shell_mode_arms.push(quote!(
            #name::#ident(cmd) => cmd.required_shell_mode(),
        ));
    }

    quote!(
        impl CommandMetadata for #name {
            fn aliases(&self) -> Vec<&str> {
                match self {
                    #(#alias_arms)*
                }
            }

            fn required_shell_mode(&self) -> Option<ShellMode> {
                match self {
                    #(#shell_mode_arms)*
                }
            }
        }
    )
    .into()
}

fn str_to_shellmode(s: String) -> Option<ShellMode> {
    Some(s.parse().unwrap())
}

fn shellmode_to_tokens(mode: Option<ShellMode>) -> TokenStream {
    if let Some(mode) = mode {
        let begin = quote!(ShellMode::);
        let name = match mode {
            ShellMode::Operational => quote!(Operational),
            ShellMode::Configuration => quote!(Configuration),
        };
        quote!(
            Some(#begin#name)
        )
    } else {
        quote!(None)
    }
}
