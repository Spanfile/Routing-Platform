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
    MissingArgument {
        argument: String,
        source: Option<Box<error::Error>>,
    },
}

impl ErrorTrait for CommandError {
    fn display(&self) -> String {
        match self {
            CommandError::NotFound { command, .. } => format!("No such command: {}", command),
            CommandError::RunError { command, .. } => {
                format!("Runtime error in command '{}'", command)
            }
            CommandError::MissingArgument { argument, .. } => {
                format!("Missing argument: {}", argument)
            }
        }
    }

    fn source(&self) -> Option<&error::Error> {
        match self {
            CommandError::NotFound { source, .. } => source.as_deref(),
            CommandError::RunError { source, .. } => source.as_deref(),
            CommandError::MissingArgument { source, .. } => source.as_deref(),
        }
    }
}
