use super::{CommandError, ExecutableCommand, Shell, ShellMode};
use crate::ConfigEditor;

#[derive(Debug)]
pub struct Configure;

impl ExecutableCommand for Configure {
    fn run(
        &self,
        shell: &mut Shell,
        _config_editor: &mut ConfigEditor,
    ) -> Result<(), CommandError> {
        if let ShellMode::Operational = shell.mode {
            shell.enter_mode();
            Ok(())
        } else {
            Err(CommandError {
                message: String::from("shell already in configuration state"),
            })
        }
    }
}
