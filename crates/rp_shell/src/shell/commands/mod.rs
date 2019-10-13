mod configure;
mod edit;
mod exit;
mod show;

use super::{super::ConfigEditor, Shell, ShellMode};
use crate::error;
use configure::Configure;
use edit::{Edit, Remove, Set, Top, Up};
use enum_dispatch::enum_dispatch;
use exit::Exit;
use show::Show;
use std::str::FromStr;

#[enum_dispatch]
pub trait ExecutableCommand {
    fn run(
        &self,
        arguments: Vec<String>,
        shell: &mut Shell,
        config_editor: &mut ConfigEditor,
    ) -> error::Result<()>;
    fn aliases(&self) -> Vec<&str>;
    fn required_shell_mode(&self) -> Option<ShellMode>;
}

#[enum_dispatch(ExecutableCommand)]
#[derive(Debug)]
pub enum Command {
    Exit,
    Configure,
    Show,
    Edit,
    Up,
    Top,
    Set,
    Remove,
}

impl FromStr for Command {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" | "quit" => Ok(Exit {}.into()),
            "configure" => Ok(Configure {}.into()),
            "show" => Ok(Show {}.into()),
            "edit" => Ok(Edit {}.into()),
            "up" => Ok(Up {}.into()),
            "top" => Ok(Top {}.into()),
            "set" => Ok(Set {}.into()),
            "remove" => Ok(Remove {}.into()),
            _ => Err(error::CommandError::NotFound {
                command: s.to_string(),
                source: None,
            }
            .into()),
        }
    }
}
