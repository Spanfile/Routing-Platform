use super::{CommandError, ExecutableCommand, Shell};
use crate::{error, error::ErrorTrait, ConfigEditor};

#[derive(Debug)]
pub struct Configure;

impl ExecutableCommand for Configure {
    fn run(&self, shell: &mut Shell, _config_editor: &mut ConfigEditor) -> error::CustomResult<()> {
        if let Err(e) = shell.enter_mode() {
            Err(CommandError::RunError {
                command: String::from("configure"),
                description: e.display(),
            }
            .into())
        } else {
            Ok(())
        }
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["configure"]
    }
}
