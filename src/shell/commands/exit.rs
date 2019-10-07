use super::{CommandError, ExecutableCommand, Shell};
use crate::ConfigEditor;

#[derive(Debug)]
pub struct Exit;

impl ExecutableCommand for Exit {
    fn run(
        &self,
        shell: &mut Shell,
        _config_editor: &mut ConfigEditor,
    ) -> Result<(), CommandError> {
        shell.exit_mode();
        Ok(())
    }
}
