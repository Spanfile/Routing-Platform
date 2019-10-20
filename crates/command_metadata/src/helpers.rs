use proc_macro2::TokenStream;
use quote::quote;
use rp_common::ShellMode;

pub fn str_to_shellmode(s: String) -> Option<ShellMode> {
    Some(s.parse().unwrap())
}

pub fn shellmode_to_tokens(mode: Option<ShellMode>) -> TokenStream {
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
