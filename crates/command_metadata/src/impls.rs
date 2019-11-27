use super::{helpers::*, ArgumentWrapper, CommandMacroArgs};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Ident, ItemStruct};

pub fn generate_command_metadata(
    ident: &Ident,
    args: CommandMacroArgs,
) -> anyhow::Result<(TokenStream, Vec<String>)> {
    let mut aliases: Vec<String> = vec![ident.to_string().to_ascii_lowercase()];
    aliases.extend(args.extra_aliases);
    let mode_tokens = shellmode_to_tokens(args.required_shell_mode);

    Ok((
        quote!(
            impl CommandMetadata for #ident {
                fn aliases(&self) -> Vec<&'static str> {
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
    ))
}

pub fn generate_command_from_args(item: &ItemStruct) -> anyhow::Result<TokenStream> {
    let mut initialisers = Vec::new();

    for field in item.fields.iter() {
        let ident = field.ident.as_ref().unwrap();
        let field_name = ident.to_string();
        let argument = syn::parse::<ArgumentWrapper>(field.ty.to_token_stream().into())?;

        let getter = match argument {
            ArgumentWrapper::Vec(argument_type) => {
                let argument_ident = create_ident(&argument_type)?;
                let mutator = if argument_type == "String" {
                    quote!()
                } else {
                    quote!(.iter().map(|v| v.parse::<#argument_ident>()).collect::<anyhow::Result<Vec<#argument_ident>>>()?)
                };

                quote!({
                    args#mutator
                })
            }
            ArgumentWrapper::Option(argument_type) => {
                let argument_ident = create_ident(&argument_type)?;
                let mutator = if argument_type == "String" {
                    quote!()
                } else {
                    quote!(.parse::<#argument_ident>()?)
                };

                quote!(if args.len() > 0 {
                    Some(args.remove(0)#mutator)
                } else {
                    None
                })
            }
            ArgumentWrapper::None(argument_type) => {
                let argument_ident = create_ident(&argument_type)?;
                let mutator = if argument_type == "String" {
                    quote!()
                } else {
                    quote!(.parse::<#argument_ident>()?)
                };

                quote!(if args.len() > 0 { Some(args.remove(0)#mutator) } else { None }
                    .ok_or_else(|| { rp_common::error::CommandError::missing_argument(#field_name, ExpectedValue::Literal(#argument_type)) })?)
            }
        };

        initialisers.push(quote!(
            #ident: #getter
        ));
    }

    let ident = &item.ident;
    let ident_str = ident.to_string();
    Ok(quote!(
        impl CommandFromArgs for #ident {
            fn from_args(mut args: Vec<String>) -> anyhow::Result<Self> {
                rp_log::debug!("Command: {}{:?}", #ident_str, args);
                // TODO: check for proper amount of arguments
                Ok(Self {
                    #(#initialisers),*
                })
            }
        }
    ))
}

fn create_ident(ident_str: &str) -> anyhow::Result<Ident> {
    Ok(syn::parse_str::<Ident>(ident_str)?)
}
