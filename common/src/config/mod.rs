mod node;
mod node_name;
mod property;

use super::{context::Context, schema::Schema};
pub use node::{ConfigNode, FromSchemaNode, Node};
pub use node_name::NodeName;
pub use property::Property;
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Config {
    pub nodes: HashMap<String, Rc<ConfigNode>>,
}

impl Config {
    pub fn from_schema(schema: Weak<Schema>) -> Result<Config, Box<dyn std::error::Error>> {
        let mut nodes = HashMap::new();
        let mut context = Context::new(None);
        context.set_value(String::from("mock"), String::from("mock"));
        let context_rc = Rc::new(context);

        if let Some(s) = schema.upgrade() {
            for (name, node) in &s.nodes {
                nodes.insert(
                    name.to_owned(),
                    Rc::new(ConfigNode::from_schema_node(
                        Rc::clone(&context_rc),
                        &name,
                        Weak::clone(&schema),
                        &node,
                    )?),
                );
            }
        }

        Ok(Config { nodes })
    }

    pub fn get_available_node_names(&self) -> Vec<NodeName> {
        let mut names = Vec::new();

        for name in self.nodes.keys() {
            names.push(NodeName::Literal(name.to_owned()));
        }

        names
    }

    pub fn get_node_with_name(&self, name: &str) -> Rc<ConfigNode> {
        match self.nodes.get(name) {
            Some(node) => Rc::clone(node),
            _ => panic!(),
        }
    }
}

impl Config {
    pub fn pretty_print(&self) {
        for (name, node) in &self.nodes {
            println!("{} {{", name);
            node.pretty_print(1);
            println!("}}");
        }
    }
}
