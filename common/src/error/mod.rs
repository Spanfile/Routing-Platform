mod regex_automata_error;
mod schema_validation_error;
mod serde_error;

use enum_dispatch::enum_dispatch;
pub use regex_automata_error::RegexAutomataError;
pub use schema_validation_error::SchemaValidationError;
pub use serde_error::SerdeError;
use std::fmt;

pub type CommonResult<T> = std::result::Result<T, CommonError>;

#[enum_dispatch]
trait CommonErrorTrait {
    fn display(&self) -> String;
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>;
}

#[enum_dispatch(CommonErrorTrait)]
#[derive(Debug)]
pub enum CommonError {
    SchemaValidationError,
    SerdeError,
    RegexAutomataError,
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl std::error::Error for CommonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        CommonErrorTrait::source(self)
    }
}

impl From<serde_json::Error> for CommonError {
    fn from(item: serde_json::error::Error) -> Self {
        SerdeError::Json(item).into()
    }
}

impl From<serde_yaml::Error> for CommonError {
    fn from(item: serde_yaml::Error) -> Self {
        SerdeError::Yaml(item).into()
    }
}

impl From<regex_automata::Error> for CommonError {
    fn from(item: regex_automata::Error) -> Self {
        RegexAutomataError { error: item }.into()
    }
}
