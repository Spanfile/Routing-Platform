use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_core::common::{CommandFromArgs, CommandMetadata, ShellMode};
use rp_log::*;
use strum::{EnumString, EnumVariantNames};

#[command(alias = "quit")]
#[derive(Debug)]
pub struct Exit {
    exit_arg: Option<ExitArgs>,
}

#[strum(serialize_all = "lowercase")]
#[derive(Debug, EnumString, EnumVariantNames)]
pub enum ExitArgs {
    Discard,
    Save,
}

impl ExecutableCommand for Exit {
    fn run(&self, shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        if shell.mode == ShellMode::Configuration && !editor.is_clean() {
            match self.exit_arg {
                Some(ExitArgs::Discard) => {
                    editor.discard_changes();
                    shell.exit_mode();
                }
                Some(ExitArgs::Save) => {
                    editor.apply_changes()?;
                    editor.save()?;
                    shell.exit_mode();
                }
                _ => {
                    warn!(
                        r#"There are unapplied changes. You can:
 * Apply them with `apply`. Afterwards save them with `save`
 * Apply and save them with `apply save`
 * Apply them, save them and exit with `exit save`
 * Discard them with `discard`
 * Discard them and exit with `exit discard`"#
                    );
                }
            }
        } else {
            shell.exit_mode();
        }

        Ok(())
    }
}
