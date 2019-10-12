use crate::error::ErrorTrait;

#[derive(Debug)]
pub enum CommandError {
    NotFound(String),
    RunError {
        command: String,
        description: String,
    },
}

impl ErrorTrait for CommandError {
    fn display(&self) -> String {
        match self {
            CommandError::NotFound(cmd) => format!("command {} not found", cmd),
            CommandError::RunError {
                command,
                description,
            } => format!("command {} failed to run: {}", command, description),
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
