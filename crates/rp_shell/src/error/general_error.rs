use crate::shell::ShellMode;
use rp_error::ErrorTrait;

#[derive(Debug)]
pub enum GeneralError {
    InvalidModeForCommand {
        command: String,
        mode: ShellMode,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
}

impl ErrorTrait for GeneralError {
    fn display(&self) -> String {
        match self {
            GeneralError::InvalidModeForCommand { command, mode, .. } => {
                format!("Invalid shell mode {:?} for command '{}'", mode, command)
            }
        }
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        match self {
            GeneralError::InvalidModeForCommand { source, .. } => source.as_deref(),
        }
    }
}
