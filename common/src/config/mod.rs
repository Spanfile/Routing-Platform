pub mod node;
pub mod property;

use super::context::Context;
use super::schema::Schema;
use node::Node;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    // it helps a ton down the line if this has the exact same type as a node's subnodes hashmap
    pub nodes: HashMap<String, Box<Node>>,
}

impl Config {
    pub fn from_schema(schema: &Schema) -> Result<Config, Box<dyn std::error::Error>> {
        let mut nodes = HashMap::new();
        let mut context = Context::new(None);
        context.set_value(String::from("mock"), String::from("mock"));

        for node in &schema.nodes {
            nodes.extend(
                Node::from_schema_node(&String::from("config"), &context, node)?
                    .into_iter()
                    .map(|n| (n.name.to_owned(), Box::new(n))),
            );
        }

        Ok(Config { nodes })
    }
}
