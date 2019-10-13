mod multi_schema_node;
mod node_locator;
mod single_schema_node;
use enum_dispatch::enum_dispatch;

use super::{Schema, Validate};
use crate::error;
pub use multi_schema_node::{MultiSchemaNode, MultiSchemaNodeSource};
pub use node_locator::NodeLocator;
use serde::{Deserialize, Serialize};
pub use single_schema_node::SingleSchemaNode;
use std::rc::Rc;

#[enum_dispatch(SchemaNode)]
pub trait SchemaNodeTrait {
    fn node_count(&self) -> usize;
    fn property_count(&self) -> usize;
    fn update_locators(&mut self, name: String, previous: Rc<NodeLocator>);
    fn get_locator(&self) -> Rc<NodeLocator>;
}

#[enum_dispatch]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SchemaNode {
    // Multi has to be the first variant because:
    // - The enum is #[serde(untagged)], which means when deserialising Serde will try to match
    //   the variant based on the given properties
    // - Each field in SingleSchemaNode is tagged as default, which means when matching the
    //   variant Serde will assume each to have their default value and thus, be automatically
    //   valid matches
    // - If Single is checked first, each of its fields will always match any properties since
    //   they always have a valid default value
    // - Serde seems to have no way of discarding a matched variant if no given property was
    //   matched, but instead only defaults were matched
    // - Thus, Single will always match even if the properties actually were Multi's
    // - The "hacky" workaround is to have Serde try match Multi first, since it has required
    //   fields
    MultiSchemaNode,
    SingleSchemaNode,
}

// enum_dispatch could be used to get rid of this boilerplate impl but it
// doesn't seem to support linking multiple traits to a single enum, only
// multiple enums to a single trait
impl Validate for SchemaNode {
    fn validate(&self, schema: &Schema) -> error::Result<()> {
        match self {
            SchemaNode::SingleSchemaNode(node) => node.validate(schema),
            SchemaNode::MultiSchemaNode(node) => node.validate(schema),
        }
    }
}
