use super::{
    super::{Query, Validate},
    NodeLocator, Schema, SchemaNode, SchemaNodeTrait,
};
use crate::error;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiSchemaNode {
    pub source: MultiSchemaNodeSource,
    // helps a ton down the line if this node is in the heap
    // like subnodes in a single node
    pub node: Box<SchemaNode>,
    #[serde(skip)]
    pub locator: Rc<NodeLocator>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MultiSchemaNodeSource {
    Query(Query),
    Template(String),
}

impl SchemaNodeTrait for MultiSchemaNode {
    fn node_count(&self) -> usize {
        1
    }

    fn property_count(&self) -> usize {
        0
    }

    fn get_locator(&self) -> Rc<NodeLocator> {
        Rc::clone(&self.locator)
    }

    fn update_locators(&mut self, name: String, locator: Rc<NodeLocator>) {
        self.locator = Rc::new(NodeLocator::new(name, Some(Rc::clone(&locator))));
        self.node
            .update_locators(String::from("template"), Rc::clone(&self.locator));
    }
}

impl Validate for MultiSchemaNode {
    fn validate(&self, schema: &Schema) -> error::CommonResult<()> {
        self.node.validate(schema)?;

        if let MultiSchemaNodeSource::Template(template) = &self.source {
            if !schema.templates.contains_key(template) {
                return Err(error::SchemaValidationError::MissingTemplate {
                    template: template.to_owned(),
                    source: None,
                }
                .into());
            }
        }

        Ok(())
    }
}
