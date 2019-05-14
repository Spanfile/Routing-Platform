use super::property::Property;

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

    pub fn from_schema_node(parent: &String, node: &crate::schema::node::Node) -> Vec<Node> {
        match &node.query {
            Some(query) => Node::build_multiple_nodes(
                parent,
                node,
                &query.run().expect(&format!(
                    "couldn't run query to get node names for schema node {:?} (in parent: {})",
                    node, parent
                )),
            ),
            None => vec![Node::build_one_node(parent, node)],
        }
    }

    fn build_one_node(parent: &String, node: &crate::schema::node::Node) -> Node {
        let name = node.name.clone();
        let path = String::from([parent.as_str(), name.as_str()].join("."));
        let mut subnodes = Vec::new();
        let mut properties = Vec::new();

        for subnode in &node.subnodes {
            subnodes.extend(
                Node::from_schema_node(&path, subnode)
                    .into_iter()
                    .map(|n| Box::new(n)),
            );
        }

        for property in &node.properties {
            properties.push(Property::from_schema_property(&path, property));
        }

        Node {
            name,
            path: parent.clone(),
            subnodes,
            properties,
        }
    }

    fn build_multiple_nodes(
        parent: &String,
        node: &crate::schema::node::Node,
        names: &Vec<String>,
    ) -> Vec<Node> {
        vec![]
    }
}
