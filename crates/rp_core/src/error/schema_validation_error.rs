use crate::schema::{Bound, DefaultValue};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaValidationError {
    #[error("Lower {lower:?} bound higher than upper {upper:?} bound")]
    Range { lower: Bound, upper: Bound },
    #[error("Invalid regex {regex}: {description}")]
    Regex { regex: String, description: String },
    #[error("No values defined")]
    NoValues,
    #[error("Template '{0}' not found")]
    MissingTemplate(String),
    #[error("Multiple values defined where no multiple values are allowed")]
    NoMultipleValuesAllowed,
    #[error("Default value {0:?} doesn't match any given value")]
    InvalidDefaultValue(DefaultValue),
}
