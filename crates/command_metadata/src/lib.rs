mod argument;
mod helpers;
mod impls;
mod macro_args;

extern crate proc_macro;

use argument::ArgumentWrapper;
use darling::FromMeta;
use impls::*;
use lazy_static::lazy_static;
use macro_args::CommandMacroArgs;
use quote::quote;
use std::{collections::HashMap, sync::Mutex};
use syn::{parse_macro_input, AttributeArgs, ItemEnum, ItemStruct};

lazy_static! {
    static ref COMMAND_ALIASES: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

#[proc_macro_attribute]
pub fn command(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemStruct);
    let ident = &item.ident;

    let args = match CommandMacroArgs::from_list(&attr) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let command_from_args_impl =
        generate_command_from_args(&item).expect("failed to create CommandFromArgs impl");
    let (command_metadata_impl, aliases) =
        generate_command_metadata(ident, args).expect("failed to create CommandMetadata impl");

    COMMAND_ALIASES
        .lock()
        .unwrap()
        .insert(ident.to_string(), aliases);

    quote!(
        #item
        #command_from_args_impl
        #command_metadata_impl
    )
    .into()
}

#[proc_macro_derive(CommandEnum)]
pub fn command_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemEnum);
    let name = &item.ident;

    let mut alias_arms = Vec::new();
    let mut shell_mode_arms = Vec::new();
    let mut command_creation_arms = Vec::new();
    let mut alias_names = Vec::new();

    for variant in item.variants.iter() {
        let ident = &variant.ident;
        alias_arms.push(quote!(
            #name::#ident(cmd) => cmd.aliases(),
        ));

        shell_mode_arms.push(quote!(
            #name::#ident(cmd) => cmd.required_shell_mode(),
        ));

        let aliases = COMMAND_ALIASES
            .lock()
            .unwrap()
            .get(&ident.to_string())
            .unwrap()
            .clone();
        command_creation_arms.push(quote!(
            #(#aliases)|* => Ok(#ident::from_args(args)?.into()),
        ));

        alias_names.push(quote!(
            #(#aliases), *
        ));
    }

    quote!(
        impl #name {
            pub fn new(command_name: &str, args: Vec<String>) -> anyhow::Result<Self> {
                match command_name {
                    #(#command_creation_arms)*
                    _ => Err(rp_core::error::CommandError::not_found(String::from(command_name))),
                }
            }

            pub fn all_aliases() -> Vec<&'static str> {
                vec![#(#alias_names), *]
            }
        }

        impl CommandMetadata for #name {
            fn aliases(&self) -> Vec<&'static str> {
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
