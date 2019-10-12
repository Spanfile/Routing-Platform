use crate::shell::{CommandError, ShellError};
use enum_dispatch::enum_dispatch;
use std::fmt;

pub type CustomResult<T> = std::result::Result<T, Error>; // TODO; bad name

#[enum_dispatch]
pub trait ErrorTrait {
    fn display(&self) -> String;
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>;
}

#[enum_dispatch(ErrorTrait)]
#[derive(Debug)]
pub enum Error {
    ShellError,
    CommandError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        ErrorTrait::source(self)
    }
}

impl From<std::io::Error> for Error {
    fn from(item: std::io::Error) -> Self {
        ShellError::Io(item).into()
    }
}
