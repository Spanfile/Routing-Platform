mod node;
mod property;
mod query;
mod template;
mod value;

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
pub use node::{Multinode, Node, NodeLocator};
pub use property::Property;
pub use query::Query;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
pub use template::Template;
pub use value::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema<'schema> {
    pub templates: HashMap<String, Template>,
    pub nodes: HashMap<String, Node<'schema>>,
    #[serde(default)]
    regex_cache: HashMap<String, Vec<u8>>,
}

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
}

trait Validate {
    fn validate(&self, schema: &Schema) -> Vec<ValidationError>;
}

pub trait Matches {
    fn matches(&self, value: &str) -> bool;
}

impl ValidationError {
    pub fn new(message: String) -> ValidationError {
        ValidationError { message }
    }
}

impl Schema<'_> {
    pub fn to_binary_file(&self, file: &mut File) -> Result<(), serde_cbor::error::Error> {
        let mut encoder = ZlibEncoder::new(file, Compression::best());
        serde_cbor::to_writer(&mut encoder, &self)
    }

    pub fn from_yaml_file(file: &File) -> Result<Schema, Box<Error>> {
        let reader = BufReader::new(file);
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn from_binary(binary: &[u8]) -> Result<Schema, Box<Error>> {
        let decoder = ZlibDecoder::new(binary);
        Ok(serde_cbor::from_reader(decoder)?)
    }
}

impl Schema<'_> {
    pub fn validate(&mut self) -> Vec<ValidationError> {
        let mut errors = self.validate_templates();
        errors.extend(self.validate_nodes());
        errors
    }

    fn validate_templates(&mut self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();
        for template in self.templates.values() {
            errors.extend(template.validate(&self));
        }
        errors
    }

    fn validate_nodes(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();
        let mut node_names = HashSet::new();

        for (name, node) in &self.nodes {
            if !node_names.insert(name) {
                errors.push(ValidationError::new(format!(
                    "Node validation error\nName: {}\nDuplicate node name",
                    &name
                )));
            }
            errors.extend(node.validate(self));
        }

        errors
    }
}

impl Schema<'_> {
    pub fn build_regex_cache(&mut self) -> Result<(), Box<dyn Error>> {
        self.regex_cache = HashMap::new();

        for (name, template) in &self.templates {
            if let Some(bytes) = template.serialise_regex()? {
                self.regex_cache.insert(name.clone(), bytes);
            }
        }

        Ok(())
    }

    pub fn load_regexes_from_cache(&self) -> Result<(), Box<dyn Error>> {
        for (name, template) in &self.templates {
            template.load_regex_from_cache(self.regex_cache.get(name));
        }

        Ok(())
    }
}

impl Schema<'_> {
    pub fn print_debug_info(&self) {
        println!(
            "Schema {{\n\tTemplates: {}\n\tNodes: {}\n\tProperties: {}\n\tRegex cache DFA size: {}\n}}",
            self.templates.len(),
            self.node_count(),
            self.property_count(),
            self.regex_cache_dfa_size(),
        );
    }

    fn regex_cache_dfa_size(&self) -> usize {
        let mut sum = 0;
        for template in self.templates.values() {
            sum += template.compiled_regex_size();
        }
        sum
    }

    fn node_count(&self) -> usize {
        let mut sum = 0;
        for node in self.nodes.values() {
            sum += node.node_count();
        }
        sum
    }

    fn property_count(&self) -> usize {
        let mut sum = 0;
        for node in self.nodes.values() {
            sum += node.property_count();
        }
        sum
    }
}
