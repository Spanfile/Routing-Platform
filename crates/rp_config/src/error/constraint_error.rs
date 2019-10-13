use rp_error::ErrorTrait;
use rp_schema::Value;

#[derive(Debug)]
pub struct ConstraintError {
    pub given: String,
    pub allowed_values: Vec<Value>,
    pub source: Option<Box<dyn ErrorTrait>>,
}

impl ErrorTrait for ConstraintError {
    fn display(&self) -> String {
        format!(
            "Given value '{}' doesn't match any allowed value.\n   Allowed values:\n   -> {}",
            self.given,
            self.allowed_values
                .iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>()
                .join("\n   -> ")
        )
    }

    fn source(&self) -> Option<&(dyn ErrorTrait)> {
        self.source.as_deref()
    }
}
