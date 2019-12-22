mod node_source;

use super::{
    super::{SourceCommand, Validate},
    Merge, MergingStrategy, NodeLocator, Schema, SchemaNode, SchemaNodeTrait,
};
pub use node_source::MultiSchemaNodeSource;
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

impl SchemaNodeTrait for MultiSchemaNode {
    fn node_count(&self) -> usize {
        1 + self.node.node_count()
    }

    fn property_count(&self) -> usize {
        self.node.property_count()
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
    fn validate(&self, schema: &Schema) -> anyhow::Result<()> {
        self.node.validate(schema)?;
        self.source.validate(schema)?;

        Ok(())
    }
}

impl Merge for MultiSchemaNode {
    fn merge(&mut self, other: Self, strategy: MergingStrategy) -> anyhow::Result<()> {
        self.node.merge(*other.node, strategy)?;
        Ok(())
    }
}
