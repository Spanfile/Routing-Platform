use super::property::Property;
use super::query::Query;
use super::{Schema, ValidationError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Node {
    pub name: String,
    #[serde(default)]
    pub subnodes: Vec<Box<Node>>,
    #[serde(default)]
    pub properties: Vec<Property>,
    #[serde(default)]
    pub query: Option<Query>,
}

impl Node {
    pub fn validate(&self, schema: &Schema) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();
        let mut prop_keys = HashSet::new();

        for prop in &self.properties {
            if !prop_keys.insert(&prop.key) {
                errors.push(ValidationError::new(format!(
                    "Property validation error\nKey: {}\nDuplicate property key",
                    prop.key
                )));
            }

            match prop.validate(schema) {
                Ok(()) => (),
                Err(e) => errors.push(e),
            }
        }

        for node in &self.subnodes {
            errors.extend(node.validate(schema));
        }

        errors
    }
}
