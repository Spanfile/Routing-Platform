use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_core::common::{CommandFromArgs, CommandMetadata, ShellMode};
use rp_log::*;

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Save;

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Load {
    name: Option<String>,
}

impl ExecutableCommand for Save {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        if !editor.is_clean() {
            warn!("There are unapplied changes. Apply them first with `apply` or apply and save them with `apply save`.");
            Ok(())
        } else {
            info!(
                "Saving configuration to {}",
                editor.get_save_path().display()
            );
            editor.save()
        }
    }
}

impl ExecutableCommand for Load {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        if !editor.is_clean() {
            warn!("There are unapplied changes. Discard them with `discard` before loading a saved configuration.");
            Ok(())
        } else if let Some(name) = &self.name {
            info!("Loading configuration from {}", name);
            editor.load_from(name)
        } else {
            info!(
                "Loading configuration from default location {}",
                editor.get_save_path().display()
            );
            editor.load()
        }
    }
}
