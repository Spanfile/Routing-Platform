use super::{ExecutableCommand, Shell, ShellMode};
use crate::{error, ConfigEditor};

#[derive(Debug)]
pub struct History;

impl ExecutableCommand for History {
    fn run(
        &self,
        arguments: Vec<String>,
        shell: &mut Shell,
        _editor: &mut ConfigEditor,
    ) -> error::Result<()> {
        match arguments.get(0) {
            Some(s) => match s.as_str() {
                "clear" => {
                    shell.clear_history();
                    Ok(())
                }
                _ => Err(error::CommandError::UnexpectedArgument {
                    argument: s.to_owned(),
                    source: None,
                }
                .into()),
            },
            None => {
                shell.print_history()?;
                Ok(())
            }
        }
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["history"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        None
    }
}
