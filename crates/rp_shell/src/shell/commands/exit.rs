use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};
use strum::{EnumString, EnumVariantNames, VariantNames};

#[command(alias = "quit")]
#[derive(Debug)]
pub struct Exit {
    discard: Option<ExitDiscard>,
}

#[strum(serialize_all = "lowercase")]
#[derive(Debug, EnumString, EnumVariantNames)]
pub enum ExitDiscard {
    Discard,
}

impl ExecutableCommand for Exit {
    fn run(&self, shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        if shell.mode == ShellMode::Configuration && !editor.is_clean() {
            if let Some(ExitDiscard::Discard) = self.discard {
                editor.discard_changes();
                shell.exit_mode();
                Ok(())
            } else {
                Err(crate::error::GeneralError::UnappliedChanges.into())
            }
        } else {
            shell.exit_mode();
            Ok(())
        }
    }
}
