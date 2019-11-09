use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};

#[command]
#[derive(Debug)]
pub struct Apply;

#[command]
#[derive(Debug)]
pub struct Discard;

impl ExecutableCommand for Apply {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        if editor.is_clean() {
            println!("Configuration clean - no changes to apply");
            Ok(())
        } else {
            editor.apply_changes()
        }
    }
}

impl ExecutableCommand for Discard {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        if editor.is_clean() {
            println!("Configuration clean - no changes to discard");
        } else {
            editor.discard_changes();
        }

        Ok(())
    }
}
