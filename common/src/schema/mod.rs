pub mod node;
pub mod property;
pub mod query;
pub mod template;
pub mod value;

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use node::Node;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use template::Template;

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub templates: Vec<Template>,
    pub nodes: Vec<Node>,
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

impl ValidationError {
    pub fn new(message: String) -> ValidationError {
        ValidationError { message }
    }
}

impl Schema {
    pub fn to_binary_file(self, file: &mut File) -> Result<(), serde_cbor::error::Error> {
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

impl Schema {
    pub fn validate(&mut self) -> Vec<ValidationError> {
        let mut errors = self.validate_templates();
        errors.extend(self.validate_nodes());
        errors
    }

    fn validate_templates(&mut self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();
        for template in &self.templates {
            errors.extend(template.validate(&self));
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

impl Schema {
    pub fn build_regex_cache(&mut self) -> Result<(), Box<dyn Error>> {
        self.regex_cache = HashMap::new();

        for template in &self.templates {
            let bytes = template.serialise_regex()?;
            self.regex_cache.insert(template.name.clone(), bytes);
        }

        Ok(())
    }

    pub fn load_regexes_from_cache(&self) -> Result<(), Box<dyn Error>> {
        for template in &self.templates {
            match self.regex_cache.get(&template.name) {
                Some(cache) => template.deserialise_regex(cache),
                None => {
                    println!(
                        "missing cached regex for template '{}', recompiling",
                        template.name
                    );
                    template.compile_regex();
                }
            }
        }

        Ok(())
    }
}

impl Schema {
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
        for template in &self.templates {
            sum += template.compiled_regex_size();
        }
        sum
    }

    fn node_count(&self) -> usize {
        let mut sum = 0;
        for node in &self.nodes {
            sum += node.node_count();
        }
        sum
    }

    fn property_count(&self) -> usize {
        let mut sum = 0;
        for node in &self.nodes {
            sum += node.property_count();
        }
        sum
    }
}
