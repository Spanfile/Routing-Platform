use crate::{error, error::ErrorTrait};

#[derive(Debug)]
pub enum CommandError {
    NotFound {
        command: String,
        source: Option<Box<error::Error>>,
    },
    RunError {
        command: String,
        source: Option<Box<error::Error>>,
    },
}

impl ErrorTrait for CommandError {
    fn display(&self) -> String {
        match self {
            CommandError::NotFound { command, .. } => format!("command {} not found", command),
            CommandError::RunError { command, .. } => format!("command {} failed to run", command),
        }
    }

    fn source(&self) -> Option<&error::Error> {
        match self {
            CommandError::NotFound { source, .. } => source.as_deref(),
            CommandError::RunError { source, .. } => source.as_deref(),
        }
    }
}
