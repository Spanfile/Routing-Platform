use super::{
    super::property::Property, NodeLocator, Schema, SchemaNode, SchemaNodeTrait, Validate,
    ValidationError,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleSchemaNode {
    #[serde(default)]
    pub subnodes: HashMap<String, Box<SchemaNode>>,
    #[serde(default)]
    pub properties: HashMap<String, Property>,
    // #[serde(default)]
    // pub name: String,
    #[serde(skip)]
    pub locator: Rc<NodeLocator>,
}

impl SchemaNodeTrait for SingleSchemaNode {
    fn node_count(&self) -> usize {
        let mut sum = 1;
        for node in self.subnodes.values() {
            sum += node.node_count();
        }
        sum
    }

    fn property_count(&self) -> usize {
        let mut sum = self.properties.len();
        for node in self.subnodes.values() {
            sum += node.property_count();
        }
        sum
    }

    fn get_locator(&self) -> Rc<NodeLocator> {
        self.locator.clone()
    }

    fn update_locators(&mut self, name: String, locator: Rc<NodeLocator>) {
        self.locator = Rc::new(NodeLocator::new(name, Some(Rc::clone(&locator))));
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

        errors
    }
}
