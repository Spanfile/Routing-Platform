use super::{ConfigNode, FromSchemaNode, Node, NodeName};
use crate::Property;
use rp_common::Context;
use rp_schema::{Schema, SingleSchemaNode};
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct SingleConfigNode {
    name: String,
    subnodes: HashMap<String, Rc<ConfigNode>>,
    properties: HashMap<String, Property>,
}

impl Node for SingleConfigNode {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn get_available_node_names(&self) -> Vec<NodeName> {
        let mut names = Vec::new();

        for name in self.subnodes.keys() {
            names.push(NodeName::Literal(name.to_owned()));
        }

        names
    }

    fn get_available_property_names(&self) -> Vec<String> {
        self.properties.keys().map(|key| key.to_owned()).collect()
    }

    fn get_node_with_name(&self, name: &str) -> Rc<ConfigNode> {
        Rc::clone(self.subnodes.get(name).expect("node not found"))
    }

    fn get_property(&self, property: &str) -> Option<&Property> {
        self.properties.get(property)
    }

    fn get_property_values(&self, of_property: Option<String>) -> HashMap<String, Vec<String>> {
        self.properties
            .iter()
            .filter(|(key, _p)| match &of_property {
                Some(prop) => *key == prop,
                None => true,
            })
            .map(|(key, property)| {
                (
                    key.to_owned(),
                    property
                        .values()
                        .iter()
                        .map(|v| v.to_owned()) // maybe not the greatest idea to clone the values?
                        .collect(),
                )
            })
            .collect()
    }

    fn pretty_print(&self, indent: usize) {
        for (name, node) in &self.subnodes {
            println!("{:indent$}{} {{", "", name, indent = indent * 4);
            node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }

        for property in self.properties.values() {
            property.pretty_print(indent);
        }
    }
}

impl FromSchemaNode<SingleSchemaNode> for SingleConfigNode {
    fn from_schema_node(
        context: Rc<Context>,
        name: &str,
        schema: Weak<Schema>,
        schema_node: &SingleSchemaNode,
    ) -> anyhow::Result<ConfigNode> {
        let name = context.format(name.to_owned())?;
        let mut subnodes = HashMap::new();
        let mut properties = HashMap::new();

        for (subname, subnode) in &schema_node.subnodes {
            subnodes.insert(
                subname.to_owned(),
                Rc::new(ConfigNode::from_schema_node(
                    Rc::clone(&context),
                    &subname,
                    Weak::clone(&schema),
                    &subnode,
                )?),
            );
        }

        for (key, property) in &schema_node.properties {
            let prop = Property::from_schema_property(
                Rc::clone(&context),
                &key,
                property,
                Weak::clone(&schema),
            )?;
            properties.insert(key.to_owned(), prop);
        }

        Ok(SingleConfigNode {
            name,
            subnodes,
            properties,
        }
        .into())
    }
}
