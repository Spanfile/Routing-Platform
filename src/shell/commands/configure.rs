use super::{CommandError, ExecutableCommand, Shell, ShellMode};
use crate::{error, ConfigEditor};

#[derive(Debug)]
pub struct Configure;

impl ExecutableCommand for Configure {
    fn run(
        &self,
        _arguments: Vec<String>,
        shell: &mut Shell,
        _editor: &mut ConfigEditor,
    ) -> error::CustomResult<()> {
        shell.enter_mode().map_err(|e| {
            error::Error::from(CommandError::RunError {
                command: String::from("configure"),
                source: Some(Box::new(e)),
            })
        })
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["configure"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        Some(ShellMode::Operational)
    }
}
