mod multinodes;

use super::property::Property;
use super::NodeName;
use crate::Context;
use multinodes::Multinodes;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub path: String,
    pub subnodes: HashMap<String, Box<Node>>,
    pub multinodes: Option<Multinodes>,
    pub properties: HashMap<String, Property>,
}

impl Node {
    pub fn full_path(&self) -> String {
        String::from([self.path.as_str(), self.name.as_str()].join("."))
    }

    pub fn from_schema_node(
        parent: &String,
        context: &Context,
        name: &String,
        node: &crate::schema::Node,
    ) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        match &node.query {
            Some(query) => {
                let mut nodes = Vec::new();

                for result in &query.run(context)? {
                    let mut result_context = Context::new(Some(context));
                    result_context.set_value(query.id.to_owned(), result.to_owned());
                    nodes.push(Node::build_node(parent, &result_context, name, node)?);
                }

                Ok(nodes)
            }
            None => Ok(vec![Node::build_node(parent, &context, name, node)?]),
        }
    }

    fn build_node(
        parent: &String,
        context: & Context,
        name: &String,
        schema_node: & crate::schema::Node,
    ) -> Result<Node, Box<dyn std::error::Error>> {
        let name = context
            .format(name.to_owned())
            .expect("couldn't context format node name");
        let path = String::from([parent.as_str(), name.as_str()].join("."));
        let mut subnodes = HashMap::new();
        let mut properties = HashMap::new();

        for (subname, subnode) in &schema_node.subnodes {
            subnodes.extend(
                Node::from_schema_node(&path, context, &subname, subnode)?
                    .into_iter()
                    .map(|n| (n.name.to_owned(), Box::new(n))),
            );
        }

        for (key, property) in &schema_node.properties {
            let prop = Property::from_schema_property(&path, context, &key, property)?;
            properties.insert(key.to_owned(), prop);
        }

        let multinodes = if let Some(multinode) = &schema_node.multinode {
            Some(Multinodes::from_schema_node(&path, /* context, */ multinode)?)
        } else {
            None
        };

        Ok(Node {
            name,
            path: parent.to_owned(),
            subnodes,
            properties,
            multinodes,
        })
    }

    pub fn get_available_node_names(&self) -> Vec<NodeName> {
        let mut names = Vec::new();

        for (name, _) in &self.subnodes {
            names.push(NodeName::Literal(name.to_owned()));
        }

        if let Some(multinodes) = &self.multinodes {
            names.extend(multinodes.get_available_node_names());
        }

        names
    }

    pub fn get_node_with_name(&self, name: &String) -> &Node {
        match self.subnodes.get(name) {
            Some(node) => return &node,
            _ => 
                if let Some(multinodes) = &self.multinodes {
                    multinodes.get_node_with_name(name)
                } else {
                    panic!();
                }
        }
    }
}

impl Node {
    pub fn pretty_print(&self, indent: usize) {
        for (name, node) in &self.subnodes {
            println!("{:indent$}{} {{", "", name, indent = indent * 4);
            node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }

        if let Some(multinodes) = &self.multinodes {
            multinodes.pretty_print(indent);
        }

        for property in self.properties.values() {
            property.pretty_print(indent);
        }
    }
}
