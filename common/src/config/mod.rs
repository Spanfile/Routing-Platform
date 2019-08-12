mod node;
mod node_name;
mod property;

use super::context::Context;
use super::schema::Schema;
pub use node::Node;
pub use node_name::NodeName;
pub use property::Property;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Config<'a> {
    // it helps a ton down the line if this has the exact same type as a node's subnodes hashmap
    pub nodes: HashMap<String, Box<Node<'a>>>,
}

impl<'a> Config<'a> {
    pub fn from_schema(schema: &Schema) -> Result<Config, Box<dyn std::error::Error>> {
        let mut nodes = HashMap::new();
        let mut context = Context::new(None);
        context.set_value(String::from("mock"), String::from("mock"));

        for (name, node) in &schema.nodes {
            nodes.extend(
                Node::from_schema_node(&String::from("config"), &context, name, node, schema)?
                    .into_iter()
                    .map(|n| (n.name.to_owned(), Box::new(n))),
            );
        }

        Ok(Config { nodes })
    }

    pub fn get_available_node_names(&self) -> Vec<NodeName> {
        let mut names = Vec::new();

        for (name, _) in &self.nodes {
            names.push(NodeName::Literal(name.to_owned()));
        }

        names
    }

    pub fn get_node_with_name(&self, name: &String) -> &Node {
        match self.nodes.get(name) {
            Some(node) => return &node,
            _ => 
                panic!(),
        }
    }
}

impl<'a> Config<'a> {
    pub fn pretty_print(&self) {
        for (name, node) in &self.nodes {
            println!("{} {{", name);
            node.pretty_print(1);
            println!("}}");
        }
    }
}
