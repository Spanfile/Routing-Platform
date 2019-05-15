use common::config::{node::Node, Config};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ConfigEditor<'a> {
    config: &'a Config,
    node_stack: Vec<&'a Node>,
}

#[derive(Debug)]
pub enum ConfigEditorError {
    NodeNotFound(String),
    PropertyNotFound(String),
    NoParentNode,
}

impl std::fmt::Display for ConfigEditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            ConfigEditorError::NodeNotFound(_name) => write!(f, "node not found"),
            ConfigEditorError::PropertyNotFound(_name) => write!(f, "property not found"),
            ConfigEditorError::NoParentNode => write!(f, "no parent node"),
        }
    }
}

impl std::error::Error for ConfigEditorError {}

impl<'a> ConfigEditor<'a> {
    pub fn new(config: &'a Config) -> ConfigEditor<'a> {
        ConfigEditor {
            config,
            node_stack: Vec::new(),
        }
    }

    pub fn get_available_nodes(&self) -> Vec<&String> {
        match self.node_stack.last() {
            Some(n) => n.subnodes.keys().collect(),
            None => self.config.nodes.keys().collect(),
        }
    }

    pub fn get_available_properties(&self) -> Vec<&String> {
        match self.node_stack.last() {
            Some(n) => n.properties.keys().collect(),
            None => vec![],
        }
    }

    pub fn get_available_nodes_and_properties(&self) -> Vec<&String> {
        let mut names = self.get_available_nodes();
        names.extend(self.get_available_properties());
        names
    }

    pub fn edit_node(&mut self, name: String) -> Result<(), ConfigEditorError> {
        match match self.node_stack.last() {
            Some(n) => &n.subnodes,
            None => &self.config.nodes,
        }
        .get(&name)
        {
            Some(node) => {
                self.node_stack.push(node);
                Ok(())
            }
            None => Err(ConfigEditorError::NodeNotFound(name)),
        }
    }

    pub fn go_up(&mut self) -> Result<(), ConfigEditorError> {
        match self.node_stack.last() {
            Some(_n) => {
                self.node_stack.pop();
                Ok(())
            }
            None => Err(ConfigEditorError::NoParentNode),
        }
    }

    pub fn get_property_values(
        &self,
        of_property: Option<String>,
    ) -> HashMap<&String, Vec<String>> {
        match self.node_stack.last() {
            Some(n) => n
                .properties
                .iter()
                .filter(|(key, _p)| match &of_property {
                    Some(prop) => *key == prop,
                    None => true,
                })
                .map(|(key, property)| {
                    (
                        key,
                        property
                            .values
                            .borrow()
                            .iter()
                            .map(|v| v.to_owned()) // maybe not the greatest idea to clone the values?
                            .collect(),
                    )
                })
                .collect(),
            None => HashMap::new(),
        }
    }

    pub fn set_property_value(
        &self,
        property: String,
        value: String,
    ) -> Result<(), ConfigEditorError> {
        let property = match self.node_stack.last() {
            Some(n) => match n.properties.get(&property) {
                Some(p) => p,
                None => return Err(ConfigEditorError::PropertyNotFound(property)),
            },
            None => return Err(ConfigEditorError::PropertyNotFound(property)),
        };

        // TODO: make sure value matches some value constraint

        let mut values = property.values.borrow_mut();
        if !property.constraints.multiple {
            values.clear();
        }
        values.push(value);

        Ok(())
    }
}
