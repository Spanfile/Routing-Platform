mod node;
mod property;

use super::schema::Schema;
use node::Node;

#[derive(Debug)]
pub struct Config {
    pub nodes: Vec<Node>,
}

impl Config {
    pub fn from_schema(schema: &Schema) -> Config {
        let mut nodes = Vec::new();

        for node in &schema.nodes {
            nodes.push(Node::from_schema_node(&String::from("config"), node));
        }

        Config { nodes }
    }
}
