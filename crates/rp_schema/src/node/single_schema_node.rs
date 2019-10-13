use super::{
    super::property::Property, NodeLocator, Schema, SchemaNode, SchemaNodeTrait, Validate,
};
use crate::error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};

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

        for (subname, subnode) in &mut self.subnodes {
            subnode.update_locators(subname.to_owned(), Rc::clone(&self.locator));
        }
    }
}

impl Validate for SingleSchemaNode {
    fn validate(&self, schema: &Schema) -> error::Result<()> {
        for property in self.properties.values() {
            property.validate(schema)?;
        }

        for node in self.subnodes.values() {
            node.validate(schema)?;
        }

        Ok(())
    }
}
