mod multinode;
mod node_locator;

use super::property::Property;
use super::query::Query;
use super::{Schema, Validate, ValidationError};
pub use multinode::Multinode;
pub use node_locator::NodeLocator;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Node {
    #[serde(default)]
    pub subnodes: HashMap<String, Rc<RefCell<Node>>>,
    #[serde(default)]
    pub multinode: Option<Box<Multinode>>,
    #[serde(default)]
    pub properties: HashMap<String, Property>,
    #[serde(default)]
    pub query: Option<Query>,
    #[serde(skip)]
    pub parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    pub fn node_count(&self) -> usize {
        let mut sum = 1;
        for node_rc in self.subnodes.values() {
            sum += node_rc.borrow().node_count();
        }
        sum
    }

    pub fn property_count(&self) -> usize {
        let mut sum = self.properties.len();
        for node_rc in self.subnodes.values() {
            sum += node_rc.borrow().property_count();
        }
        sum
    }

    pub fn get_locator(&self) -> NodeLocator {
        unimplemented!();
    }
}

impl Validate for Node {
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

        for node_rc in self.subnodes.values() {
            errors.extend(node_rc.borrow().validate(schema));
        }

        if let Some(multinode) = &self.multinode {
            errors.extend(multinode.validate(schema));
        }

        errors
    }
}
