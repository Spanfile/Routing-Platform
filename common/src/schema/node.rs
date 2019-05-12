use super::property::Property;
use super::query::Query;
use super::{Schema, Validate, ValidationError};
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
    pub fn node_count(&self) -> usize {
        let mut sum = 1;
        for node in &self.subnodes {
            sum += node.node_count();
        }
        sum
    }

    pub fn property_count(&self) -> usize {
        let mut sum = self.properties.len();
        for node in &self.subnodes {
            sum += node.property_count();
        }
        sum
    }
}

impl Validate for Node {
    fn validate(&self, schema: &Schema) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();
        let mut prop_keys = HashSet::new();

        for prop in &self.properties {
            if !prop_keys.insert(&prop.key) {
                errors.push(ValidationError::new(format!(
                    "Property validation error\nKey: {}\nDuplicate property key",
                    prop.key
                )));
            }

            errors.extend(prop.validate(schema));
        }

        for node in &self.subnodes {
            errors.extend(node.validate(schema));
        }

        errors
    }
}
