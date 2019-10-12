use super::{ExecutableCommand, Shell, ShellMode};
use crate::{error, ConfigEditor};

#[derive(Debug)]
pub struct Edit;

#[derive(Debug)]
pub struct Up;

impl ExecutableCommand for Edit {
    fn run(
        &self,
        arguments: Vec<String>,
        _shell: &mut Shell,
        editor: &mut ConfigEditor,
    ) -> error::CustomResult<()> {
        for arg in arguments {
            editor.edit_node(arg)?;
        }

        Ok(())
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["edit"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        Some(ShellMode::Configuration)
    }
}

impl ExecutableCommand for Up {
    fn run(
        &self,
        _arguments: Vec<String>,
        _shell: &mut Shell,
        editor: &mut ConfigEditor,
    ) -> error::CustomResult<()> {
        editor.go_up()
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["up"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        Some(ShellMode::Configuration)
    }
}
