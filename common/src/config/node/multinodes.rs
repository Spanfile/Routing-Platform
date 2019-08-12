use super::Node;
use crate::Context;
use crate::config::NodeName;
use crate::schema::Schema;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Multinodes<'a> {
    nodes: HashMap<String, Box<Node<'a>>>,
    path: String,
    schema_node: &'a crate::schema::Multinode,
    schema: &'a Schema,
    context: Context<'a>,
}

impl<'a> Multinodes<'a> {
    pub fn from_schema_node(
        parent: &String,
        context: &'a Context<'a>,
        schema_node: &'a crate::schema::Multinode,
        schema: &'a Schema,
    ) -> Result<Multinodes<'a>, Box<dyn std::error::Error>> {
        Ok(Multinodes {
            nodes: HashMap::new(),
            path: parent.to_owned(),
            schema,
            schema_node: &schema_node,
            context: context.flatten(),
        })
    }

    pub fn get_available_node_names(&self) -> Vec<NodeName> {
        let mut names = vec!(NodeName::Multiple(self.schema.templates.get(&self.schema_node.template).unwrap()));

        for (name, _) in &self.nodes {
            names.push(NodeName::Literal(name.to_owned()));
        }

        names
    }

    pub fn get_node_with_name(&self, name: &String) -> &Node {
        match self.nodes.get(name) {
            Some(node) => return node,
            _ => {
                let _new_node = Node::from_schema_node(&self.path, &self.context, name, &self.schema_node.node, self.schema);
                panic!();
            }
        }
    }
}

impl<'a> Multinodes<'a> {
    pub fn pretty_print(&self, indent: usize) {
        for (name, node) in &self.nodes {
            println!("{:indent$}{} {{", "", name, indent = indent * 4);
            node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }
    }
}
