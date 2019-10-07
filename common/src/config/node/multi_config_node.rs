use super::{ConfigNode, FromSchemaNode, Node, NodeName};
use crate::{
    config::Property,
    schema::{MultiSchemaNode, NodeLocator, Schema, SchemaNodeTrait},
    Context,
};
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct MultiConfigNode {
    nodes: HashMap<String, Box<ConfigNode>>,
    name: String,
    template: String,
    node_locator: NodeLocator,
    context: Rc<Context>,
    schema: Weak<Schema>,
}

impl Node for MultiConfigNode {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn get_available_node_names(&self) -> Vec<NodeName> {
        let schema = self.schema.upgrade().expect("schema dropped");
        let mut names = vec![NodeName::Multiple(Rc::downgrade(
            schema
                .templates
                .get(&self.template)
                .expect("template not found"),
        ))];

        for (name, _) in &self.nodes {
            names.push(NodeName::Literal(name.to_owned()));
        }

        names
    }

    fn get_available_property_names(&self) -> Vec<String> {
        vec![]
    }

    fn get_node_with_name(&self, name: &str) -> &ConfigNode {
        match self.nodes.get(name) {
            Some(node) => node,
            _ => {
                unimplemented!();
                // let new_node = ConfigNode::from_schema_node(
                //     Rc::clone(&self.context),
                //     name,
                //     Weak::clone(&self.schema),
                //     self.schema
                //         .upgrade()
                //         .expect("schema dropped")
                //         .find_node(self.node_locator)
                //         .expect("schema node not found"),
                // )
                // .expect("failed to create new node");
                // new_node
            }
        }
    }

    fn get_property(&self, _property: &str) -> Option<&Property> {
        None
    }

    fn get_property_values(&self, _of_property: Option<String>) -> HashMap<String, Vec<String>> {
        HashMap::new()
    }

    fn set_property_value(
        &self,
        property: &str,
        value: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!();
    }

    fn pretty_print(&self, indent: usize) {
        for (name, node) in &self.nodes {
            println!("{:indent$}{} {{", "", name, indent = indent * 4);
            node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }
    }
}

impl FromSchemaNode<MultiSchemaNode> for MultiConfigNode {
    fn from_schema_node(
        context: Rc<Context>,
        name: &str,
        schema: Weak<Schema>,
        schema_node: &MultiSchemaNode,
    ) -> Result<Vec<ConfigNode>, Box<dyn std::error::Error>> {
        Ok(vec![MultiConfigNode {
            nodes: HashMap::new(),
            name: name.to_string(),
            template: schema_node.template.to_string(),
            context: Rc::clone(&context),
            node_locator: schema_node.get_locator(),
            schema,
        }
        .into()])
    }
}
