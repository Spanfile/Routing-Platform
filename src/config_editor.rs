use crate::{error, error::ErrorTrait};
use common::{
    config::{Config, ConfigNode, Node, NodeName, Property},
    schema::Schema,
};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct ConfigEditor<'a> {
    schema: &'a Schema,
    config: &'a Config,
    node_stack: Vec<Rc<ConfigNode>>,
}

#[derive(Debug)]
pub enum ConfigEditorError {
    NodeNotFound {
        node: String,
        source: Option<Box<error::Error>>,
    },
    PropertyNotFound {
        property: String,
        source: Option<Box<error::Error>>,
    },
    NoParentNode {
        source: Option<Box<error::Error>>,
    },
    ValueError {
        source: Option<Box<error::Error>>,
    },
    AmbiguousNodeName {
        name: String,
        source: Option<Box<error::Error>>,
    },
}

impl ErrorTrait for ConfigEditorError {
    fn display(&self) -> String {
        match self {
            ConfigEditorError::NodeNotFound { node, .. } => format!("node '{}' not found", node),
            ConfigEditorError::PropertyNotFound { property, .. } => {
                format!("property '{}' not found", property)
            }
            ConfigEditorError::NoParentNode { .. } => String::from("no parent node"),
            ConfigEditorError::ValueError { .. } => String::from("invalid value"),
            ConfigEditorError::AmbiguousNodeName { name, .. } => format!(
                "ambiguous node name '{}' (multiple literal node names)",
                name
            ),
        }
    }

    fn source(&self) -> Option<&error::Error> {
        match &self {
            ConfigEditorError::NodeNotFound { source, .. } => source.as_deref(),
            ConfigEditorError::PropertyNotFound { source, .. } => source.as_deref(),
            ConfigEditorError::NoParentNode { source, .. } => source.as_deref(),
            ConfigEditorError::ValueError { source } => source.as_deref(),
            ConfigEditorError::AmbiguousNodeName { source, .. } => source.as_deref(),
        }
    }
}

impl<'a> ConfigEditor<'a> {
    pub fn new(config: &'a Config, schema: &'a Schema) -> ConfigEditor<'a> {
        ConfigEditor {
            schema,
            config,
            node_stack: Vec::new(),
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

    pub fn edit_node(&mut self, name: String) -> error::CustomResult<()> {
        // match match self.node_stack.last() {
        //     Some(n) => &n.subnodes,
        //     None => &self.config.nodes,
        // }
        // .get(&name)
        // {
        //     Some(node) => {
        //         self.node_stack.push(node);
        //         Ok(())
        //     }
        //     None => Err(ConfigEditorError::NodeNotFound(name)),
        // }

        let mut matching_name: Option<NodeName> = None;
        for node_name in match self.node_stack.last() {
            Some(n) => n.get_available_node_names(),
            None => self.config.get_available_node_names(),
        } {
            if node_name.matches(&name) {
                match &matching_name {
                    Some(existing_match) => match existing_match {
                        NodeName::Literal(_n) => {
                            if let NodeName::Literal(_n) = node_name {
                                return Err(ConfigEditorError::AmbiguousNodeName {
                                    name,
                                    source: None,
                                }
                                .into());
                            }
                        }
                        NodeName::Multiple(_t) => {
                            if let NodeName::Multiple(_t) = node_name {
                                return Err(ConfigEditorError::AmbiguousNodeName {
                                    name,
                                    source: None,
                                }
                                .into());
                            }
                        }
                    },
                    _ => matching_name = Some(node_name),
                }
            }
        }

        if matching_name.is_some() {
            self.node_stack.push(match self.node_stack.last() {
                Some(n) => n.get_node_with_name(&name),
                None => self.config.get_node_with_name(&name),
            });
            Ok(())
        } else {
            Err(ConfigEditorError::NodeNotFound {
                node: name,
                source: None,
            }
            .into())
        }
    }

    pub fn go_up(&mut self) -> error::CustomResult<()> {
        self.node_stack
            .pop()
            .ok_or(error::Error::from(ConfigEditorError::NoParentNode {
                source: None,
            }))?;
        Ok(())
    }

    fn get_property(&self, property: String) -> error::CustomResult<&Property> {
        self.node_stack
            .last()
            .ok_or(error::Error::from(ConfigEditorError::PropertyNotFound {
                property: property.to_owned(),
                source: None,
            }))?
            .get_property(&property)
            .ok_or(
                ConfigEditorError::PropertyNotFound {
                    property,
                    source: None,
                }
                .into(),
            )
    }

    pub fn get_property_values(
        &self,
        of_property: Option<String>,
    ) -> Option<HashMap<String, Vec<String>>> {
        self.node_stack
            .last()
            .map(|n| n.get_property_values(of_property))
    }

    pub fn set_property_value(&self, property: String, value: String) -> error::CustomResult<()> {
        let property = self.get_property(property)?;

        property.set(value, self.schema).map_err(|e| {
            ConfigEditorError::ValueError {
                source: Some(Box::new(e.into())),
            }
            .into()
        })
    }

    pub fn remove_property_value(
        &self,
        property: String,
        value: Option<String>,
    ) -> error::CustomResult<()> {
        let property = self.get_property(property)?;

        property.remove(value).map_err(|e| {
            ConfigEditorError::ValueError {
                source: Some(Box::new(e.into())),
            }
            .into()
        })
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
