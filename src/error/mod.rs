mod common_error;

use super::GeneralError;
use crate::{
    shell::{CommandError, ShellError},
    ConfigEditorError,
};
use common_error::CommonError;
use enum_dispatch::enum_dispatch;
use std::fmt;

pub type CustomResult<T> = std::result::Result<T, Error>; // TODO; bad name

#[enum_dispatch]
pub trait ErrorTrait {
    fn display(&self) -> String;
    fn source(&self) -> Option<&Error>;
}

#[enum_dispatch(ErrorTrait)]
#[derive(Debug)]
pub enum Error {
    ShellError,
    CommandError,
    GeneralError,
    ConfigEditorError,
    CommonError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())?;
        if let Some(source) = self.source() {
            write!(f, "\nCaused by: {}", source.display())
        } else {
            Ok(())
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None // TODO
    }
}

impl From<std::io::Error> for Error {
    fn from(item: std::io::Error) -> Self {
        ShellError::Io {
            err: item,
            source: None,
        }
        .into()
    }
}

impl From<common::error::CommonError> for Error {
    fn from(item: common::error::CommonError) -> Self {
        CommonError { source: item }.into()
    }
}
