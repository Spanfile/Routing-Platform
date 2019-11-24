use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};
use rp_log::*;

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Save;

impl ExecutableCommand for Save {
    fn run(&self, _shell: &mut Shell, config_editor: &mut ConfigEditor) -> anyhow::Result<()> {
        info!("Saving configuration...");
        config_editor.save()
    }
}
