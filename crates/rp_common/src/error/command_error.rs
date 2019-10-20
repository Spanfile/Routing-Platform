use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("No such command: '{0}'")]
    NotFound(String),
    #[error("Missing argument: {0}")]
    MissingArgument(String),
    #[error("Unexpected argument: {0}")]
    UnexpectedArgument(String),
}
