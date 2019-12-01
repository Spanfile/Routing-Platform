use crate::error;
use rp_config::{Changeable, Config, ConfigNode, Node, NodeName, Property};
use rp_schema::Schema;
use std::{collections::HashMap, fs::OpenOptions, path::PathBuf, rc::Rc};

#[derive(Debug)]
pub struct ConfigEditor<'a> {
    schema: &'a Schema,
    config: &'a Config,
    node_stack: Vec<Rc<ConfigNode>>,
    pub save_directory: PathBuf,
    pub save_filename: PathBuf,
}

impl<'a> ConfigEditor<'a> {
    pub fn new(config: &'a Config, schema: &'a Schema) -> ConfigEditor<'a> {
        ConfigEditor {
            schema,
            config,
            node_stack: Vec::new(),
            save_directory: PathBuf::from("./save/"),
            save_filename: PathBuf::from("config.save"),
        }
    }

    pub fn get_current_node(&self) -> Option<String> {
        self.node_stack.last().map(|n| n.name())
    }

    pub fn get_current_path(&self) -> Vec<String> {
        self.node_stack.iter().map(|n| n.name()).collect()
    }

    pub fn get_available_nodes(&self) -> Vec<String> {
        match self.node_stack.last() {
            Some(n) => n
                .get_available_node_names()
                .into_iter()
                .map(|name| format!("{}", name))
                .collect(),
            None => self.config.nodes.keys().map(|key| key.to_owned()).collect(),
        }
    }

    pub fn get_available_properties(&self) -> Option<Vec<String>> {
        self.node_stack
            .last()
            .map(|n| n.get_available_property_names())
    }

    pub fn get_available_nodes_and_properties(&self) -> Vec<String> {
        let mut names = self.get_available_nodes();
        if let Some(available_properties) = self.get_available_properties() {
            names.extend(available_properties);
        }
        names
    }

    pub fn edit_node(&mut self, name: &str) -> anyhow::Result<()> {
        let mut matching_name: Option<NodeName> = None;
        for node_name in match self.node_stack.last() {
            Some(n) => n.get_available_node_names(),
            None => self.config.get_available_node_names(),
        } {
            if node_name.matches(&name)? {
                match (matching_name, &node_name) {
                    (Some(NodeName::Literal(_)), NodeName::Literal(_))
                    | (Some(NodeName::Multiple(_)), NodeName::Multiple(_)) => {
                        return Err(
                            error::ConfigEditorError::AmbiguousNodeName(name.to_string()).into(),
                        )
                    }
                    _ => matching_name = Some(node_name),
                }
            }
        }

        if matching_name.is_some() {
            self.node_stack.push(
                match self.node_stack.last() {
                    Some(n) => n.get_node_with_name(&name)?,
                    None => self.config.get_node_with_name(&name),
                }
                .ok_or_else(|| error::ConfigEditorError::NodeNotFound(name.to_string()))?,
            );
            Ok(())
        } else {
            Err(error::ConfigEditorError::NodeNotFound(name.to_string()).into())
        }
    }

    pub fn go_up(&mut self) -> anyhow::Result<()> {
        self.node_stack
            .pop()
            .ok_or_else(|| error::ConfigEditorError::AlreadyAtTop)?;
        Ok(())
    }

    pub fn go_top(&mut self) -> anyhow::Result<()> {
        if self.node_stack.is_empty() {
            Err(error::ConfigEditorError::AlreadyAtTop.into())
        } else {
            self.node_stack.clear();
            Ok(())
        }
    }

    fn get_property(&self, property: &str) -> anyhow::Result<&Property> {
        Ok(self
            .node_stack
            .last()
            .ok_or_else(|| error::ConfigEditorError::PropertyNotFound(property.to_owned()))?
            .get_property(&property)
            .ok_or_else(|| error::ConfigEditorError::PropertyNotFound(property.to_owned()))?)
    }

    pub fn get_property_values(
        &self,
        of_property: Option<String>,
    ) -> Option<HashMap<String, Vec<String>>> {
        self.node_stack
            .last()
            .map(|n| n.get_property_values(of_property))
    }

    pub fn set_property_value(&self, property: &str, value: &str) -> anyhow::Result<()> {
        let property = self.get_property(property)?;
        Ok(property.set(value)?)
    }

    pub fn remove_property_value(&self, property: &str, value: Option<&str>) -> anyhow::Result<()> {
        let property = self.get_property(property)?;

        property.remove(value)?;
        Ok(())
    }

    pub fn remove_node(&self, node: &str) -> anyhow::Result<()> {
        if let Some(current) = self.node_stack.last() {
            current.remove_subnode(node)
        } else {
            Err(rp_common::error::NodeRemovalError {
                node: String::from(node),
            }
            .into())
        }
    }

    pub fn is_clean(&self) -> bool {
        self.config.is_clean()
    }

    pub fn apply_changes(&self) -> anyhow::Result<bool> {
        self.config.apply_changes()
    }

    pub fn discard_changes(&self) {
        self.config.discard_changes();
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.save_directory.join(&self.save_filename))?;
        self.config.save_config(file)
    }

    pub fn load(&self, name: &str) -> anyhow::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .open(self.save_directory.join(name))?;
        self.config.load_config(file)
    }
}

impl<'a> ConfigEditor<'a> {
    pub fn pretty_print_config(&self) {
        self.config.pretty_print();
    }

    pub fn pretty_print_current_node(&self) {
        match self.node_stack.last() {
            Some(n) => n.pretty_print(0),
            None => self.config.pretty_print(),
        }
    }
}
