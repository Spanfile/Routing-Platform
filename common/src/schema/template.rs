use super::ValidationError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub regex: String,
}

impl Template {
    pub fn validate(&self) -> Result<(), ValidationError> {
        match Regex::new(&self.regex) {
            Ok(_r) => Ok(()),
            Err(e) => Err(ValidationError::new(format!(
                "Template validation error\nName: {}\nRegex: {}\n{}",
                &self.name,
                &self.regex,
                e.description()
            ))),
        }
    }

    pub fn matches(&self, value: &String) -> bool {
        Regex::new(&self.regex).unwrap().is_match(value)
    }
}
