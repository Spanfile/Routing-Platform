use super::Node;
use crate::Context;
use crate::config::NodeName;
use crate::schema::Schema;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Multinodes<'a, 'b> {
    nodes: HashMap<String, Box<Node<'a, 'b>>>,
    path: String,
    schema_node: &'a crate::schema::Multinode,
    schema: &'a Schema,
    context: Context<'b>,
}

impl<'a, 'b> Multinodes<'a, 'b> {
    pub fn from_schema_node(
        parent: &String,
        context: &'b Context<'b>,
        schema_node: &'a crate::schema::Multinode,
        schema: &'a Schema,
    ) -> Result<Multinodes<'a, 'b>, Box<dyn std::error::Error>> {
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

impl<'a, 'b> Multinodes<'a, 'b> {
    pub fn pretty_print(&self, indent: usize) {
        for (name, node) in &self.nodes {
            println!("{:indent$}{} {{", "", name, indent = indent * 4);
            node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }
    }
}
