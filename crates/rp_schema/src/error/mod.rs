mod io_error;
mod query_error;
mod regex_automata_error;
mod schema_validation_error;
mod serde_error;

pub use io_error::IoError;
pub use query_error::QueryError;
pub use regex_automata_error::RegexAutomataError;
use rp_error::ErrorTrait;
pub use schema_validation_error::SchemaValidationError;
pub use serde_error::SerdeError;
use std::convert::From;

pub type Result<T> = std::result::Result<T, SchemaError>;

#[derive(Debug)]
pub enum SchemaError {
    SchemaValidation(SchemaValidationError),
    Serde(SerdeError),
    Io(IoError),
    RegexAutomata(RegexAutomataError),
    Query(QueryError),
}

impl ErrorTrait for SchemaError {
    fn display(&self) -> String {
        match self {
            SchemaError::SchemaValidation(err) => err.display(),
            SchemaError::Serde(err) => err.display(),
            SchemaError::Io(err) => err.display(),
            SchemaError::RegexAutomata(err) => err.display(),
            SchemaError::Query(err) => err.display(),
        }
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        match self {
            SchemaError::SchemaValidation(err) => err.source(),
            SchemaError::Serde(err) => err.source(),
            SchemaError::Io(err) => err.source(),
            SchemaError::RegexAutomata(err) => err.source(),
            SchemaError::Query(err) => err.source(),
        }
    }
}

impl From<SchemaValidationError> for SchemaError {
    fn from(item: SchemaValidationError) -> Self {
        SchemaError::SchemaValidation(item)
    }
}

impl From<SerdeError> for SchemaError {
    fn from(item: SerdeError) -> Self {
        SchemaError::Serde(item)
    }
}

impl From<IoError> for SchemaError {
    fn from(item: IoError) -> Self {
        SchemaError::Io(item)
    }
}

impl From<RegexAutomataError> for SchemaError {
    fn from(item: RegexAutomataError) -> Self {
        SchemaError::RegexAutomata(item)
    }
}

impl From<QueryError> for SchemaError {
    fn from(item: QueryError) -> Self {
        SchemaError::Query(item)
    }
}
