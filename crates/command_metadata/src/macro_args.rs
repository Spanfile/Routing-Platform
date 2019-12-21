use super::helpers::*;
use darling::FromMeta;
use rp_core::common::ShellMode;

#[derive(Debug, FromMeta)]
pub struct CommandMacroArgs {
    #[darling(multiple, rename = "alias")]
    pub extra_aliases: Vec<String>,
    #[darling(map = "str_to_shellmode", default)]
    pub required_shell_mode: Option<ShellMode>,
}
