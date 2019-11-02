use super::{helpers::*, ArgumentType, ArgumentWrapper, CommandMacroArgs};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Ident, ItemStruct, Type};

pub fn generate_command_metadata(
    ident: Ident,
    args: CommandMacroArgs,
) -> (TokenStream, Vec<String>) {
    let name = ident.clone();
    let mut aliases: Vec<String> = vec![ident.to_string().to_ascii_lowercase()];
    aliases.extend(args.extra_aliases);
    let mode_tokens = shellmode_to_tokens(args.required_shell_mode);

    (
        quote!(
            impl CommandMetadata for #name {
                fn aliases(&self) -> Vec<&str> {
                    let mut aliases = Vec::new();
                    #(aliases.push(#aliases);)*
                    aliases
                }

                fn required_shell_mode(&self) -> Option<ShellMode> {
                    #mode_tokens
                }
            }
        ),
        aliases,
    )
}

pub fn generate_command_from_args(item: &ItemStruct) -> TokenStream {
    let mut initialisers = Vec::new();

    for field in item.fields.iter() {
        let ident_str = field.ident.clone().unwrap().to_string();

        let getter = if let Type::Path(type_path) = &field.ty {
            let argument = syn::parse::<ArgumentWrapper>(type_path.path.to_token_stream().into())
                .expect("failed to parse argument wrapper");

            match argument {
                ArgumentWrapper::Vec(_t) => quote!(args),
                ArgumentWrapper::Option(_t) => quote!(if args.len() > 0 {
                    Some(args.remove(0))
                } else {
                    None
                }
                .map(|v| v.to_string())),
                ArgumentWrapper::None(ArgumentType::String) => quote!(if args.len() > 0 { Some(args
                        .remove(0)) } else { None }
                        .ok_or_else(|| {
                            anyhow::Error::from(rp_common::error::CommandError::MissingArgument(String::from(#ident_str)))
                        })?),
                ArgumentWrapper::None(ArgumentType::Integer) => {
                    quote!(if args.len() > 0 { Some(args
                        .remove(0)) } else { None }.map(|v| v.parse::<i64>)?)
                }
                ArgumentWrapper::None(ArgumentType::Float) => quote!(if args.len() > 0 { Some(args
                        .remove(0)) } else { None }.map(|v| v.parse::<f64>)?),
                ArgumentWrapper::None(ArgumentType::Boolean) => {
                    quote!(if args.len() > 0 { Some(args
                        .remove(0)) } else { None }.map(|v| v.parse::<bool>)?)
                }
            }
        } else {
            panic!();
        };

        let ident = field.ident.clone();
        initialisers.push(quote!(
            #ident: #getter
        ));
    }

    let name = item.ident.clone();
    let name_str = name.to_string();
    quote!(
        impl CommandFromArgs for #name {
            fn from_args(mut args: Vec<String>) -> anyhow::Result<Self> {
                rp_log::debug!("Command: {}{:?}", #name_str, args);
                // TODO: check for proper amount of arguments
                Ok(Self {
                    #(#initialisers),*
                })
            }
        }
    )
}
