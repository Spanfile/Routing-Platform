use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};
use rp_log::*;
use strum::{EnumString, EnumVariantNames};

#[command]
#[derive(Debug)]
pub struct Apply {
    save: Option<ApplyArgs>,
}

#[strum(serialize_all = "lowercase")]
#[derive(Debug, EnumString, EnumVariantNames)]
enum ApplyArgs {
    Save,
}

#[command]
#[derive(Debug)]
pub struct Discard;

impl ExecutableCommand for Apply {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        if editor.is_clean() {
            info!("Configuration clean - no changes to apply");
        } else {
            if editor.apply_changes()? {
                if let Some(ApplyArgs::Save) = self.save {
                    editor.save()?;
                    info!("Changes applied and saved");
                } else {
                    info!("Changes applied but not yet saved - save them with `save`");
                }
            } else {
                warn!("No changes were applied");
            }
        }

        Ok(())
    }
}

impl ExecutableCommand for Discard {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        if editor.is_clean() {
            info!("Configuration clean - no changes to discard");
        } else {
            editor.discard_changes();
        }

        Ok(())
    }
}
