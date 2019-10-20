use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};

#[command(required_shell_mode = "Operational")]
#[derive(Debug)]
pub struct Configure;

impl ExecutableCommand for Configure {
    fn run(&self, shell: &mut Shell, _editor: &mut ConfigEditor) -> anyhow::Result<()> {
        shell.enter_mode()
    }
}
