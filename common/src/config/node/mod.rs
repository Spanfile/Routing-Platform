mod multi_config_node;
mod single_config_node;

use super::{NodeName, Property};
use crate::{schema::SchemaNode, Context};
use enum_dispatch::enum_dispatch;
use multi_config_node::MultiConfigNode;
use single_config_node::SingleConfigNode;
use std::{collections::HashMap, rc::Rc};

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

trait FromSchemaNode<TBuiltFrom>
where
    Self: std::marker::Sized,
{
    fn from_schema_node(
        parent: &str,
        context: Rc<Context>,
        name: &str,
        schema_node: &TBuiltFrom,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>>;
}

#[enum_dispatch(Node)]
#[derive(Debug)]
pub enum ConfigNode {
    SingleConfigNode,
    MultiConfigNode,
}

impl ConfigNode {
    pub fn from_schema_node(
        parent: &str,
        context: Rc<Context>,
        name: &str,
        schema_node: &SchemaNode,
    ) -> Result<Vec<ConfigNode>, Box<dyn std::error::Error>> {
        match schema_node {
            SchemaNode::Single(single_schema_node) => {
                Ok(
                    SingleConfigNode::from_schema_node(parent, context, name, single_schema_node)?
                        .into_iter()
                        .map(|n| n.into())
                        .collect(),
                )
            }
            SchemaNode::Multi { .. } => unimplemented!(),
        }
    }
}

// impl ConfigNode {
//     pub fn name(&self) -> String {
//         match self {
//             ConfigNode::Single(single_config_node) =>
// single_config_node.name(),             ConfigNode::Multi(multi_config_node)
// => multi_config_node.name(),         }
//     }

//     pub fn full_path(&self) -> String {
//         match self {
//             ConfigNode::Single(single_config_node) =>
// single_config_node.full_path(),             ConfigNode::Multi =>
// unimplemented!(),         }
//     }

//

//     pub fn get_available_node_names(&self) -> Vec<NodeName> {
//         match self {
//             ConfigNode::Single(single_config_node) =>
// single_config_node.get_available_node_names(),             ConfigNode::Multi
// => unimplemented!(),         }
//     }

//     pub fn get_available_property_names(&self) -> Vec<String> {
//         match self {
//             ConfigNode::Single(single_config_node) => {
//                 single_config_node.get_available_property_names()
//             }
//             ConfigNode::Multi => unimplemented!(),
//         }
//     }

//     pub fn get_node_with_name(&self, name: &str) -> &ConfigNode {
//         match self {
//             ConfigNode::Single(single_config_node) =>
// single_config_node.get_node_with_name(name),             ConfigNode::Multi =>
// unimplemented!(),         }
//     }

//     pub fn get_property(&self, property: &str) -> Option<&Property> {
//         match self {
//             ConfigNode::Single(single_config_node) =>
// single_config_node.get_property(property),             ConfigNode::Multi =>
// unimplemented!(),         }
//     }

//     pub fn get_property_values(&self, of_property: Option<String>) ->
// HashMap<String, Vec<String>> {         match self {
//             ConfigNode::Single(single_config_node) => {
//                 single_config_node.get_property_values(of_property)
//             }
//             ConfigNode::Multi => unimplemented!(),
//         }
//     }
// }

// impl ConfigNode {
//     pub fn pretty_print(&self, indent: usize) {
//         match self {
//             ConfigNode::Single(single_config_node) =>
// single_config_node.pretty_print(indent),             ConfigNode::Multi =>
// unimplemented!(),         }
//     }
// }
