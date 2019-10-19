extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use rp_common::ShellMode;
use syn::{parse_macro_input, AttributeArgs, ItemEnum, ItemStruct};

#[derive(Debug, FromMeta)]
struct CommandMacroArgs {
    #[darling(multiple, rename = "alias")]
    extra_aliases: Vec<String>,
    #[darling(map = "str_to_shellmode", default)]
    required_shell_mode: Option<ShellMode>,
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

    quote!(
        #item
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
    )
    .into()
}

#[proc_macro_derive(Command)]
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
