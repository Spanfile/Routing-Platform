use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};
use rp_log::*;

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Save;

impl ExecutableCommand for Save {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        if !editor.is_clean() {
            warn!("There are unapplied changes. Apply them first with `apply` or apply and save them with `apply save`");
            Ok(())
        } else {
            info!("Saving configuration to {}", editor.save_location);
            editor.save()
        }
    }
}
