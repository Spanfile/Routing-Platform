use super::Node;
use crate::config::NodeName;
use crate::schema::NodeLocator;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Multinodes {
    nodes: HashMap<String, Box<Node>>,
    path: String,
    node_locator: NodeLocator,
}

impl Multinodes {
    pub fn from_schema_node(
        parent: &str,
        // context: &'a Context,
        schema_multinode: &crate::schema::Multinode,
    ) -> Result<Multinodes, Box<dyn std::error::Error>> {
        Ok(Multinodes {
            nodes: HashMap::new(),
            path: parent.to_owned(),
            // schema,
            // schema_node,
            // context: context.flatten(),
            node_locator: schema_multinode.get_node_locator(),
        })
    }

    pub fn get_available_node_names(&self) -> Vec<NodeName> {
        panic!();
        // let mut names = vec!(NodeName::Multiple(self.schema.templates.get(&self.schema_node.template).unwrap()));

        // for (name, _) in &self.nodes {
        //     names.push(NodeName::Literal(name.to_owned()));
        // }

        // names
    }

    pub fn get_node_with_name(&self, _name: &str) -> &Node {
        panic!();
        // match self.nodes.get(name) {
        //     Some(node) => return node,
        //     _ => {
        //         let _new_node = Node::from_schema_node(&self.path, &self.context, name, &self.schema_node.node, self.schema);
        //         panic!();
        //     }
        // }
    }
}

impl Multinodes {
    pub fn pretty_print(&self, indent: usize) {
        for (name, node) in &self.nodes {
            println!("{:indent$}{} {{", "", name, indent = indent * 4);
            node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }
    }
}
