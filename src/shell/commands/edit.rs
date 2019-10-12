use super::{ExecutableCommand, Shell, ShellMode};
use crate::{error, ConfigEditor};

#[derive(Debug)]
pub struct Edit;

impl ExecutableCommand for Edit {
    fn run(&self, _shell: &mut Shell, config_editor: &mut ConfigEditor) -> error::CustomResult<()> {
        Ok(())
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["edit"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        Some(ShellMode::Configuration)
    }
}
