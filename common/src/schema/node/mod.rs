mod multinode;
mod node_locator;

use super::property::Property;
use super::query::Query;
use super::{Schema, Validate, ValidationError};
pub use multinode::Multinode;
pub use node_locator::NodeLocator;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Node<'node> {
    #[serde(default)]
    pub subnodes: HashMap<String, Box<Node<'node>>>,
    #[serde(default)]
    pub multinode: Option<Box<Multinode<'node>>>,
    #[serde(default)]
    pub properties: HashMap<String, Property>,
    #[serde(default)]
    pub query: Option<Query>,
    #[serde(skip)]
    parent: Option<&'node Node<'node>>,
}

impl Node<'_> {
    pub fn node_count(&self) -> usize {
        let mut sum = 1;
        for node in self.subnodes.values() {
            sum += node.node_count();
        }
        sum
    }

    pub fn property_count(&self) -> usize {
        let mut sum = self.properties.len();
        for node in self.subnodes.values() {
            sum += node.property_count();
        }
        sum
    }

    pub fn get_locator(&self) -> NodeLocator {
        panic!();
    }
}

impl Validate for Node<'_> {
    fn validate(&self, schema: &Schema) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();
        let mut prop_keys = HashSet::new();

        for (key, property) in &self.properties {
            if !prop_keys.insert(key) {
                errors.push(ValidationError::new(format!(
                    "Property validation error\nKey: {}\nDuplicate property key",
                    &key
                )));
            }

            errors.extend(property.validate(schema));
        }

        for node in self.subnodes.values() {
            errors.extend(node.validate(schema));
        }

        if let Some(multinode) = &self.multinode {
            errors.extend(multinode.validate(schema));
        }

        errors
    }
}
