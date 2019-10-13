use rp_error::ErrorTrait;

#[derive(Debug)]
pub enum CommandError {
    NotFound {
        command: String,
        source: Option<Box<dyn ErrorTrait>>,
    },
    MissingArgument {
        argument: String,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    UnexpectedArgument {
        argument: String,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
}

impl ErrorTrait for CommandError {
    fn display(&self) -> String {
        match self {
            CommandError::NotFound { command, .. } => format!("No such command: {}", command),
            CommandError::MissingArgument { argument, .. } => {
                format!("Missing argument: {}", argument)
            }
            CommandError::UnexpectedArgument { argument, .. } => {
                format!("Unexpected argument: {}", argument)
            }
        }
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        match self {
            CommandError::NotFound { source, .. } => source.as_deref(),
            CommandError::MissingArgument { source, .. } => source.as_deref(),
            CommandError::UnexpectedArgument { source, .. } => source.as_deref(),
        }
    }
}
