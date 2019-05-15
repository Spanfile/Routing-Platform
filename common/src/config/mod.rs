mod node;
mod property;

use super::context::Context;
use super::schema::Schema;
use node::Node;

#[derive(Debug)]
pub struct Config {
    pub nodes: Vec<Node>,
}

impl Config {
    pub fn from_schema(schema: &Schema) -> Result<Config, Box<dyn std::error::Error>> {
        let mut nodes = Vec::new();
        let mut context = Context::new(None);
        context.set_value(String::from("mock"), String::from("mock"));

        for node in &schema.nodes {
            nodes.extend(Node::from_schema_node(
                &String::from("config"),
                &context,
                node,
            )?);
        }

        Ok(Config { nodes })
    }
}
