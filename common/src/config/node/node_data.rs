use super::Node;
use crate::config::property::Property;
use crate::context::Context;
use std::collections::HashMap;

#[derive(Debug)]
pub struct NodeData {
    pub name: String,
    pub path: String,
    pub subnodes: HashMap<String, Box<Node>>,
    pub properties: HashMap<String, Property>,
}

impl NodeData {
    pub fn new(
        parent: &String,
        context: &Context,
        name: &String,
        node: &crate::schema::node::Node,
    ) -> Result<NodeData, Box<dyn std::error::Error>> {
        let name = context
            .format(name.to_owned())
            .expect("couldn't context format node name");
        let path = String::from([parent.as_str(), name.as_str()].join("."));
        let mut subnodes = HashMap::new();
        let mut properties = HashMap::new();

        for (subname, subnode) in &node.subnodes {
            subnodes.extend(
                Node::from_schema_node(&path, context, &subname, subnode)?
                    .into_iter()
                    .map(|n| (n.name().to_owned(), Box::new(n))),
            );
        }

        for (key, property) in &node.properties {
            let prop = Property::from_schema_property(&path, context, &key, property)?;
            properties.insert(key.to_owned(), prop);
        }

        Ok(NodeData {
            name,
            path: parent.to_owned(),
            subnodes,
            properties,
        })
    }

    pub fn full_path(&self) -> String {
        String::from([self.path.as_str(), self.name.as_str()].join("."))
    }
}

impl NodeData {
    pub fn pretty_print(&self, indent: usize) {
        for (name, node) in &self.subnodes {
            println!("{:indent$}{} {{", "", name, indent = indent * 4);
            node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }

        for property in self.properties.values() {
            property.pretty_print(indent);
        }
    }
}
