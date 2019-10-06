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
    parent: Option<Weak<ConfigNode>>,
    template: String,
    node_locator: NodeLocator,
    context: Rc<Context>,
    schema: Weak<Schema>,
}

impl Node for MultiConfigNode {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn full_path(&self) -> String {
        [
            if let Some(parent) = &self.parent {
                parent.upgrade().expect("parent dropped").full_path()
            } else {
                String::from("")
            },
            self.name.to_owned(),
        ]
        .join(".")
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
                println!("{:?}", self.node_locator);
                // let new_node = ConfigNode::from_schema_node(
                //     &self.path,
                //     Rc::clone(&self.context),
                //     name,
                //     &self.schema_node.node,
                //     self.schema,
                // );
                panic!();
            }
        }
    }

    fn get_property(&self, _property: &str) -> Option<&Property> {
        None
    }

    fn get_property_values(&self, _of_property: Option<String>) -> HashMap<String, Vec<String>> {
        HashMap::new()
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
        parent: Option<Weak<ConfigNode>>,
        context: Rc<Context>,
        name: &str,
        schema: Weak<Schema>,
        schema_node: &MultiSchemaNode,
    ) -> Result<Vec<ConfigNode>, Box<dyn std::error::Error>> {
        Ok(vec![MultiConfigNode {
            nodes: HashMap::new(),
            name: name.to_string(),
            parent,
            template: schema_node.template.to_string(),
            context: Rc::clone(&context),
            node_locator: schema_node.get_locator(),
            schema,
        }
        .into()])
    }
}
