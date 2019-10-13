use crate::{Bound, DefaultValue};
use rp_error::ErrorTrait;

#[derive(Debug)]
pub enum SchemaValidationError {
    Range {
        lower: Bound,
        upper: Bound,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    Regex {
        regex: String,
        description: String,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    NoValues {
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    MissingTemplate {
        template: String,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    NoMultipleValuesAllowed {
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    InvalidDefaultValue {
        default: DefaultValue,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
}

impl ErrorTrait for SchemaValidationError {
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

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
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
