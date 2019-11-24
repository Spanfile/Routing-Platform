use super::{Changeable, ConfigNode, FromSchemaNode, Node, NodeName, Save, SaveBuilder};
use crate::Property;
use colored::Colorize;
use rp_common::Context;
use rp_schema::{MultiSchemaNode, MultiSchemaNodeSource, NodeLocator, Schema, SchemaNodeTrait};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug, PartialEq, Clone)]
enum NodeChange {
    Unchanged,
    New,
    Removed,
}

#[derive(Debug)]
pub struct MultiConfigNode {
    nodes: RefCell<HashMap<String, (Rc<ConfigNode>, NodeChange)>>,
    name: String,
    new_node_creation_allowed: NewNodeCreationAllowed,
    node_locator: Rc<NodeLocator>,
    context: Rc<Context>,
    schema: Weak<Schema>,
}

#[derive(Debug)]
enum NewNodeCreationAllowed {
    No,
    Yes { template: String },
}

impl Node for MultiConfigNode {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn get_available_node_names(&self) -> Vec<NodeName> {
        let schema = self.schema.upgrade().expect("schema dropped");
        let mut names = vec![];

        if let NewNodeCreationAllowed::Yes { template } = &self.new_node_creation_allowed {
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
            Some((node, _)) => Rc::clone(&node),
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

                nodes.insert(name.to_owned(), (Rc::clone(&new_node), NodeChange::New));

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

    fn pretty_print(&self, indent: usize) {
        for (name, (node, change)) in &*self.nodes.borrow() {
            let (name, left_brace, right_brace) = match change {
                NodeChange::Unchanged => (name.normal(), "{".normal(), "}".normal()),
                NodeChange::New => (["+", name].concat().green(), "{".green(), "}".green()),
                NodeChange::Removed => (["-", name].concat().red(), "{".red(), "}".red()),
            };

            println!("{:indent$}{} {}", "", name, left_brace, indent = indent * 4);
            node.pretty_print(indent + 1);
            println!("{:indent$}{}", "", right_brace, indent = indent * 4);
        }
    }

    fn remove_subnode(&self, node: &str) -> anyhow::Result<()> {
        // if new nodes aren't allowed to be created, existing ones can't be removed
        // either
        match self.new_node_creation_allowed {
            NewNodeCreationAllowed::Yes { .. } => {
                let mut nodes = self.nodes.try_borrow_mut()?;
                let (_node, change) =
                    nodes
                        .get_mut(node)
                        .ok_or(rp_common::error::NodeRemovalError {
                            node: String::from(node),
                        })?;
                // TODO: handle case when removing non-unchanged node
                *change = NodeChange::Removed;
                Ok(())
            }
            NewNodeCreationAllowed::No => Err(rp_common::error::NodeRemovalError {
                node: String::from(node),
            }
            .into()),
        }
    }
}

impl Changeable for MultiConfigNode {
    fn is_clean(&self) -> bool {
        self.nodes
            .borrow()
            .values()
            .all(|(node, change)| node.is_clean() && *change == NodeChange::Unchanged)
    }

    fn apply_changes(&self) -> anyhow::Result<()> {
        let new_nodes: HashMap<String, (Rc<ConfigNode>, NodeChange)> = match self
            .nodes
            .try_borrow()?
            .iter()
            .filter_map(|(name, (node, change))| match change {
                NodeChange::Unchanged | NodeChange::New => {
                    if let Err(e) = node.apply_changes() {
                        Some(Err(e))
                    } else {
                        Some(Ok((name.clone(), (Rc::clone(node), NodeChange::Unchanged))))
                    }
                }
                NodeChange::Removed => None,
            })
            .collect()
        {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        self.nodes.replace(new_nodes);

        Ok(())
    }

    fn discard_changes(&self) {
        let new_nodes: HashMap<String, (Rc<ConfigNode>, NodeChange)> = self
            .nodes
            .borrow()
            .iter()
            .filter_map(|(name, (node, change))| match change {
                NodeChange::New => None,
                NodeChange::Removed | NodeChange::Unchanged => {
                    Some((name.clone(), (Rc::clone(node), NodeChange::Unchanged)))
                }
            })
            .collect();

        self.nodes.replace(new_nodes);
    }
}

impl Save for MultiConfigNode {
    fn save(&self, builder: &mut SaveBuilder) -> anyhow::Result<()> {
        for (name, (node, _)) in self.nodes.try_borrow()?.iter() {
            builder.begin_node(name.clone())?;
            node.save(builder)?;
            builder.end_node()?;
        }

        Ok(())
    }
}

impl FromSchemaNode<MultiSchemaNode> for MultiConfigNode {
    fn from_schema_node(
        context: Rc<Context>,
        name: &str,
        schema: Weak<Schema>,
        schema_node: &MultiSchemaNode,
    ) -> anyhow::Result<ConfigNode> {
        let mut nodes = HashMap::new();

        let new_node_creation_allowed = match &schema_node.source {
            MultiSchemaNodeSource::Query(query) => {
                for result in &query.run(&context)? {
                    let mut result_context = Context::new(Some(Rc::clone(&context)));
                    result_context.set_value(query.id.to_owned(), result.to_owned());
                    nodes.insert(
                        result.to_owned(),
                        (
                            Rc::new(ConfigNode::from_schema_node(
                                Rc::new(result_context),
                                &result.to_owned(),
                                Weak::clone(&schema),
                                &schema_node.node,
                            )?),
                            NodeChange::Unchanged,
                        ),
                    );
                }
                NewNodeCreationAllowed::No
            }
            MultiSchemaNodeSource::Template(template) => NewNodeCreationAllowed::Yes {
                template: template.to_owned(),
            },
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
