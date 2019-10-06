use super::{ConfigNode, FromSchemaNode, Node, NodeName, SingleConfigNode};
use crate::{
    config::Property,
    schema::{MultiSchemaNode, NodeLocator},
    Context,
};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct MultiConfigNode {
    nodes: HashMap<String, Box<SingleConfigNode>>,
    name: String,
    path: String,
    template: String,
    node_locator: NodeLocator,
    context: Rc<Context>,
}

impl Node for MultiConfigNode {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn full_path(&self) -> String {
        [self.path.as_str(), self.name.as_str()].join(".")
    }

    fn get_available_node_names(&self) -> Vec<NodeName> {
        unimplemented!();
        // let mut names =
        // vec![NodeName::Multiple(
        //     self.schema
        //         .templates
        //         .get(&self.schema_node.template)
        //         .unwrap(),
        // )];

        // for (name, _) in &self.nodes {
        //     names.push(NodeName::Literal(name.to_owned()));
        // }

        // names
    }

    fn get_available_property_names(&self) -> Vec<String> {
        vec![]
    }

    fn get_node_with_name(&self, name: &str) -> &ConfigNode {
        unimplemented!();
        // match self.nodes.get(name) {
        //     Some(node) => return node,
        //     _ => {
        //         let _new_node = Node::from_schema_node(&self.path,
        // &self.context, name, &self.schema_node.node, self.schema);
        // panic!();}
        // }
    }

    fn get_property(&self, property: &str) -> Option<&Property> {
        None
    }

    fn get_property_values(&self, of_property: Option<String>) -> HashMap<String, Vec<String>> {
        HashMap::new()
    }

    fn pretty_print(&self, indent: usize) {
        for (name, node) in &self.nodes {
            println!("{:indent$}{} {{", "", name, indent = indent * 4);
            // node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }
    }
}

impl FromSchemaNode<MultiSchemaNode> for MultiConfigNode {
    fn from_schema_node(
        parent: &str,
        context: Rc<Context>,
        name: &str,
        schema_node: &MultiSchemaNode,
    ) -> Result<Vec<MultiConfigNode>, Box<dyn std::error::Error>> {
        Ok(vec![MultiConfigNode {
            nodes: HashMap::new(),
            name: name.to_string(),
            path: parent.to_owned(),
            template: schema_node.template.to_string(),
            context: Rc::clone(&context),
            node_locator: schema_node.get_locator(),
        }])
    }
}
