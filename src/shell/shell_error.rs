use crate::error::ErrorTrait;
use std::convert::From;

#[derive(Debug)]
pub enum ShellError {
    CannotEnterState,
    Io(std::io::Error),
}

impl ErrorTrait for ShellError {
    fn display(&self) -> String {
        String::from("")
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
