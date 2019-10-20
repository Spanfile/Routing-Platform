use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Show;

impl ExecutableCommand for Show {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        editor.pretty_print_current_node();
        Ok(())
    }
}
