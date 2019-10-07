mod node;
mod property;
mod query;
mod template;
mod validate;
mod value;

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
pub use node::{
    MultiSchemaNode, MultiSchemaNodeSource, NodeLocator, SchemaNode, SchemaNodeTrait,
    SingleSchemaNode,
};
pub use property::Property;
pub use query::Query;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::BufReader,
    rc::Rc,
};
pub use template::Template;
pub use validate::{Validate, ValidationError};
pub use value::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub templates: HashMap<String, Rc<Template>>,
    pub nodes: HashMap<String, SchemaNode>,
    #[serde(default)]
    regex_cache: HashMap<String, Vec<u8>>,
}

pub trait Matches {
    fn matches(&self, value: &str) -> bool;
}

impl Schema {
    pub fn to_binary_file(&self, file: &mut File) -> Result<(), serde_json::error::Error> {
        let encoder = ZlibEncoder::new(file, Compression::best());
        serde_json::to_writer(encoder, &self)
    }

    pub fn from_yaml_file(file: &File) -> Result<Schema, Box<dyn Error>> {
        let reader = BufReader::new(file);
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn from_binary(binary: &[u8]) -> Result<Schema, Box<dyn Error>> {
        let decoder = ZlibDecoder::new(binary);
        let mut schema: Schema = serde_json::from_reader(decoder)?;
        schema.load_regexes_from_cache()?;
        schema.populate_node_metadata();
        Ok(schema)
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

impl Schema {
    pub fn build_regex_cache(&mut self) -> Result<(), Box<dyn Error>> {
        self.regex_cache = HashMap::new();

        for (name, template) in &self.templates {
            if let Some(bytes) = template.serialise_regex()? {
                self.regex_cache.insert(name.clone(), bytes);
            }
        }

        Ok(())
    }

    fn populate_node_metadata(&mut self) {
        let root_locator = Rc::new(NodeLocator::new(String::from("schema"), None));
        for (name, node) in self.nodes.iter_mut() {
            node.update_locators(name.to_owned(), Rc::clone(&root_locator));
        }
    }

    fn load_regexes_from_cache(&self) -> Result<(), Box<dyn Error>> {
        for (name, template) in &self.templates {
            template.load_regex_from_cache(self.regex_cache.get(name));
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

    pub fn find_node(&self, locator: &NodeLocator) -> Option<&SchemaNode> {
        unimplemented!();
    }
}
