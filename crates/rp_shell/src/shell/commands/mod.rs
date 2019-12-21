mod apply;
mod configure;
mod edit;
mod exit;
mod history;
mod save_load;
mod show;

use super::{super::ConfigEditor, Shell, ShellMode};
use apply::{Apply, Discard};
use command_metadata::CommandEnum;
use configure::Configure;
use edit::{Edit, Remove, Set, Top, Up};
use enum_dispatch::enum_dispatch;
use exit::Exit;
use history::History;
use rp_core::common::{CommandFromArgs, CommandMetadata};
use save_load::{Load, Save};
use show::Show;

#[enum_dispatch]
pub trait ExecutableCommand: CommandMetadata {
    fn run(&self, shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()>;
}

#[enum_dispatch(ExecutableCommand)]
#[derive(Debug, CommandEnum)]
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
    Apply,
    Discard,
    Save,
    Load,
}
