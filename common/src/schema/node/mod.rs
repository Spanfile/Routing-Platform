mod multi_schema_node;
mod node_locator;
mod single_schema_node;

use super::{Schema, Validate, ValidationError};
pub use multi_schema_node::MultiSchemaNode;
pub use node_locator::NodeLocator;
use serde::{Deserialize, Serialize};
pub use single_schema_node::SingleSchemaNode;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SchemaNode {
    /*
     * Multi has to be the first variant because:
     * - The enum is untagged, which means when deserialising Serde will try to match the
     *   variant based on the given properties
     * - Each field in SingleSchemaNode is tagged as default, which means when matching the
     *   variant Serde will assume each to have their default value and thus, be automatically
     *   valid matches
     * - If Single is checked first, each of its fields will always match any properties since
     *   they always have a valid default value
     * - Serde seems to have no way of discarding a matched variant if no given property was
     *   matched, but instead only defaults were matched
     * - Thus, Single will always match even if the properties actually were Multi's
     * - The "hacky" workaround is to have Serde try match Multi first, since it has required
     *   fields
     */
    Multi(MultiSchemaNode),
    Single(SingleSchemaNode),
}

impl SchemaNode {
    pub fn node_count(&self) -> usize {
        match self {
            SchemaNode::Single(singlenode) => singlenode.node_count(),
            SchemaNode::Multi(multinode) => multinode.node_count(),
        }
    }

    pub fn property_count(&self) -> usize {
        match self {
            SchemaNode::Single(singlenode) => singlenode.property_count(),
            SchemaNode::Multi(multinode) => multinode.property_count(),
        }
    }

    pub fn get_locator(&self) -> NodeLocator {
        unimplemented!();
        // NodeLocator::new(self.name.to_string(), None)
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

impl Validate for SchemaNode {
    fn validate(&self, schema: &Schema) -> Vec<ValidationError> {
        match self {
            SchemaNode::Single(singlenode) => singlenode.validate(schema),
            SchemaNode::Multi(multinode) => multinode.validate(schema),
        }
    }
}
