mod configure;
mod edit;
mod exit;
mod history;
mod show;

use super::{super::ConfigEditor, Shell, ShellMode};
use crate::error;
use configure::Configure;
use edit::{Edit, Remove, Set, Top, Up};
use enum_dispatch::enum_dispatch;
use exit::Exit;
use history::History;
use rp_common::CommandMetadata;
use show::Show;
use std::str::FromStr;

#[enum_dispatch]
pub trait ExecutableCommand: CommandMetadata {
    fn run(
        &self,
        arguments: Vec<String>,
        shell: &mut Shell,
        config_editor: &mut ConfigEditor,
    ) -> anyhow::Result<()>;
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
    History,
}

impl CommandMetadata for Command {
    fn aliases(&self) -> Vec<&str> {
        match self {
            Command::Exit(cmd) => cmd.aliases(),
            Command::Configure(cmd) => cmd.aliases(),
            Command::Show(cmd) => cmd.aliases(),
            Command::Edit(cmd) => cmd.aliases(),
            Command::Up(cmd) => cmd.aliases(),
            Command::Top(cmd) => cmd.aliases(),
            Command::Set(cmd) => cmd.aliases(),
            Command::Remove(cmd) => cmd.aliases(),
            Command::History(cmd) => cmd.aliases(),
        }
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        match self {
            Command::Exit(cmd) => cmd.required_shell_mode(),
            Command::Configure(cmd) => cmd.required_shell_mode(),
            Command::Show(cmd) => cmd.required_shell_mode(),
            Command::Edit(cmd) => cmd.required_shell_mode(),
            Command::Up(cmd) => cmd.required_shell_mode(),
            Command::Top(cmd) => cmd.required_shell_mode(),
            Command::Set(cmd) => cmd.required_shell_mode(),
            Command::Remove(cmd) => cmd.required_shell_mode(),
            Command::History(cmd) => cmd.required_shell_mode(),
        }
    }
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        match s {
            "exit" | "quit" => Ok(Exit {}.into()),
            "configure" => Ok(Configure {}.into()),
            "show" => Ok(Show {}.into()),
            "edit" => Ok(Edit {}.into()),
            "up" => Ok(Up {}.into()),
            "top" => Ok(Top {}.into()),
            "set" => Ok(Set {}.into()),
            "remove" => Ok(Remove {}.into()),
            "history" => Ok(History {}.into()),
            _ => Err(error::CommandError::NotFound(s.to_string()).into()),
        }
    }
}
