use super::{ConfigNode, FromSchemaNode, Node, NodeName};
use crate::{
    config::Property,
    schema::{MultiSchemaNode, MultiSchemaNodeSource, NodeLocator, Schema, SchemaNodeTrait},
    Context,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct MultiConfigNode {
    nodes: RefCell<HashMap<String, Rc<ConfigNode>>>,
    name: String,
    new_node_creation_allowed: NewNodeCreationAllowed,
    node_locator: Rc<NodeLocator>,
    context: Rc<Context>,
    schema: Weak<Schema>,
}

#[derive(Debug)]
enum NewNodeCreationAllowed {
    No,
    Yes(String),
}

impl Node for MultiConfigNode {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn get_available_node_names(&self) -> Vec<NodeName> {
        let schema = self.schema.upgrade().expect("schema dropped");
        let mut names = vec![];

        if let NewNodeCreationAllowed::Yes(template) = &self.new_node_creation_allowed {
            names.push(NodeName::Multiple(Rc::downgrade(
                schema.templates.get(template).expect("template not found"),
            )));
        }

        for name in self.nodes.borrow().keys() {
            names.push(NodeName::Literal(name.to_owned()));
        }

        names
    }

    fn get_available_property_names(&self) -> Vec<String> {
        vec![]
    }

    fn get_node_with_name(&self, name: &str) -> Rc<ConfigNode> {
        let mut nodes = self.nodes.borrow_mut();
        match nodes.get(name) {
            Some(node) => Rc::clone(&node),
            _ => {
                let new_node = Rc::new(
                    ConfigNode::from_schema_node(
                        Rc::clone(&self.context),
                        name,
                        Weak::clone(&self.schema),
                        self.schema
                            .upgrade()
                            .expect("schema dropped")
                            .find_node(Rc::clone(&self.node_locator))
                            .expect("schema node not found"),
                    )
                    .expect("failed to create new node"),
                );

                nodes.insert(name.to_owned(), Rc::clone(&new_node));

                new_node
            }
        }
    }

    fn get_property(&self, _property: &str) -> Option<&Property> {
        None
    }

    fn get_property_values(&self, _of_property: Option<String>) -> HashMap<String, Vec<String>> {
        HashMap::new()
    }

    fn set_property_value(
        &self,
        property: &str,
        value: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!();
    }

    fn pretty_print(&self, indent: usize) {
        for (name, node) in &*self.nodes.borrow() {
            println!("{:indent$}{} {{", "", name, indent = indent * 4);
            node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }
    }
}

impl FromSchemaNode<MultiSchemaNode> for MultiConfigNode {
    fn from_schema_node(
        context: Rc<Context>,
        name: &str,
        schema: Weak<Schema>,
        schema_node: &MultiSchemaNode,
    ) -> Result<ConfigNode, Box<dyn std::error::Error>> {
        let mut nodes = HashMap::new();

        let new_node_creation_allowed = match &schema_node.source {
            MultiSchemaNodeSource::Query(query) => {
                for result in &query.run(&context)? {
                    let mut result_context = Context::new(Some(Rc::clone(&context)));
                    result_context.set_value(query.id.to_owned(), result.to_owned());
                    nodes.insert(
                        result.to_owned(),
                        Rc::new(ConfigNode::from_schema_node(
                            Rc::new(result_context),
                            &result.to_owned(),
                            Weak::clone(&schema),
                            &schema_node.node,
                        )?),
                    );
                }
                NewNodeCreationAllowed::No
            }
            MultiSchemaNodeSource::Template(template) => {
                NewNodeCreationAllowed::Yes(template.to_owned())
            }
        };

        Ok(MultiConfigNode {
            nodes: RefCell::new(nodes),
            name: name.to_owned(),
            new_node_creation_allowed,
            context: Rc::clone(&context),
            node_locator: schema_node.node.get_locator(),
            schema,
        }
        .into())
    }
}
