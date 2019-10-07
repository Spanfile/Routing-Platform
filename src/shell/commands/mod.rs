mod configure;
mod exit;

use super::{super::ConfigEditor, Shell, ShellMode};
use configure::Configure;
use enum_dispatch::enum_dispatch;
use exit::Exit;
use std::str::FromStr;

#[enum_dispatch]
pub trait ExecutableCommand {
    fn run(&self, shell: &mut Shell, config_editor: &mut ConfigEditor) -> Result<(), CommandError>;
}

#[enum_dispatch(ExecutableCommand)]
#[derive(Debug)]
pub enum Command {
    Exit,
    Configure,
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!();
    }
}

#[derive(Debug, Clone)]
pub struct CommandError {
    message: String,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl std::error::Error for CommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
