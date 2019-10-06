use super::Schema;

pub trait Validate {
    fn validate(&self, schema: &Schema) -> Vec<ValidationError>;
}

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
}

impl ValidationError {
    pub fn new(message: String) -> ValidationError {
        ValidationError { message }
    }
}
