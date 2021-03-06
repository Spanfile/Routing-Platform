use super::{
    Changeable, ConfigNode, FromSchemaNode, Load, LoadSource, Node, NodeName, Property, Save,
    SaveBuilder,
};
use crate::{
    common::{helpers, Context},
    error,
    schema::{Schema, SingleSchemaNode},
};
use rp_log::*;
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct SingleConfigNode {
    name: String,
    subnodes: HashMap<String, Rc<ConfigNode>>,
    properties: HashMap<String, Property>,
}

impl Node for SingleConfigNode {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn get_available_node_names(&self) -> Vec<NodeName> {
        let mut names = Vec::new();

        for name in self.subnodes.keys() {
            names.push(NodeName::Literal(name.to_owned()));
        }

        names
    }

    fn get_available_property_names(&self) -> Vec<String> {
        self.properties.keys().map(|key| key.to_owned()).collect()
    }

    fn get_node_with_name(&self, name: &str) -> anyhow::Result<Option<Rc<ConfigNode>>> {
        Ok(self.subnodes.get(name).map(|n| Rc::clone(n)))
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
            node.pretty_print(indent + 1);
            println!("{:indent$}}}", "", indent = indent * 4);
        }

        for property in self.properties.values() {
            property.pretty_print(indent);
        }
    }

    fn remove_subnode(&self, node: &str) -> anyhow::Result<()> {
        Err(error::NodeRemovalError {
            node: String::from(node),
        }
        .into())
    }
}

impl Changeable for SingleConfigNode {
    fn is_clean(&self) -> bool {
        self.properties.values().all(|prop| prop.is_clean())
            && self.subnodes.values().all(|node| node.is_clean())
    }

    fn apply_changes(&self) -> anyhow::Result<bool> {
        let mut edits = false;

        for prop in self.properties.values() {
            edits = prop.apply_changes()? || edits;
        }

        for node in self.subnodes.values() {
            edits = node.apply_changes()? || edits;
        }

        Ok(edits)
    }

    fn discard_changes(&self) {
        for prop in self.properties.values() {
            prop.discard_changes();
        }

        for node in self.subnodes.values() {
            node.discard_changes();
        }
    }
}

impl Save for SingleConfigNode {
    fn save(&self, builder: &mut SaveBuilder) -> anyhow::Result<()> {
        for (name, node) in &self.subnodes {
            builder.begin_node(name.clone())?;
            node.save(builder)?;
            builder.end_node()?;
        }

        for (name, property) in &self.properties {
            for value in property.values() {
                builder.set_property(name.clone(), value)?;
            }
        }

        Ok(())
    }
}

impl Load for SingleConfigNode {
    fn load(&self, source: &mut LoadSource) -> anyhow::Result<()> {
        for (name, node) in &self.subnodes {
            source.begin_node(name)?;
            node.load(source)?;
            source.end_node()?;
        }

        for (name, property) in &self.properties {
            match source.get_property(name) {
                Ok(values) => {
                    let existing = property.values();
                    if helpers::equal_vecs(&existing, values) {
                        trace!("Loaded values equal to existing values in node '{}' property '{}', not loading ({:?})", name, self.name, values);
                    } else {
                        if !existing.is_empty() {
                            property.remove(None)?;
                        }

                        for value in values {
                            property.set(value)?;
                        }
                    }
                }
                Err(e) => {
                    if let Some(error::LoadError::NoSuchProperty(name)) = e.downcast_ref() {
                        trace!(
                            "Property '{}' in node '{}' could not be loaded (error: {})",
                            name,
                            self.name,
                            e
                        );
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }
}

impl FromSchemaNode<SingleSchemaNode> for SingleConfigNode {
    fn from_schema_node(
        context: Rc<Context>,
        name: &str,
        schema: Weak<Schema>,
        schema_node: &SingleSchemaNode,
    ) -> anyhow::Result<ConfigNode> {
        let name = context.format(name.to_owned())?;
        let mut subnodes = HashMap::new();
        let mut properties = HashMap::new();

        for (subname, subnode) in &schema_node.subnodes {
            subnodes.insert(
                subname.to_owned(),
                Rc::new(ConfigNode::from_schema_node(
                    Rc::clone(&context),
                    &subname,
                    Weak::clone(&schema),
                    &subnode,
                )?),
            );
        }

        for (key, property) in &schema_node.properties {
            let prop = Property::from_schema_property(
                Rc::clone(&context),
                &key,
                property,
                Weak::clone(&schema),
            )?;
            properties.insert(key.to_owned(), prop);
        }

        Ok(SingleConfigNode {
            name,
            subnodes,
            properties,
        }
        .into())
    }
}
