use super::{ExecutableCommand, Shell, ShellMode};
use crate::{error, ConfigEditor};

#[derive(Debug)]
pub struct Show;

impl ExecutableCommand for Show {
    fn run(
        &self,
        _arguments: Vec<String>,
        _shell: &mut Shell,
        editor: &mut ConfigEditor,
    ) -> error::Result<()> {
        editor.pretty_print_current_node();
        Ok(())
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["show"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        Some(ShellMode::Configuration)
    }
}
