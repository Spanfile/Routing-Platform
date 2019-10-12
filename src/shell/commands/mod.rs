mod command_error;
mod configure;
mod exit;
mod show;

use super::{super::ConfigEditor, Shell, ShellMode};
use crate::error;
pub use command_error::CommandError;
use configure::Configure;
use enum_dispatch::enum_dispatch;
use exit::Exit;
use show::Show;
use std::str::FromStr;

#[enum_dispatch]
pub trait ExecutableCommand {
    fn run(&self, shell: &mut Shell, config_editor: &mut ConfigEditor) -> error::CustomResult<()>;
    fn aliases(&self) -> Vec<&str>;
    fn required_shell_mode(&self) -> Option<ShellMode>;
}

#[enum_dispatch(ExecutableCommand)]
#[derive(Debug)]
pub enum Command {
    Exit,
    Configure,
    Show,
}

impl FromStr for Command {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" | "quit" => Ok(Exit {}.into()),
            "configure" => Ok(Configure {}.into()),
            "show" => Ok(Show {}.into()),
            _ => Err(CommandError::NotFound {
                command: s.to_string(),
                source: None,
            }
            .into()),
        }
    }
}
