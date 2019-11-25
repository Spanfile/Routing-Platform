mod multi_config_node;
mod single_config_node;

use super::{Changeable, NodeName, Property, Save, SaveBuilder};
use enum_dispatch::enum_dispatch;
use multi_config_node::MultiConfigNode;
use rp_common::Context;
use rp_schema::{Schema, SchemaNode};
use single_config_node::SingleConfigNode;
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

#[enum_dispatch]
pub trait Node: Changeable + Save {
    fn name(&self) -> String;

    fn get_available_node_names(&self) -> Vec<NodeName>;
    fn get_available_property_names(&self) -> Vec<String>;
    fn get_node_with_name(&self, name: &str) -> Rc<ConfigNode>;
    fn get_property(&self, property: &str) -> Option<&Property>;
    fn get_property_values(&self, of_property: Option<String>) -> HashMap<String, Vec<String>>;

    fn remove_subnode(&self, subnode: &str) -> anyhow::Result<()>;

    fn pretty_print(&self, indent: usize);
}

pub trait FromSchemaNode<TBuiltFrom>
where
    Self: std::marker::Sized,
{
    fn from_schema_node(
        context: Rc<Context>,
        name: &str,
        schema: Weak<Schema>,
        schema_node: &TBuiltFrom,
    ) -> anyhow::Result<ConfigNode>;
}

#[enum_dispatch(Node)]
#[derive(Debug)]
pub enum ConfigNode {
    SingleConfigNode,
    MultiConfigNode,
}

impl FromSchemaNode<SchemaNode> for ConfigNode {
    fn from_schema_node(
        context: Rc<Context>,
        name: &str,
        schema: Weak<Schema>,
        schema_node: &SchemaNode,
    ) -> anyhow::Result<ConfigNode> {
        match schema_node {
            SchemaNode::SingleSchemaNode(node) => Ok(SingleConfigNode::from_schema_node(
                context, name, schema, node,
            )?),
            SchemaNode::MultiSchemaNode(node) => Ok(MultiConfigNode::from_schema_node(
                context, name, schema, node,
            )?),
        }
    }
}

impl Changeable for ConfigNode {
    fn is_clean(&self) -> bool {
        match self {
            Self::SingleConfigNode(node) => node.is_clean(),
            Self::MultiConfigNode(node) => node.is_clean(),
        }
    }

    fn apply_changes(&self) -> anyhow::Result<bool> {
        match self {
            Self::SingleConfigNode(node) => node.apply_changes(),
            Self::MultiConfigNode(node) => node.apply_changes(),
        }
    }

    fn discard_changes(&self) {
        match self {
            Self::SingleConfigNode(node) => node.discard_changes(),
            Self::MultiConfigNode(node) => node.discard_changes(),
        }
    }
}

impl Save for ConfigNode {
    fn save(&self, builder: &mut SaveBuilder) -> anyhow::Result<()> {
        match self {
            Self::SingleConfigNode(node) => node.save(builder),
            Self::MultiConfigNode(node) => node.save(builder),
        }
    }
}
