use super::shell::ShellMode;
use crate::{error, error::ErrorTrait};

#[derive(Debug)]
pub enum GeneralError {
    InvalidModeForCommand {
        command: String,
        mode: ShellMode,
        source: Option<Box<error::Error>>,
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

    fn source(&self) -> Option<&error::Error> {
        match self {
            GeneralError::InvalidModeForCommand { source, .. } => source.as_deref(),
        }
    }
}
