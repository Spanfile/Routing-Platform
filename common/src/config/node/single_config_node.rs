use super::{ConfigNode, FromSchemaNode, Node, NodeName};
use crate::{config::Property, schema::SingleSchemaNode, Context};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct SingleConfigNode {
    name: String,
    path: String,
    subnodes: HashMap<String, Box<ConfigNode>>,
    properties: HashMap<String, Property>,
}

impl Node for SingleConfigNode {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn full_path(&self) -> String {
        [self.path.as_str(), self.name.as_str()].join(".")
    }

    fn get_available_node_names(&self) -> Vec<NodeName> {
        let mut names = Vec::new();

        for name in self.subnodes.keys() {
            names.push(NodeName::Literal(name.to_owned()));
        }

        // if let Some(multinodes) = &self.multinodes {
        //     names.extend(multinodes.get_available_node_names());
        // }

        names
    }

    fn get_available_property_names(&self) -> Vec<String> {
        self.properties.keys().map(|key| key.to_owned()).collect()
    }

    fn get_node_with_name(&self, name: &str) -> &ConfigNode {
        match self.subnodes.get(name) {
            Some(node) => &node,
            _ => {
                // if let Some(multinodes) = &self.multinodes {
                //     multinodes.get_node_with_name(name)
                // } else {
                //     panic!();
                // }
                panic!();
            }
        }
    }

    fn get_property(&self, property: &str) -> Option<&Property> {
        self.properties.get(property)
    }

    fn get_property_values(&self, of_property: Option<String>) -> HashMap<String, Vec<String>> {
        self.properties
            .iter()
            .filter(|(key, _p)| match &of_property {
                Some(prop) => *key == prop,
                None => true,
            })
            .map(|(key, property)| {
                (
                    key.to_owned(),
                    property
                        .values()
                        .iter()
                        .map(|v| v.to_owned()) // maybe not the greatest idea to clone the values?
                        .collect(),
                )
            })
            .collect()
    }

    fn pretty_print(&self, indent: usize) {
        for (name, node) in &self.subnodes {
            println!("{:indent$}{} {{", "", name, indent = indent * 4);
            // node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }

        // if let Some(multinodes) = &self.multinodes {
        //     multinodes.pretty_print(indent);
        // }

        for property in self.properties.values() {
            property.pretty_print(indent);
        }
    }
}

impl FromSchemaNode<SingleSchemaNode> for SingleConfigNode {
    fn from_schema_node(
        parent: &str,
        context: Rc<Context>,
        name: &str,
        schema_node: &SingleSchemaNode,
    ) -> Result<Vec<SingleConfigNode>, Box<dyn std::error::Error>> {
        match &schema_node.query {
            Some(query) => {
                let mut nodes = Vec::new();

                for result in &query.run(&context)? {
                    let mut result_context = Context::new(Some(Rc::clone(&context)));
                    result_context.set_value(query.id.to_owned(), result.to_owned());
                    nodes.push(SingleConfigNode::build_node(
                        parent,
                        Rc::new(result_context),
                        name,
                        schema_node,
                    )?);
                }

                Ok(nodes)
            }
            None => Ok(vec![SingleConfigNode::build_node(
                parent,
                context,
                name,
                schema_node,
            )?]),
        }
    }
}

impl SingleConfigNode {
    fn build_node(
        parent: &str,
        context: Rc<Context>,
        name: &str,
        schema_node: &SingleSchemaNode,
    ) -> Result<SingleConfigNode, Box<dyn std::error::Error>> {
        let name = context
            .format(name.to_owned())
            .expect("couldn't context format node name");
        let path = [parent.to_owned(), name.to_owned()].join(".");
        let mut subnodes = HashMap::new();
        let mut properties = HashMap::new();

        for (subname, subnode) in &schema_node.subnodes {
            subnodes.extend(
                ConfigNode::from_schema_node(&path, Rc::clone(&context), &subname, &subnode)?
                    .into_iter()
                    .map(|n| (n.name().to_owned(), Box::new(n))),
            );
        }

        for (key, property) in &schema_node.properties {
            let prop = Property::from_schema_property(&path, Rc::clone(&context), &key, property)?;
            properties.insert(key.to_owned(), prop);
        }

        // let multinodes = if let Some(multinode) = &schema_node.multinode {
        //     Some(Multinodes::from_schema_node(&path, context, multinode)?)
        // } else {
        //     None
        // };

        Ok(SingleConfigNode {
            name,
            path: parent.to_owned(),
            subnodes,
            properties,
            // multinodes,
        })
    }
}
