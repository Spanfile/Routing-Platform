mod node;
mod property;
mod query;
mod template;
mod value;

use node::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use template::Template;

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub templates: Vec<Template>,
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
}

impl ValidationError {
    pub fn new(message: String) -> ValidationError {
        ValidationError { message }
    }
}

impl Schema {
    pub fn to_binary_file(self, file: &mut File) -> Result<(), serde_cbor::error::Error> {
        serde_cbor::to_writer(file, &self)
    }

    pub fn from_yaml_file(file: &File) -> Result<Schema, Box<Error>> {
        let reader = BufReader::new(file);
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn from_binary_file(file: &File) -> Result<Schema, Box<Error>> {
        let reader = BufReader::new(file);
        Ok(serde_cbor::from_reader(reader)?)
    }
}

impl Schema {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = self.validate_templates();
        errors.extend(self.validate_nodes());
        errors
    }

    fn validate_templates(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();
        for template in &self.templates {
            match template.validate() {
                Ok(()) => (),
                Err(e) => errors.push(e),
            }
        }
        errors
    }

    fn validate_nodes(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();
        let mut node_names = HashSet::new();
        for node in &self.nodes {
            if !node_names.insert(&node.name) {
                errors.push(ValidationError::new(format!(
                    "Node validation error\nName: {}\nDuplicate node name",
                    node.name
                )));
            }
            errors.extend(node.validate(self));
        }
        errors
    }
}
