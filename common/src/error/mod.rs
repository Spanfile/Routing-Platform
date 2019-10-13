mod constraint_error;
mod format_error;
mod io_error;
mod property_error;
mod regex_automata_error;
mod schema_validation_error;
mod serde_error;

pub use constraint_error::ConstraintError;
use enum_dispatch::enum_dispatch;
pub use format_error::FormatError;
pub use io_error::IoError;
pub use property_error::PropertyError;
pub use regex_automata_error::RegexAutomataError;
pub use schema_validation_error::SchemaValidationError;
pub use serde_error::SerdeError;
use std::fmt;

pub type CommonResult<T> = std::result::Result<T, CommonError>;

#[enum_dispatch]
pub trait CommonErrorTrait {
    fn display(&self) -> String;
    fn source(&self) -> Option<&CommonError>;
}

#[enum_dispatch(CommonErrorTrait)]
#[derive(Debug)]
pub enum CommonError {
    SchemaValidationError,
    PropertyError,
    SerdeError,
    RegexAutomataError,
    ConstraintError,
    IoError,
    FormatError,
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())?;
        if let Some(source) = self.source() {
            write!(f, "\n-> {}", source)
        } else {
            Ok(())
        }
    }
}

impl std::error::Error for CommonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<serde_json::Error> for CommonError {
    fn from(item: serde_json::error::Error) -> Self {
        SerdeError::Json {
            error: item,
            source: None,
        }
        .into()
    }
}

impl From<serde_yaml::Error> for CommonError {
    fn from(item: serde_yaml::Error) -> Self {
        SerdeError::Yaml {
            error: item,
            source: None,
        }
        .into()
    }
}

impl From<regex_automata::Error> for CommonError {
    fn from(item: regex_automata::Error) -> Self {
        RegexAutomataError {
            error: item,
            source: None,
        }
        .into()
    }
}

impl From<std::io::Error> for CommonError {
    fn from(item: std::io::Error) -> Self {
        IoError {
            error: item,
            source: None,
        }
        .into()
    }
}
