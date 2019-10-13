use super::{CommonError, CommonErrorTrait};
use crate::schema::Value;

#[derive(Debug)]
pub struct ConstraintError {
    pub given: String,
    pub allowed_values: Vec<Value>,
    pub source: Option<Box<CommonError>>,
}

impl CommonErrorTrait for ConstraintError {
    fn display(&self) -> String {
        format!(
            "Given value '{}' doesn't match any allowed value.\n   Allowed values:\n   -> {}",
            self.given,
            self.allowed_values
                .iter()
                .map(|v| format!("{:?}", v))
                .collect::<Vec<String>>()
                .join("\n   -> ")
        )
    }

    fn source(&self) -> Option<&CommonError> {
        self.source.as_deref()
    }
}
