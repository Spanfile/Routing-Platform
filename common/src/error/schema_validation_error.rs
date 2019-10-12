use super::CommonErrorTrait;
use crate::schema::{Bound, DefaultValue};
use std::error;

#[derive(Debug)]
pub enum SchemaValidationError {
    Range { lower: Bound, upper: Bound },
    DuplicateNodeName { name: String },
    Regex { regex: String, description: String },
    NoValues,
    MissingTemplate { template: String },
    NoMultipleValuesAllowed,
    InvalidDefaultValue { default: DefaultValue },
    DuplicateProperty { property: String },
}

impl CommonErrorTrait for SchemaValidationError {
    fn display(&self) -> String {
        match self {
            SchemaValidationError::Range { lower, upper } => format!(
                "lower {:?} bound higher than upper {:?} bound",
                lower, upper
            ),
            SchemaValidationError::DuplicateNodeName { name } => {
                format!("duplicate node name: {}", name)
            }
            SchemaValidationError::Regex { regex, description } => {
                format!("invalid regex {}: {}", regex, description)
            }
            SchemaValidationError::NoValues => String::from("no values defined"),
            SchemaValidationError::MissingTemplate { template } => {
                format!("template {} not found", template)
            }
            SchemaValidationError::NoMultipleValuesAllowed => {
                String::from("multiple values defined where no multiple values are allowed")
            }
            SchemaValidationError::InvalidDefaultValue { default } => {
                format!("default value {:?} doesn't match any given value", default)
            }
            SchemaValidationError::DuplicateProperty { property } => {
                format!("duplicate property name: {}", property)
            }
            _ => String::from("other schema validation error"),
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
