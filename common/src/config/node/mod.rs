mod node_data;

use crate::context::Context;
use node_data::NodeData;

#[derive(Debug)]
pub enum Node {
    Static(NodeData),
    Multi {
        base: NodeData,
        nodes: Vec<NodeData>,
    },
}

#[derive(Debug)]
pub enum NodeName {
    Literal(String),
    Template(String),
}

impl Node {
    pub fn full_path(&self) -> String {
        match self {
            Node::Static(data) => data.full_path(),
            Node::Multi { base, nodes } => base.full_path(),
        }
    }

    pub fn from_schema_node(
        parent: &String,
        context: &Context,
        name: &String,
        node: &crate::schema::node::Node,
    ) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        match &node.query {
            Some(query) => {
                let results = query.run(context)?;
                let mut nodes = Vec::new();

                for result in &results {
                    let mut result_context = Context::new(Some(context));
                    result_context.set_value(query.id.to_owned(), result.to_owned());
                    nodes.push(Node::Static(NodeData::new(
                        parent,
                        &result_context,
                        name,
                        node,
                    )?));
                }

                Ok(nodes)
            }
            None => Ok(vec![Node::Static(NodeData::new(
                parent, &context, name, node,
            )?)]),
        }
    }

    pub fn name(&self) -> &String {
        match self {
            Node::Static(data) => &data.name,
            Node::Multi(data) => &data.name,
        }
    }

    pub fn get_available_names(&self) -> Vec<NodeName> {
        match self {
            Node::Static(data) => vec![NodeName::Literal(data.name.to_owned())],
            Node::Multi(data) => {
                let mut names = vec![NodeName::Template(data.name.to_owned())];
                names
            }
        }
    }
}

impl Node {
    pub fn pretty_print(&self, indent: usize) {
        match self {
            Node::Static(data) => data.pretty_print(indent),
            Node::Multi(data) => data.pretty_print(indent),
        }
    }
}
