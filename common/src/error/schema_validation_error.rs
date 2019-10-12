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
        String::from("") // TODO
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
