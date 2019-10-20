use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandMetadata, ShellMode};

#[command(alias = "quit")]
#[derive(Debug)]
pub struct Exit;

impl ExecutableCommand for Exit {
    fn run(&self, shell: &mut Shell, _editor: &mut ConfigEditor) -> anyhow::Result<()> {
        shell.exit_mode();
        Ok(())
    }
}
