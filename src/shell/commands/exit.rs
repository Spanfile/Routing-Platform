use super::{ExecutableCommand, Shell, ShellMode};
use crate::{error, ConfigEditor};

#[derive(Debug)]
pub struct Exit;

impl ExecutableCommand for Exit {
    fn run(
        &self,
        _arguments: Vec<String>,
        shell: &mut Shell,
        _editor: &mut ConfigEditor,
    ) -> error::CustomResult<()> {
        shell.exit_mode();
        Ok(())
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["configure"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        None
    }
}
