use crate::{error, error::ErrorTrait};
use std::convert::From;

#[derive(Debug)]
pub enum ShellError {
    CannotEnterState {
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
            ShellError::CannotEnterState { .. } => String::from("cannot enter state"),
            ShellError::Io { err, .. } => err.to_string(),
        }
    }

    fn source(&self) -> Option<&error::Error> {
        match self {
            ShellError::CannotEnterState { source } => source.as_deref(),
            ShellError::Io { source, .. } => source.as_deref(),
        }
    }
}
