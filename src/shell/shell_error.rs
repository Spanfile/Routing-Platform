use super::ShellMode;
use crate::{error, error::ErrorTrait};

#[derive(Debug)]
pub enum ShellError {
    CannotEnterMode {
        mode: ShellMode,
        source: Option<Box<error::Error>>,
    },
    Io {
        err: std::io::Error,
        source: Option<Box<error::Error>>,
    },
}

impl ErrorTrait for ShellError {
    fn display(&self) -> String {
        match self {
            ShellError::CannotEnterMode { mode, .. } => {
                format!("cannot enter mode (from {:?})", mode)
            }
            ShellError::Io { err, .. } => err.to_string(),
        }
    }

    fn source(&self) -> Option<&error::Error> {
        match self {
            ShellError::CannotEnterMode { source, .. } => source.as_deref(),
            ShellError::Io { source, .. } => source.as_deref(),
        }
    }
}
