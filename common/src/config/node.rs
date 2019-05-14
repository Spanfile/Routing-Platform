use super::property::Property;
use crate::context::Context;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub path: String,
    pub subnodes: Vec<Box<Node>>,
    pub properties: Vec<Property>,
}

impl Node {
    pub fn full_path(&self) -> String {
        String::from([self.path.as_str(), self.name.as_str()].join("."))
    }

    pub fn from_schema_node(
        parent: &String,
        context: &Context,
        node: &crate::schema::node::Node,
    ) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        match &node.query {
            Some(query) => {
                let results = query.run(context)?;
                let mut nodes = Vec::new();

                for result in &results {
                    let mut result_context = Context::new(Some(context));
                    result_context.set_value(query.id.to_owned(), result.to_owned());
                    nodes.push(Node::build_node(parent, &result_context, node)?);
                }

                Ok(nodes)
            }
            None => Ok(vec![Node::build_node(parent, &context, node)?]),
        }
    }

    fn build_node(
        parent: &String,
        context: &Context,
        node: &crate::schema::node::Node,
    ) -> Result<Node, Box<dyn std::error::Error>> {
        let name = context
            .format(node.name.to_owned())
            .expect("couldn't context format node name");
        let path = String::from([parent.as_str(), name.as_str()].join("."));
        let mut subnodes = Vec::new();
        let mut properties = Vec::new();

        for subnode in &node.subnodes {
            subnodes.extend(
                Node::from_schema_node(&path, context, subnode)?
                    .into_iter()
                    .map(|n| Box::new(n)),
            );
        }

        for property in &node.properties {
            properties.push(Property::from_schema_property(&path, context, property)?);
        }

        Ok(Node {
            name,
            path: parent.to_owned(),
            subnodes,
            properties,
        })
    }
}
