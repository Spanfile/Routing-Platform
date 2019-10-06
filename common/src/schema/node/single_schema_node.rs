use super::{
    super::{property::Property, query::Query},
    NodeLocator, Schema, SchemaNode, Validate, ValidationError,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleSchemaNode {
    #[serde(default)]
    pub subnodes: HashMap<String, Box<SchemaNode>>,
    #[serde(default)]
    pub properties: HashMap<String, Property>,
    #[serde(default)]
    pub query: Option<Query>,
    #[serde(default)]
    pub name: String,
}

impl SingleSchemaNode {
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
        NodeLocator::new(self.name.to_string(), None)
        // if let Some(parent) = &self.parent {
        //     if let Some(parent) = parent.upgrade() {
        //         NodeLocator::new(self.name.to_string(),
        // Some(parent.borrow().get_locator()))     } else {
        //         panic!("parent node dropped");
        //     }
        // } else {
        //     NodeLocator::new(self.name.to_string(), None)
        // }
    }
}

impl Validate for SingleSchemaNode {
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

        // if let Some(multinode) = &self.multinode {
        //     errors.extend(multinode.validate(schema));
        // }

        errors
    }
}
