use super::{CommonError, CommonErrorTrait};
use crate::schema::{Bound, DefaultValue};

#[derive(Debug)]
pub enum SchemaValidationError {
    Range {
        lower: Bound,
        upper: Bound,
        source: Option<Box<CommonError>>,
    },
    Regex {
        regex: String,
        description: String,
        source: Option<Box<CommonError>>,
    },
    NoValues {
        source: Option<Box<CommonError>>,
    },
    MissingTemplate {
        template: String,
        source: Option<Box<CommonError>>,
    },
    NoMultipleValuesAllowed {
        source: Option<Box<CommonError>>,
    },
    InvalidDefaultValue {
        default: DefaultValue,
        source: Option<Box<CommonError>>,
    },
}

impl CommonErrorTrait for SchemaValidationError {
    fn display(&self) -> String {
        match self {
            SchemaValidationError::Range { lower, upper, .. } => format!(
                "lower {:?} bound higher than upper {:?} bound",
                lower, upper
            ),
            SchemaValidationError::Regex {
                regex, description, ..
            } => format!("invalid regex {}: {}", regex, description),
            SchemaValidationError::NoValues { .. } => String::from("no values defined"),
            SchemaValidationError::MissingTemplate { template, .. } => {
                format!("template {} not found", template)
            }
            SchemaValidationError::NoMultipleValuesAllowed { .. } => {
                String::from("multiple values defined where no multiple values are allowed")
            }
            SchemaValidationError::InvalidDefaultValue { default, .. } => {
                format!("default value {:?} doesn't match any given value", default)
            }
        }
    }

    fn source(&self) -> Option<&CommonError> {
        match self {
            SchemaValidationError::Range { source, .. } => source.as_deref(),
            SchemaValidationError::Regex { source, .. } => source.as_deref(),
            SchemaValidationError::NoValues { source, .. } => source.as_deref(),
            SchemaValidationError::MissingTemplate { source, .. } => source.as_deref(),
            SchemaValidationError::NoMultipleValuesAllowed { source, .. } => source.as_deref(),
            SchemaValidationError::InvalidDefaultValue { source, .. } => source.as_deref(),
        }
    }
}
