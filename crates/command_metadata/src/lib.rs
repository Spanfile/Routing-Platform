extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use rp_common::ShellMode;
use syn::{parse_macro_input, AttributeArgs, ItemStruct};

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
    let mut aliases: Vec<String> = vec![item.ident.to_string().to_ascii_lowercase().to_owned()];
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

fn str_to_shellmode(s: String) -> Option<ShellMode> {
    None
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
