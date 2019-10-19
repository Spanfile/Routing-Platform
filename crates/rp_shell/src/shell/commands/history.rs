use super::{ExecutableCommand, Shell};
use crate::{error, ConfigEditor};
use command_metadata::command;
use rp_common::{CommandMetadata, ShellMode};

#[command]
#[derive(Debug)]
pub struct History;

impl ExecutableCommand for History {
    fn run(
        &self,
        arguments: Vec<String>,
        shell: &mut Shell,
        _editor: &mut ConfigEditor,
    ) -> anyhow::Result<()> {
        match arguments.get(0) {
            Some(s) => match s.as_str() {
                "clear" => {
                    shell.clear_history();
                    Ok(())
                }
                _ => Err(error::CommandError::UnexpectedArgument(s.to_owned()).into()),
            },
            None => {
                shell.print_history()?;
                Ok(())
            }
        }
    }
}
