use crate::error;
use rp_config::{Config, ConfigNode, Node, NodeName, Property};
use rp_schema::Schema;
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct ConfigEditor<'a> {
    schema: &'a Schema,
    config: &'a Config,
    node_stack: Vec<Rc<ConfigNode>>,
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

    pub fn edit_node(&mut self, name: String) -> error::Result<()> {
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
                                return Err(error::ConfigEditorError::AmbiguousNodeName {
                                    name,
                                    source: None,
                                }
                                .into());
                            }
                        }
                        NodeName::Multiple(_t) => {
                            if let NodeName::Multiple(_t) = node_name {
                                return Err(error::ConfigEditorError::AmbiguousNodeName {
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
            Err(error::ConfigEditorError::NodeNotFound {
                node: name,
                source: None,
            }
            .into())
        }
    }

    pub fn go_up(&mut self) -> error::Result<()> {
        self.node_stack.pop().ok_or_else(|| {
            error::Error::from(error::ConfigEditorError::AlreadyAtTop { source: None })
        })?;
        Ok(())
    }

    pub fn go_top(&mut self) -> error::Result<()> {
        if self.node_stack.is_empty() {
            Err(error::ConfigEditorError::AlreadyAtTop { source: None }.into())
        } else {
            self.node_stack.clear();
            Ok(())
        }
    }

    fn get_property(&self, property: &str) -> error::Result<&Property> {
        self.node_stack
            .last()
            .ok_or_else(|| {
                error::Error::from(error::ConfigEditorError::PropertyNotFound {
                    property: property.to_owned(),
                    source: None,
                })
            })?
            .get_property(&property)
            .ok_or_else(|| {
                error::ConfigEditorError::PropertyNotFound {
                    property: property.to_string(),
                    source: None,
                }
                .into()
            })
    }

    pub fn get_property_values(
        &self,
        of_property: Option<String>,
    ) -> Option<HashMap<String, Vec<String>>> {
        self.node_stack
            .last()
            .map(|n| n.get_property_values(of_property))
    }

    pub fn set_property_value(&self, property: &str, value: &str) -> error::Result<()> {
        let property = self.get_property(property)?;

        property.set(value, self.schema).map_err(|e| {
            error::ConfigEditorError::ValueError {
                source: Some(Box::new(e)),
            }
            .into()
        })
    }

    pub fn remove_property_value(&self, property: &str, value: Option<&str>) -> error::Result<()> {
        let property = self.get_property(property)?;

        property.remove(value).map_err(|e| {
            error::ConfigEditorError::ValueError {
                source: Some(Box::new(e)),
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
