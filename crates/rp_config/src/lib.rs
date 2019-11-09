mod changeable;
pub mod error;
mod node;
mod node_name;
mod property;

use anyhow::anyhow;
pub use changeable::Changeable;
pub use node::{ConfigNode, FromSchemaNode, Node};
pub use node_name::NodeName;
pub use property::Property;
use rp_common::Context;
use rp_schema::Schema;
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Config {
    pub nodes: HashMap<String, Rc<ConfigNode>>,
}

impl Changeable for Config {
    fn is_clean(&self) -> bool {
        self.nodes.values().all(|node| node.is_clean())
    }

    fn apply_changes(&self) -> anyhow::Result<()> {
        unimplemented!()
    }
}

impl Config {
    pub fn from_schema(schema: Weak<Schema>) -> anyhow::Result<Config> {
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
        } else {
            return Err(anyhow!("Schema weak reference upgrading failed"));
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
