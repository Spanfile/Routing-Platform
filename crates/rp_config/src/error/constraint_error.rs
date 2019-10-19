use rp_schema::Value;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Given value '{}' doesn't match any allowed value.\n   Allowed values:\n   -> {}", given, allowed_values
                .iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>()
                .join("\n   -> "))]
pub struct ConstraintError {
    pub given: String,
    pub allowed_values: Vec<Value>,
}
