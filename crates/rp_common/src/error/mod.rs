mod format_error;

pub use format_error::FormatError;
use rp_error::ErrorTrait;
use std::convert::From;

pub type Result<T> = std::result::Result<T, CommonError>;

#[derive(Debug)]
pub enum CommonError {
    Format(FormatError),
}

impl ErrorTrait for CommonError {
    fn display(&self) -> String {
        match self {
            CommonError::Format(err) => err.display(),
        }
    }

    fn source(&self) -> Option<&(dyn ErrorTrait)> {
        match self {
            CommonError::Format(err) => err.source(),
        }
    }
}

impl From<FormatError> for CommonError {
    fn from(item: FormatError) -> Self {
        CommonError::Format(item)
    }
}
