mod multi_config_node;
mod single_config_node;

use super::{NodeName, Property};
use crate::{
    schema::{Schema, SchemaNode},
    Context,
};
use enum_dispatch::enum_dispatch;
use multi_config_node::MultiConfigNode;
use single_config_node::SingleConfigNode;
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

#[enum_dispatch]
pub trait Node {
    fn name(&self) -> String;
    fn full_path(&self) -> String;
    fn get_available_node_names(&self) -> Vec<NodeName>;
    fn get_available_property_names(&self) -> Vec<String>;
    fn get_node_with_name(&self, name: &str) -> &ConfigNode;
    fn get_property(&self, property: &str) -> Option<&Property>;
    fn get_property_values(&self, of_property: Option<String>) -> HashMap<String, Vec<String>>;
    fn pretty_print(&self, indent: usize);
}

pub trait FromSchemaNode<TBuiltFrom>
where
    Self: std::marker::Sized,
{
    fn from_schema_node(
        parent: &str,
        context: Rc<Context>,
        name: &str,
        schema: Weak<Schema>,
        schema_node: &TBuiltFrom,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>>;
}

#[enum_dispatch(Node)]
#[derive(Debug)]
pub enum ConfigNode {
    SingleConfigNode,
    MultiConfigNode,
}

impl FromSchemaNode<SchemaNode> for ConfigNode {
    fn from_schema_node(
        parent: &str,
        context: Rc<Context>,
        name: &str,
        schema: Weak<Schema>,
        schema_node: &SchemaNode,
    ) -> Result<Vec<ConfigNode>, Box<dyn std::error::Error>> {
        match schema_node {
            SchemaNode::Single(single_schema_node) => Ok(SingleConfigNode::from_schema_node(
                parent,
                context,
                name,
                schema,
                single_schema_node,
            )?
            .into_iter()
            .map(|n| n.into())
            .collect()),
            SchemaNode::Multi(multi_schema_node) => Ok(MultiConfigNode::from_schema_node(
                parent,
                context,
                name,
                schema,
                multi_schema_node,
            )?
            .into_iter()
            .map(|n| n.into())
            .collect()),
        }
    }
}
