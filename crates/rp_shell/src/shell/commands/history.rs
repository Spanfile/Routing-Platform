use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};
use strum::{EnumString, EnumVariantNames, VariantNames};

#[command]
#[derive(Debug)]
pub struct History {
    clear: Option<String>,
}

#[strum(serialize_all = "lowercase")]
#[derive(Debug, EnumString, EnumVariantNames)]
enum HistoryArgs {
    Clear,
}

impl ExecutableCommand for History {
    fn run(&self, shell: &mut Shell, _editor: &mut ConfigEditor) -> anyhow::Result<()> {
        match &self.clear {
            Some(s) => {
                match s.parse()? {
                    HistoryArgs::Clear => shell.clear_history(),
                }
                Ok(())
            }
            None => {
                shell.print_history()?;
                Ok(())
            }
        }
    }
}
